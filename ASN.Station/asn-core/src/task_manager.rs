use crate::queue::{TASK_QUEUES, TASK_STATUS_MAP, Task, TaskPriority, TaskStatus};
use rand::random;

pub fn parse_commands(raw: &str) -> Vec<(String, TaskPriority)> {
    raw.split(';')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .map(|cmd| {
            let priority = classify_priority(&cmd);
            (cmd, priority)
        })
        .collect()
}

fn classify_priority(cmd: &str) -> TaskPriority {
    let cmd = cmd.to_lowercase();

    match cmd.as_str() {
        "low" => TaskPriority::Low,
        "normal" => TaskPriority::Normal,
        "high" => TaskPriority::High,
        "critical" => TaskPriority::Critical,
        _ => {
            if cmd.contains("crit") {
                TaskPriority::Critical
            } else if cmd.contains("high") {
                TaskPriority::High
            } else if cmd.contains("low") {
                TaskPriority::Low
            } else {
                TaskPriority::Normal
            }
        }
    }
}

pub fn build_tasks(parsed: Vec<(String, TaskPriority)>) -> Vec<Task> {
    parsed
        .into_iter()
        .map(|(cmd, priority)| Task::new(random(), vec![cmd], priority))
        .collect()
}

pub fn queue_tasks(tasks: Vec<Task>) {
    let mut queue = TASK_QUEUES.lock().unwrap();
    let mut status_map = TASK_STATUS_MAP.lock().unwrap();

    for task in tasks {
        queue.enqueue(task.clone());
        status_map.insert(task.id, TaskStatus::Pending);
    }
}
