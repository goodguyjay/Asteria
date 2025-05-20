use crate::{asn_debug, asn_err, asn_info};
use once_cell::sync::Lazy;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TaskPriority {
    Low,
    Normal,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

pub struct TaskQueues {
    pub critical: VecDeque<Task>,
    pub high: VecDeque<Task>,
    pub normal: VecDeque<Task>,
    pub low: VecDeque<Task>,
}

impl TaskQueues {
    pub fn new() -> Self {
        TaskQueues {
            critical: VecDeque::new(),
            high: VecDeque::new(),
            normal: VecDeque::new(),
            low: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, task: Task) {
        match task.priority {
            TaskPriority::Critical => self.critical.push_back(task),
            TaskPriority::High => self.high.push_back(task),
            TaskPriority::Normal => self.normal.push_back(task),
            TaskPriority::Low => self.low.push_back(task),
        }
    }

    pub fn total_len(&self) -> usize {
        self.critical.len() + self.high.len() + self.normal.len() + self.low.len()
    }

    pub fn all_tasks(&self) -> Vec<&Task> {
        self.critical
            .iter()
            .chain(self.high.iter())
            .chain(self.normal.iter())
            .chain(self.low.iter())
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: u64,
    pub commands: Vec<String>,
    pub priority: TaskPriority,
    pub status: TaskStatus,
    pub deadline: Option<u64>,
    pub retries: u8,
}

impl Task {
    pub fn new(id: u64, commands: Vec<String>, priority: TaskPriority) -> Self {
        Task {
            id,
            commands,
            priority,
            status: TaskStatus::Pending,
            deadline: None,
            retries: 0,
        }
    }
}

pub static TASK_QUEUES: Lazy<Arc<Mutex<TaskQueues>>> =
    Lazy::new(|| Arc::new(Mutex::new(TaskQueues::new())));

pub static TASK_STATUS_MAP: Lazy<Arc<Mutex<HashMap<u64, TaskStatus>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

pub fn init() {
    asn_info!("Initializing Task Queue...");

    let task_queue = Arc::clone(&TASK_QUEUES);
    std::thread::spawn(move || task_runner(task_queue));
}

fn task_runner(queues: Arc<Mutex<TaskQueues>>) {
    loop {
        let maybe_task = {
            let mut qs = queues.lock().unwrap();
            next_task_to_run(&mut *qs)
        };

        if let Some(mut task) = maybe_task {
            {
                let mut status_map = TASK_STATUS_MAP.lock().unwrap();
                status_map.insert(task.id, TaskStatus::Running);
            }

            task.status = TaskStatus::Running;

            asn_info!(
                "Executing task '{}' with {} subcommands(s)... | Priority: {:?} | Status: {:?}",
                task.id,
                task.commands.join("; "),
                task.commands.len(),
                TaskStatus::Running,
            );

            for (i, cmd) in task.commands.iter().enumerate() {
                asn_info!("  ↳ [{}] {}", i + 1, cmd);
                std::thread::sleep(std::time::Duration::from_secs(8));
            }

            let success = simulate_task_execution(&task);

            if success {
                asn_info!("Task {} completed successfully.", task.id);
                task.status = TaskStatus::Completed;
            } else {
                task.retries += 1;

                if task.retries < 5 {
                    asn_err!(
                        "Task {} failed (attempt {}/5). Retrying...",
                        task.id,
                        task.retries
                    );

                    {
                        let mut qs = queues.lock().unwrap();
                        qs.enqueue(task.clone());
                    }

                    {
                        let mut status_map = TASK_STATUS_MAP.lock().unwrap();
                        status_map.insert(task.id, TaskStatus::Pending);
                    }

                    continue;
                } else {
                    asn_err!(
                        "Task {} failed after 5 retries - marking as failed.",
                        task.id
                    );
                    task.status = TaskStatus::Failed;
                }
            }

            {
                let mut status_map = TASK_STATUS_MAP.lock().unwrap();
                status_map.insert(task.id, task.status);
            }

            asn_debug!(
                "De-queuing task: '{}' | Priority: {:?} | Remaining queue size: {} | Status: {:?}",
                task.commands.join("; "),
                task.priority,
                queues.lock().unwrap().total_len(),
                task.status
            );
        } else {
            // No tasks in the queue, briefly sleep to avoid CPU burn
            std::thread::sleep(std::time::Duration::from_millis(500));
            asn_debug!("No tasks in queue. Waiting...");
        }

        asn_debug!(
            "Current task status map: {:?}",
            *TASK_STATUS_MAP.lock().unwrap()
        );
    }
}

fn next_task_to_run(queues: &mut TaskQueues) -> Option<Task> {
    // edf
    if !queues.critical.is_empty() {
        let idx = queues
            .critical
            .iter()
            .enumerate()
            .min_by_key(|(_i, task)| task.deadline.unwrap_or(u64::MAX))
            .map(|(i, _)| i)
            .unwrap();

        return Some(queues.critical.remove(idx).unwrap());
    }

    if let Some(task) = queues.high.pop_front() {
        return Some(task);
    }

    if let Some(task) = queues.normal.pop_front() {
        return Some(task);
    }

    if let Some(task) = queues.low.pop_front() {
        return Some(task);
    }

    None
}

fn simulate_task_execution(task: &Task) -> bool {
    use rand::Rng;
    let mut rng = rand::rng();

    rng.random_range(0..10) < 7 // 70% chance of success yeesh
}
