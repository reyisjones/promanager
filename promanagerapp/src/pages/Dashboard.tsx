import React, { useState, useEffect } from 'react';
import { Plus, Calendar, CheckCircle, Clock, AlertTriangle } from 'lucide-react';
import { Task, TaskStats, Project, CreateTask } from '../types';
import { taskApi, projectApi } from '../api';
import { formatDate, isOverdue } from '../utils';
import { TaskModal } from '../components/TaskModal';

export const Dashboard: React.FC = () => {
  const [stats, setStats] = useState<TaskStats | null>(null);
  const [todayTasks, setTodayTasks] = useState<Task[]>([]);
  const [upcomingTasks, setUpcomingTasks] = useState<Task[]>([]);
  const [projects, setProjects] = useState<Project[]>([]);
  const [loading, setLoading] = useState(true);
  const [isTaskModalOpen, setIsTaskModalOpen] = useState(false);

  useEffect(() => {
    loadDashboardData();
  }, []);

  const loadDashboardData = async () => {
    try {
      setLoading(true);
      const [statsData, todayData, upcomingData, projectsData] = await Promise.all([
        taskApi.getStats(),
        taskApi.getToday(),
        taskApi.getUpcoming(),
        projectApi.getAll(),
      ]);

      setStats(statsData);
      setTodayTasks(todayData);
      setUpcomingTasks(upcomingData);
      setProjects(projectsData);
    } catch (error) {
      console.error('Error loading dashboard data:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleToggleTaskComplete = async (taskId: string, completed: boolean) => {
    try {
      await taskApi.markComplete(taskId, completed);
      await loadDashboardData(); // Refresh data
    } catch (error) {
      console.error('Error updating task:', error);
    }
  };

  const handleCreateTask = async (taskData: CreateTask) => {
    try {
      await taskApi.create(taskData);
      await loadDashboardData(); // Refresh data
    } catch (error) {
      console.error('Error creating task:', error);
      throw error;
    }
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center h-full">
        <div className="text-lg text-gray-600">Loading dashboard...</div>
      </div>
    );
  }

  return (
    <div className="flex flex-col w-full h-full overflow-hidden">
      {/* Header */}
      <div className="flex items-center justify-between p-6 bg-white border-b border-gray-200">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Dashboard</h1>
          <p className="text-gray-600">Welcome back! Here's your project overview.</p>
        </div>
        <button 
          onClick={() => setIsTaskModalOpen(true)}
          className="flex items-center px-4 py-2 text-white bg-primary-600 rounded-lg hover:bg-primary-700 transition-colors"
        >
          <Plus className="w-4 h-4 mr-2" />
          Quick Add Task
        </button>
      </div>

      {/* Main Content */}
      <div className="flex-1 p-6 overflow-auto">
        {/* Stats Cards */}
        {stats && (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
            <div className="bg-white p-6 rounded-lg shadow-sm border border-gray-200">
              <div className="flex items-center">
                <div className="p-2 bg-blue-100 rounded-lg">
                  <CheckCircle className="w-6 h-6 text-blue-600" />
                </div>
                <div className="ml-4">
                  <p className="text-sm font-medium text-gray-600">Total Tasks</p>
                  <p className="text-2xl font-bold text-gray-900">{stats.total_tasks}</p>
                </div>
              </div>
            </div>

            <div className="bg-white p-6 rounded-lg shadow-sm border border-gray-200">
              <div className="flex items-center">
                <div className="p-2 bg-green-100 rounded-lg">
                  <CheckCircle className="w-6 h-6 text-green-600" />
                </div>
                <div className="ml-4">
                  <p className="text-sm font-medium text-gray-600">Completed</p>
                  <p className="text-2xl font-bold text-gray-900">{stats.completed_tasks}</p>
                </div>
              </div>
            </div>

            <div className="bg-white p-6 rounded-lg shadow-sm border border-gray-200">
              <div className="flex items-center">
                <div className="p-2 bg-yellow-100 rounded-lg">
                  <Clock className="w-6 h-6 text-yellow-600" />
                </div>
                <div className="ml-4">
                  <p className="text-sm font-medium text-gray-600">Today's Tasks</p>
                  <p className="text-2xl font-bold text-gray-900">{stats.today_tasks}</p>
                </div>
              </div>
            </div>

            <div className="bg-white p-6 rounded-lg shadow-sm border border-gray-200">
              <div className="flex items-center">
                <div className="p-2 bg-red-100 rounded-lg">
                  <AlertTriangle className="w-6 h-6 text-red-600" />
                </div>
                <div className="ml-4">
                  <p className="text-sm font-medium text-gray-600">Overdue</p>
                  <p className="text-2xl font-bold text-gray-900">{stats.overdue_tasks}</p>
                </div>
              </div>
            </div>
          </div>
        )}

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
          {/* Today's Tasks */}
          <div className="bg-white rounded-lg shadow-sm border border-gray-200">
            <div className="p-6 border-b border-gray-200">
              <div className="flex items-center">
                <Calendar className="w-5 h-5 text-primary-600 mr-2" />
                <h2 className="text-lg font-semibold text-gray-900">Today's Tasks</h2>
              </div>
            </div>
            <div className="p-6">
              {todayTasks.length === 0 ? (
                <div className="text-center text-gray-500 py-8">
                  <Calendar className="w-12 h-12 mx-auto text-gray-300 mb-4" />
                  <p>No tasks scheduled for today</p>
                </div>
              ) : (
                <div className="space-y-3">
                  {todayTasks.map((task) => (
                    <div
                      key={task.id}
                      className={`flex items-center p-3 rounded-lg border ${
                        task.completed 
                          ? 'bg-green-50 border-green-200' 
                          : isOverdue(task.due_date || '') 
                            ? 'bg-red-50 border-red-200' 
                            : 'bg-gray-50 border-gray-200'
                      }`}
                    >
                      <input
                        type="checkbox"
                        checked={task.completed}
                        onChange={(e) => handleToggleTaskComplete(task.id, e.target.checked)}
                        className="mr-3 h-4 w-4 text-primary-600 rounded border-gray-300"
                      />
                      <div className="flex-1">
                        <p className={`font-medium ${task.completed ? 'line-through text-gray-500' : 'text-gray-900'}`}>
                          {task.title}
                        </p>
                        {task.due_date && (
                          <p className="text-sm text-gray-500">
                            Due: {formatDate(task.due_date)}
                          </p>
                        )}
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </div>
          </div>

          {/* Recent Projects */}
          <div className="bg-white rounded-lg shadow-sm border border-gray-200">
            <div className="p-6 border-b border-gray-200">
              <h2 className="text-lg font-semibold text-gray-900">Recent Projects</h2>
            </div>
            <div className="p-6">
              {projects.length === 0 ? (
                <div className="text-center text-gray-500 py-8">
                  <CheckCircle className="w-12 h-12 mx-auto text-gray-300 mb-4" />
                  <p>No projects yet</p>
                </div>
              ) : (
                <div className="space-y-3">
                  {projects.slice(0, 5).map((project) => (
                    <div key={project.id} className="flex items-center p-3 rounded-lg border border-gray-200 hover:bg-gray-50 cursor-pointer transition-colors">
                      <div
                        className="w-4 h-4 rounded-full mr-3"
                        style={{ backgroundColor: project.color }}
                      />
                      <div className="flex-1">
                        <p className="font-medium text-gray-900">{project.name}</p>
                        {project.description && (
                          <p className="text-sm text-gray-500">{project.description}</p>
                        )}
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </div>
          </div>
        </div>

        {/* Upcoming Tasks */}
        {upcomingTasks.length > 0 && (
          <div className="mt-8 bg-white rounded-lg shadow-sm border border-gray-200">
            <div className="p-6 border-b border-gray-200">
              <h2 className="text-lg font-semibold text-gray-900">Upcoming Tasks</h2>
            </div>
            <div className="p-6">
              <div className="space-y-3">
                {upcomingTasks.slice(0, 5).map((task) => (
                  <div key={task.id} className="flex items-center p-3 rounded-lg border border-gray-200">
                    <input
                      type="checkbox"
                      checked={task.completed}
                      onChange={(e) => handleToggleTaskComplete(task.id, e.target.checked)}
                      className="mr-3 h-4 w-4 text-primary-600 rounded border-gray-300"
                    />
                    <div className="flex-1">
                      <p className="font-medium text-gray-900">{task.title}</p>
                      {task.due_date && (
                        <p className="text-sm text-gray-500">
                          Due: {formatDate(task.due_date)}
                        </p>
                      )}
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </div>
        )}
      </div>

      <TaskModal
        isOpen={isTaskModalOpen}
        onClose={() => setIsTaskModalOpen(false)}
        onSubmit={handleCreateTask}
        projects={projects}
      />
    </div>
  );
};
