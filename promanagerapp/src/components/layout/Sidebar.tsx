import React from 'react';
import { NavLink } from 'react-router-dom';
import { 
  LayoutDashboard, 
  FolderOpen, 
  CheckSquare, 
  Calendar,
  Settings 
} from 'lucide-react';

export const Sidebar: React.FC = () => {
  const navigationItems = [
    { 
      name: 'Dashboard', 
      href: '/dashboard', 
      icon: LayoutDashboard 
    },
    { 
      name: 'Projects', 
      href: '/projects', 
      icon: FolderOpen 
    },
    { 
      name: 'Tasks', 
      href: '/tasks', 
      icon: CheckSquare 
    },
    { 
      name: 'Calendar', 
      href: '/calendar', 
      icon: Calendar 
    },
  ];

  return (
    <div className="flex flex-col w-64 bg-white shadow-lg">
      {/* Logo/Header */}
      <div className="flex items-center justify-center h-16 px-4 border-b border-gray-200">
        <h1 className="text-xl font-bold text-gray-800">ProManager</h1>
      </div>

      {/* Navigation */}
      <nav className="flex-1 px-4 py-6 space-y-2">
        {navigationItems.map((item) => {
          const Icon = item.icon;
          return (
            <NavLink
              key={item.name}
              to={item.href}
              className={({ isActive }) =>
                `flex items-center px-3 py-2 rounded-lg text-sm font-medium transition-colors ${
                  isActive
                    ? 'bg-primary-100 text-primary-700 border-l-4 border-primary-700'
                    : 'text-gray-600 hover:bg-gray-100 hover:text-gray-900'
                }`
              }
            >
              <Icon className="w-5 h-5 mr-3" />
              {item.name}
            </NavLink>
          );
        })}
      </nav>

      {/* Footer */}
      <div className="px-4 py-4 border-t border-gray-200">
        <button className="flex items-center w-full px-3 py-2 text-sm font-medium text-gray-600 rounded-lg hover:bg-gray-100 hover:text-gray-900">
          <Settings className="w-5 h-5 mr-3" />
          Settings
        </button>
      </div>
    </div>
  );
};
