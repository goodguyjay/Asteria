use crate::{asn_debug, asn_info};
use once_cell::sync::Lazy;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TaskPriority {
    Low,
    Normal,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: u64,
    pub commands: Vec<String>,
    pub priority: TaskPriority,
}

impl Task {
    pub fn new(id: u64, commands: Vec<String>, priority: TaskPriority) -> Self {
        Task {
            id,
            commands,
            priority,
        }
    }
}

pub static TASK_QUEUE: Lazy<Arc<Mutex<VecDeque<Task>>>> =
    Lazy::new(|| Arc::new(Mutex::new(VecDeque::new())));

pub fn init() {
    asn_info!("Initializing Task Queue...");

    let task_queue = Arc::clone(&TASK_QUEUE);
    std::thread::spawn(move || task_runner(task_queue));
}

fn task_runner(queue: Arc<Mutex<VecDeque<Task>>>) {
    loop {
        let maybe_task = {
            let mut q = queue.lock().unwrap();
            q.pop_front()
        };

        if let Some(task) = maybe_task {
            asn_info!(
                "Executing task '{}' with {} subcommands(s)...",
                task.id,
                task.commands.join("; ")
            );

            for (i, cmd) in task.commands.iter().enumerate() {
                asn_info!("  ↳ [{}] {}", i + 1, cmd);
                std::thread::sleep(std::time::Duration::from_secs(2));
            }

            // simulating
            asn_info!("Task '{}' completed.", task.commands.join("; "));

            asn_debug!(
                "De-queuing task: '{}' | Priority: {:?} | Remaining queue size: {}",
                task.commands.join("; "),
                task.priority,
                queue.lock().unwrap().len()
            );
        } else {
            // No tasks in the queue, briefly sleep to avoid CPU burn
            std::thread::sleep(std::time::Duration::from_millis(500));
            asn_debug!("No tasks in queue. Waiting...");
        }
    }
}
