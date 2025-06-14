use crate::database::Database;
use crate::models::*;
use std::sync::Mutex;
use tauri::State;

// Project commands
#[tauri::command]
pub fn create_project(
    db: State<Mutex<Database>>,
    project_data: CreateProject,
) -> Result<Project, String> {
    let db = db.lock().map_err(|_| "Failed to lock database")?;
    db.create_project(project_data).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_projects(db: State<Mutex<Database>>) -> Result<Vec<Project>, String> {
    let db = db.lock().map_err(|_| "Failed to lock database")?;
    db.get_projects().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_project(
    db: State<Mutex<Database>>,
    project_data: UpdateProject,
) -> Result<Project, String> {
    let db = db.lock().map_err(|_| "Failed to lock database")?;
    db.update_project(project_data).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_project(db: State<Mutex<Database>>, id: String) -> Result<(), String> {
    let db = db.lock().map_err(|_| "Failed to lock database")?;
    db.delete_project(&id).map_err(|e| e.to_string())
}

// Task commands
#[tauri::command]
pub fn create_task(db: State<Mutex<Database>>, task_data: CreateTask) -> Result<Task, String> {
    let db = db.lock().map_err(|_| "Failed to lock database")?;
    db.create_task(task_data).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_tasks(db: State<Mutex<Database>>) -> Result<Vec<Task>, String> {
    let db = db.lock().map_err(|_| "Failed to lock database")?;
    db.get_tasks().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_tasks_by_project(
    db: State<Mutex<Database>>,
    project_id: String,
) -> Result<Vec<Task>, String> {
    let db = db.lock().map_err(|_| "Failed to lock database")?;
    db.get_tasks_by_project(&project_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_task(db: State<Mutex<Database>>, task_data: UpdateTask) -> Result<Task, String> {
    let db = db.lock().map_err(|_| "Failed to lock database")?;
    db.update_task(task_data).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_task(db: State<Mutex<Database>>, id: String) -> Result<(), String> {
    let db = db.lock().map_err(|_| "Failed to lock database")?;
    db.delete_task(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_today_tasks(db: State<Mutex<Database>>) -> Result<Vec<Task>, String> {
    let db = db.lock().map_err(|_| "Failed to lock database")?;
    db.get_today_tasks().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_upcoming_tasks(db: State<Mutex<Database>>) -> Result<Vec<Task>, String> {
    let db = db.lock().map_err(|_| "Failed to lock database")?;
    db.get_upcoming_tasks().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn mark_task_complete(
    db: State<Mutex<Database>>,
    id: String,
    completed: bool,
) -> Result<Task, String> {
    let db = db.lock().map_err(|_| "Failed to lock database")?;
    let update_data = UpdateTask {
        id,
        completed: Some(completed),
        title: None,
        description: None,
        project_id: None,
        status: None,
        priority: None,
        due_date: None,
    };
    db.update_task(update_data).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_task_stats(db: State<Mutex<Database>>) -> Result<TaskStats, String> {
    let db = db.lock().map_err(|_| "Failed to lock database")?;
    db.get_task_stats().map_err(|e| e.to_string())
}
