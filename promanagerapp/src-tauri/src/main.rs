#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod database;
mod models;
mod commands;

use database::Database;
use std::sync::Mutex;

fn main() {
  // Initialize database
  let db = Database::new().expect("Failed to initialize database");
  
  tauri::Builder::default()
    .manage(Mutex::new(db))
    .invoke_handler(tauri::generate_handler![
      commands::create_project,
      commands::get_projects,
      commands::update_project,
      commands::delete_project,
      commands::create_task,
      commands::get_tasks,
      commands::get_tasks_by_project,
      commands::update_task,
      commands::delete_task,
      commands::get_today_tasks,
      commands::get_upcoming_tasks,
      commands::mark_task_complete,
      commands::get_task_stats
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
