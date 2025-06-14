import { invoke } from '@tauri-apps/api/tauri';
import { 
  Project, 
  Task, 
  CreateProject, 
  UpdateProject, 
  CreateTask, 
  UpdateTask, 
  TaskStats 
} from './types';

// Project API
export const projectApi = {
  create: async (projectData: CreateProject): Promise<Project> => {
    return await invoke('create_project', { projectData });
  },

  getAll: async (): Promise<Project[]> => {
    return await invoke('get_projects');
  },

  update: async (projectData: UpdateProject): Promise<Project> => {
    return await invoke('update_project', { projectData });
  },

  delete: async (id: string): Promise<void> => {
    return await invoke('delete_project', { id });
  },
};

// Task API
export const taskApi = {
  create: async (taskData: CreateTask): Promise<Task> => {
    return await invoke('create_task', { taskData });
  },

  getAll: async (): Promise<Task[]> => {
    return await invoke('get_tasks');
  },

  getByProject: async (projectId: string): Promise<Task[]> => {
    return await invoke('get_tasks_by_project', { projectId });
  },

  update: async (taskData: UpdateTask): Promise<Task> => {
    return await invoke('update_task', { taskData });
  },

  delete: async (id: string): Promise<void> => {
    return await invoke('delete_task', { id });
  },

  getToday: async (): Promise<Task[]> => {
    return await invoke('get_today_tasks');
  },

  getUpcoming: async (): Promise<Task[]> => {
    return await invoke('get_upcoming_tasks');
  },

  markComplete: async (id: string, completed: boolean): Promise<Task> => {
    return await invoke('mark_task_complete', { id, completed });
  },

  getStats: async (): Promise<TaskStats> => {
    return await invoke('get_task_stats');
  },
};
