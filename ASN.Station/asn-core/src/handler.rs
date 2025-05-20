use crate::queue::{TASK_QUEUES, TASK_STATUS_MAP};
use crate::task_manager::{build_tasks, parse_commands, queue_tasks};
use crate::{asn_err, asn_info};
use std::io::{Read, Write};
use std::net::TcpStream;

pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer) {
        Ok(size) => {
            let command = String::from_utf8_lossy(&buffer[..size]);
            asn_info!("Received command: {}", command);

            if command.to_uppercase() == "GET_ALL" {
                let status_map = TASK_STATUS_MAP.lock().unwrap();
                let qs = TASK_QUEUES.lock().unwrap();

                let mut response = String::new();

                for task in qs.all_tasks() {
                    let status = status_map.get(&task.id).unwrap_or(&task.status);
                    response.push_str(&format!(
                        "[{}] {:?} | {:?} | {} \n",
                        task.id,
                        task.priority,
                        status,
                        task.commands.join("; ")
                    ));
                }

                stream.write_all(response.as_bytes()).unwrap();
                return;
            }

            let parsed = parse_commands(&command);
            let tasks = build_tasks(parsed);
            queue_tasks(tasks);

            let response = format!("Command '{}' executed.", command);
            stream.write_all(response.as_bytes()).unwrap();
        }

        Err(e) => asn_err!("Failed to read from stream: {}", e),
    }
}
