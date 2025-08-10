// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use rusqlite::{params, Connection, Result};
use std::path::PathBuf;

use serde::Serialize;

#[derive(Serialize)]
#[derive(Debug)]
struct Task {
    id: i32,
    name: String,
    description: String,
    status: String,
}

#[tauri::command]
fn create_task(name: &str, description: &str) -> Result<String, String> {
    println!("create task called: {} - {}", name, description);

    let mut db_path = PathBuf::from("/Users/srikar/Workspace/kaanbaan");
    db_path.push("tasks.db");

    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO tasks (name, description) VALUES (?1, ?2)",
        params![name, description],
    ).map_err(|e| e.to_string())?;

    Ok("Task created successfully".to_string())
}

#[tauri::command]
fn list_tasks() -> Result<Vec<Task>, String> {
    let mut db_path = PathBuf::from("/Users/srikar/Workspace/kaanbaan");
    db_path.push("tasks.db");

    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT * FROM tasks").map_err(|e| e.to_string())?;
    
    let task_iter = stmt.query_map(params![], |row| {
        Ok(Task {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            status: row.get(3)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut tasks = Vec::new();
    for task in task_iter {
        tasks.push(task.map_err(|e| e.to_string())?);
    }
    Ok(tasks)
}

fn setup_database() -> Result<()> {
    let mut db_path = PathBuf::from("/Users/srikar/Workspace/kaanbaan");
    db_path.push("tasks.db");

    let conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            status TEXT DEFAULT 'pending'
        )",
        params![],
    )?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    setup_database().expect("Failed to set up database");
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![create_task, list_tasks])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
