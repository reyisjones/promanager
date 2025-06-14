import React, { useState } from 'react';
import { X } from 'lucide-react';
import { CreateTask, TaskStatus, TaskPriority, Project } from '../types';
import { Button } from './ui/Button';
import { Input } from './ui/Input';
import { Select } from './ui/Select';

interface TaskModalProps {
  isOpen: boolean;
  onClose: () => void;
  onSubmit: (task: CreateTask) => Promise<void>;
  projects: Project[];
}

export const TaskModal: React.FC<TaskModalProps> = ({
  isOpen,
  onClose,
  onSubmit,
  projects,
}) => {
  const [formData, setFormData] = useState<CreateTask>({
    title: '',
    description: '',
    project_id: '',
    status: 'todo' as TaskStatus,
    priority: 'medium' as TaskPriority,
    due_date: '',
  });
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsSubmitting(true);
    
    try {
      const taskData = {
        ...formData,
        due_date: formData.due_date ? new Date(formData.due_date).toISOString() : undefined,
      };
      await onSubmit(taskData);
      
      // Reset form
      setFormData({
        title: '',
        description: '',
        project_id: '',
        status: 'todo' as TaskStatus,
        priority: 'medium' as TaskPriority,
        due_date: '',
      });
      onClose();
    } catch (error) {
      console.error('Error creating task:', error);
    } finally {
      setIsSubmitting(false);
    }
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-white rounded-lg shadow-xl w-full max-w-md mx-4">
        <div className="flex items-center justify-between p-6 border-b border-gray-200">
          <h2 className="text-lg font-semibold text-gray-900">Create New Task</h2>
          <button
            onClick={onClose}
            className="text-gray-400 hover:text-gray-600 transition-colors"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        <form onSubmit={handleSubmit} className="p-6 space-y-4">
          <Input
            label="Title"
            placeholder="Enter task title"
            value={formData.title}
            onChange={(value) => setFormData({ ...formData, title: value })}
            required
          />

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              Description
            </label>
            <textarea
              value={formData.description}
              onChange={(e) => setFormData({ ...formData, description: e.target.value })}
              placeholder="Enter task description (optional)"
              rows={3}
              className="w-full px-3 py-2 border border-gray-300 rounded-lg shadow-sm focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
            />
          </div>

          <Select
            label="Project"
            value={formData.project_id}
            onChange={(value) => setFormData({ ...formData, project_id: value })}
            placeholder="Select a project (optional)"
            options={projects.map((project) => ({
              value: project.id,
              label: project.name,
            }))}
          />

          <div className="grid grid-cols-2 gap-4">
            <Select
              label="Priority"
              value={formData.priority}
              onChange={(value) => setFormData({ ...formData, priority: value as TaskPriority })}
              options={[
                { value: 'low', label: 'Low' },
                { value: 'medium', label: 'Medium' },
                { value: 'high', label: 'High' },
              ]}
              required
            />

            <Select
              label="Status"
              value={formData.status}
              onChange={(value) => setFormData({ ...formData, status: value as TaskStatus })}
              options={[
                { value: 'todo', label: 'To Do' },
                { value: 'in_progress', label: 'In Progress' },
                { value: 'done', label: 'Done' },
              ]}
              required
            />
          </div>

          <Input
            label="Due Date"
            type="date"
            value={formData.due_date}
            onChange={(value) => setFormData({ ...formData, due_date: value })}
          />

          <div className="flex space-x-3 pt-4">
            <Button
              type="button"
              variant="secondary"
              onClick={onClose}
              className="flex-1"
              disabled={isSubmitting}
            >
              Cancel
            </Button>
            <Button
              type="submit"
              variant="primary"
              className="flex-1"
              disabled={isSubmitting || !formData.title.trim()}
            >
              {isSubmitting ? 'Creating...' : 'Create Task'}
            </Button>
          </div>
        </form>
      </div>
    </div>
  );
};
