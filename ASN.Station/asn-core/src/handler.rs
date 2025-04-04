use crate::queue::{TASK_QUEUE, Task, TaskPriority};
use crate::{asn_debug, asn_err, asn_info};
use std::io::{Read, Write};
use std::net::TcpStream;

pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer) {
        Ok(size) => {
            let command = String::from_utf8_lossy(&buffer[..size]);
            asn_info!("Received command: {}", command);

            let commands: Vec<String> = command
                .split(';')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            let mut tasks: Vec<Task> = Vec::new();
            let mut priorities_found = std::collections::HashSet::new();

            for cmd in commands {
                let priority = classify_priority(&cmd);
                priorities_found.insert(priority.clone());

                let task = Task::new(rand::random(), vec![cmd.clone()], priority.clone());
                tasks.push(task);
            }

            let mut queue = TASK_QUEUE.lock().unwrap();

            if priorities_found.len() > 1 {
                asn_debug!("Mixed priority batch received.");
            }

            tasks.sort_by(|a, b| b.priority.cmp(&a.priority));

            for task in tasks {
                queue.push_back(task.clone());

                asn_debug!(
                    "Task queued: [{}] '{}' | Priority: {:?} | Queue size: {}",
                    task.id,
                    task.commands.join("; "),
                    task.priority,
                    queue.len()
                );
            }

            drop(queue);

            let response = format!("Command '{}' executed.", command);
            stream.write_all(response.as_bytes()).unwrap();
        }

        Err(e) => asn_err!("Failed to read from stream: {}", e),
    }
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
