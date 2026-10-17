use crate::todo::Todo;
use std::fs;

const FILE_PATH &str = "todos.json";
pub fn load_todos() -> Vec<Todo> {
    match fs::read_to_string(FILE_PATH) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| vec![]),
        Err(_) => vec![], // Kalau file belum ada, return array kosong
    }
}

pub fn save_todos(todos: &Vec<Todo>) {
    let json = serde_json::to_string_pretty(todos).expect("Gagal serialize JSON");
    fs::write(FILE_PATH, json).expect("Gagal menyimpan file");
}