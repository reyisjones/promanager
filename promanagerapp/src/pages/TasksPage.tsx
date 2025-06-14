import React from 'react';

export const TasksPage: React.FC = () => {
  return (
    <div className="flex flex-col w-full h-full overflow-hidden">
      <div className="flex items-center justify-between p-6 bg-white border-b border-gray-200">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Tasks</h1>
          <p className="text-gray-600">Manage all your tasks in one place.</p>
        </div>
      </div>
      <div className="flex-1 p-6 overflow-auto">
        <div className="text-center text-gray-500 py-8">
          <p>Tasks page coming soon...</p>
        </div>
      </div>
    </div>
  );
};
