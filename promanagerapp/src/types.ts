export interface Project {
  id: string;
  name: string;
  description?: string;
  color: string;
  created_at: string; // ISO date string
  updated_at: string; // ISO date string
}

export interface Task {
  id: string;
  title: string;
  description?: string;
  project_id?: string;
  status: TaskStatus;
  priority: TaskPriority;
  due_date?: string; // ISO date string
  completed: boolean;
  completed_at?: string; // ISO date string
  created_at: string; // ISO date string
  updated_at: string; // ISO date string
}

export type TaskStatus = 'todo' | 'in_progress' | 'done';
export type TaskPriority = 'low' | 'medium' | 'high';

export interface CreateProject {
  name: string;
  description?: string;
  color: string;
}

export interface UpdateProject {
  id: string;
  name?: string;
  description?: string;
  color?: string;
}

export interface CreateTask {
  title: string;
  description?: string;
  project_id?: string;
  status: TaskStatus;
  priority: TaskPriority;
  due_date?: string; // ISO date string
}

export interface UpdateTask {
  id: string;
  title?: string;
  description?: string;
  project_id?: string;
  status?: TaskStatus;
  priority?: TaskPriority;
  due_date?: string; // ISO date string
  completed?: boolean;
}

export interface TaskStats {
  total_tasks: number;
  completed_tasks: number;
  pending_tasks: number;
  today_tasks: number;
  overdue_tasks: number;
}

export interface ProjectWithTaskCount extends Project {
  task_count?: number;
  completed_task_count?: number;
}
