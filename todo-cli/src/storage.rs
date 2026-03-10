use crate::task::Task;
use std::fs;

const FILE_PATH: &str = "tasks.json";

pub fn load_tasks() -> Vec<Task> {
    match fs::read_to_string(FILE_PATH) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => vec![],
    }
}

pub fn save_tasks(tasks: &[Task]) {
    let content = serde_json::to_string_pretty(tasks).expect("Error serializando tareas");
    fs::write(FILE_PATH, content).expect("Error guardando tareas");
}
