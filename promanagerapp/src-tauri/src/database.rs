use rusqlite::{Connection, Result};
use crate::models::*;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("promanager.db")?;
        let db = Database { conn };
        db.create_tables()?;
        Ok(db)
    }

    fn create_tables(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS projects (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                color TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT,
                project_id TEXT,
                status TEXT NOT NULL,
                priority TEXT NOT NULL,
                due_date TEXT,
                completed BOOLEAN NOT NULL DEFAULT 0,
                completed_at TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY(project_id) REFERENCES projects(id)
            )",
            [],
        )?;

        Ok(())
    }

    // Project operations
    pub fn create_project(&self, project_data: CreateProject) -> Result<Project> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        
        let project = Project {
            id: id.clone(),
            name: project_data.name,
            description: project_data.description,
            color: project_data.color,
            created_at: now,
            updated_at: now,
        };

        self.conn.execute(
            "INSERT INTO projects (id, name, description, color, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (
                &project.id,
                &project.name,
                &project.description,
                &project.color,
                project.created_at.to_rfc3339(),
                project.updated_at.to_rfc3339(),
            ),
        )?;

        Ok(project)
    }

    pub fn get_projects(&self) -> Result<Vec<Project>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, description, color, created_at, updated_at FROM projects ORDER BY created_at DESC"
        )?;

        let project_iter = stmt.query_map([], |row| {
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                color: row.get(3)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap()
                    .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .unwrap()
                    .with_timezone(&Utc),
            })
        })?;

        let mut projects = Vec::new();
        for project in project_iter {
            projects.push(project?);
        }

        Ok(projects)
    }

    pub fn update_project(&self, project_data: UpdateProject) -> Result<Project> {
        let now = Utc::now();

        // First get the current project
        let mut stmt = self.conn.prepare(
            "SELECT id, name, description, color, created_at, updated_at FROM projects WHERE id = ?1"
        )?;

        let mut project = stmt.query_row([&project_data.id], |row| {
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                color: row.get(3)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap()
                    .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .unwrap()
                    .with_timezone(&Utc),
            })
        })?;

        // Update fields if provided
        if let Some(name) = project_data.name {
            project.name = name;
        }
        if let Some(description) = project_data.description {
            project.description = Some(description);
        }
        if let Some(color) = project_data.color {
            project.color = color;
        }
        project.updated_at = now;

        // Update in database
        self.conn.execute(
            "UPDATE projects SET name = ?1, description = ?2, color = ?3, updated_at = ?4 WHERE id = ?5",
            (
                &project.name,
                &project.description,
                &project.color,
                project.updated_at.to_rfc3339(),
                &project.id,
            ),
        )?;

        Ok(project)
    }

    pub fn delete_project(&self, id: &str) -> Result<()> {
        // First delete all tasks associated with this project
        self.conn.execute("DELETE FROM tasks WHERE project_id = ?1", [id])?;
        
        // Then delete the project
        self.conn.execute("DELETE FROM projects WHERE id = ?1", [id])?;
        
        Ok(())
    }

    // Task operations
    pub fn create_task(&self, task_data: CreateTask) -> Result<Task> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        
        let task = Task {
            id: id.clone(),
            title: task_data.title,
            description: task_data.description,
            project_id: task_data.project_id,
            status: task_data.status,
            priority: task_data.priority,
            due_date: task_data.due_date,
            completed: false,
            completed_at: None,
            created_at: now,
            updated_at: now,
        };

        self.conn.execute(
            "INSERT INTO tasks (id, title, description, project_id, status, priority, due_date, completed, completed_at, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            (
                &task.id,
                &task.title,
                &task.description,
                &task.project_id,
                task.status.to_string(),
                task.priority.to_string(),
                task.due_date.map(|d| d.to_rfc3339()),
                task.completed,
                task.completed_at.map(|d| d.to_rfc3339()),
                task.created_at.to_rfc3339(),
                task.updated_at.to_rfc3339(),
            ),
        )?;

        Ok(task)
    }

    pub fn get_tasks(&self) -> Result<Vec<Task>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, description, project_id, status, priority, due_date, completed, completed_at, created_at, updated_at 
             FROM tasks ORDER BY created_at DESC"
        )?;

        let task_iter = stmt.query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                project_id: row.get(3)?,
                status: TaskStatus::from(row.get::<_, String>(4)?),
                priority: TaskPriority::from(row.get::<_, String>(5)?),
                due_date: row.get::<_, Option<String>>(6)?
                    .map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc)),
                completed: row.get(7)?,
                completed_at: row.get::<_, Option<String>>(8)?
                    .map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc)),
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(9)?)
                    .unwrap()
                    .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(10)?)
                    .unwrap()
                    .with_timezone(&Utc),
            })
        })?;

        let mut tasks = Vec::new();
        for task in task_iter {
            tasks.push(task?);
        }

        Ok(tasks)
    }

    pub fn get_tasks_by_project(&self, project_id: &str) -> Result<Vec<Task>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, description, project_id, status, priority, due_date, completed, completed_at, created_at, updated_at 
             FROM tasks WHERE project_id = ?1 ORDER BY created_at DESC"
        )?;

        let task_iter = stmt.query_map([project_id], |row| {
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                project_id: row.get(3)?,
                status: TaskStatus::from(row.get::<_, String>(4)?),
                priority: TaskPriority::from(row.get::<_, String>(5)?),
                due_date: row.get::<_, Option<String>>(6)?
                    .map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc)),
                completed: row.get(7)?,
                completed_at: row.get::<_, Option<String>>(8)?
                    .map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc)),
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(9)?)
                    .unwrap()
                    .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(10)?)
                    .unwrap()
                    .with_timezone(&Utc),
            })
        })?;

        let mut tasks = Vec::new();
        for task in task_iter {
            tasks.push(task?);
        }

        Ok(tasks)
    }

    pub fn update_task(&self, task_data: UpdateTask) -> Result<Task> {
        let now = Utc::now();

        // First get the current task
        let mut stmt = self.conn.prepare(
            "SELECT id, title, description, project_id, status, priority, due_date, completed, completed_at, created_at, updated_at 
             FROM tasks WHERE id = ?1"
        )?;

        let mut task = stmt.query_row([&task_data.id], |row| {
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                project_id: row.get(3)?,
                status: TaskStatus::from(row.get::<_, String>(4)?),
                priority: TaskPriority::from(row.get::<_, String>(5)?),
                due_date: row.get::<_, Option<String>>(6)?
                    .map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc)),
                completed: row.get(7)?,
                completed_at: row.get::<_, Option<String>>(8)?
                    .map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc)),
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(9)?)
                    .unwrap()
                    .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(10)?)
                    .unwrap()
                    .with_timezone(&Utc),
            })
        })?;

        // Update fields if provided
        if let Some(title) = task_data.title {
            task.title = title;
        }
        if let Some(description) = task_data.description {
            task.description = Some(description);
        }
        if let Some(project_id) = task_data.project_id {
            task.project_id = Some(project_id);
        }
        if let Some(status) = task_data.status {
            task.status = status;
        }
        if let Some(priority) = task_data.priority {
            task.priority = priority;
        }
        if let Some(due_date) = task_data.due_date {
            task.due_date = Some(due_date);
        }
        if let Some(completed) = task_data.completed {
            task.completed = completed;
            if completed && task.completed_at.is_none() {
                task.completed_at = Some(now);
            } else if !completed {
                task.completed_at = None;
            }
        }
        task.updated_at = now;

        // Update in database
        self.conn.execute(
            "UPDATE tasks SET title = ?1, description = ?2, project_id = ?3, status = ?4, priority = ?5, due_date = ?6, completed = ?7, completed_at = ?8, updated_at = ?9 WHERE id = ?10",
            (
                &task.title,
                &task.description,
                &task.project_id,
                task.status.to_string(),
                task.priority.to_string(),
                task.due_date.map(|d| d.to_rfc3339()),
                task.completed,
                task.completed_at.map(|d| d.to_rfc3339()),
                task.updated_at.to_rfc3339(),
                &task.id,
            ),
        )?;

        Ok(task)
    }

    pub fn delete_task(&self, id: &str) -> Result<()> {
        self.conn.execute("DELETE FROM tasks WHERE id = ?1", [id])?;
        Ok(())
    }

    pub fn get_today_tasks(&self) -> Result<Vec<Task>> {
        let today = Utc::now().date_naive();
        let today_start = today.and_hms_opt(0, 0, 0).unwrap().and_utc().to_rfc3339();
        let today_end = today.and_hms_opt(23, 59, 59).unwrap().and_utc().to_rfc3339();

        let mut stmt = self.conn.prepare(
            "SELECT id, title, description, project_id, status, priority, due_date, completed, completed_at, created_at, updated_at 
             FROM tasks WHERE due_date BETWEEN ?1 AND ?2 ORDER BY due_date ASC"
        )?;

        let task_iter = stmt.query_map([today_start, today_end], |row| {
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                project_id: row.get(3)?,
                status: TaskStatus::from(row.get::<_, String>(4)?),
                priority: TaskPriority::from(row.get::<_, String>(5)?),
                due_date: row.get::<_, Option<String>>(6)?
                    .map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc)),
                completed: row.get(7)?,
                completed_at: row.get::<_, Option<String>>(8)?
                    .map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc)),
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(9)?)
                    .unwrap()
                    .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(10)?)
                    .unwrap()
                    .with_timezone(&Utc),
            })
        })?;

        let mut tasks = Vec::new();
        for task in task_iter {
            tasks.push(task?);
        }

        Ok(tasks)
    }

    pub fn get_upcoming_tasks(&self) -> Result<Vec<Task>> {
        let tomorrow = Utc::now().date_naive() + chrono::Duration::days(1);
        let next_week = tomorrow + chrono::Duration::days(7);
        let tomorrow_start = tomorrow.and_hms_opt(0, 0, 0).unwrap().and_utc().to_rfc3339();
        let next_week_end = next_week.and_hms_opt(23, 59, 59).unwrap().and_utc().to_rfc3339();

        let mut stmt = self.conn.prepare(
            "SELECT id, title, description, project_id, status, priority, due_date, completed, completed_at, created_at, updated_at 
             FROM tasks WHERE due_date BETWEEN ?1 AND ?2 ORDER BY due_date ASC"
        )?;

        let task_iter = stmt.query_map([tomorrow_start, next_week_end], |row| {
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                project_id: row.get(3)?,
                status: TaskStatus::from(row.get::<_, String>(4)?),
                priority: TaskPriority::from(row.get::<_, String>(5)?),
                due_date: row.get::<_, Option<String>>(6)?
                    .map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc)),
                completed: row.get(7)?,
                completed_at: row.get::<_, Option<String>>(8)?
                    .map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc)),
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(9)?)
                    .unwrap()
                    .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(10)?)
                    .unwrap()
                    .with_timezone(&Utc),
            })
        })?;

        let mut tasks = Vec::new();
        for task in task_iter {
            tasks.push(task?);
        }

        Ok(tasks)
    }

    pub fn get_task_stats(&self) -> Result<TaskStats> {
        let mut stmt = self.conn.prepare("SELECT COUNT(*) FROM tasks")?;
        let total_tasks: i32 = stmt.query_row([], |row| row.get(0))?;

        let mut stmt = self.conn.prepare("SELECT COUNT(*) FROM tasks WHERE completed = 1")?;
        let completed_tasks: i32 = stmt.query_row([], |row| row.get(0))?;

        let pending_tasks = total_tasks - completed_tasks;

        let today = Utc::now().date_naive();
        let today_start = today.and_hms_opt(0, 0, 0).unwrap().and_utc().to_rfc3339();
        let today_end = today.and_hms_opt(23, 59, 59).unwrap().and_utc().to_rfc3339();

        let mut stmt = self.conn.prepare("SELECT COUNT(*) FROM tasks WHERE due_date BETWEEN ?1 AND ?2")?;
        let today_tasks: i32 = stmt.query_row([today_start, today_end], |row| row.get(0))?;

        let now = Utc::now().to_rfc3339();
        let mut stmt = self.conn.prepare("SELECT COUNT(*) FROM tasks WHERE due_date < ?1 AND completed = 0")?;
        let overdue_tasks: i32 = stmt.query_row([now], |row| row.get(0))?;

        Ok(TaskStats {
            total_tasks,
            completed_tasks,
            pending_tasks,
            today_tasks,
            overdue_tasks,
        })
    }
}
