# ProManager - Project Management Desktop Application

A comprehensive project management desktop application built with Tauri (Rust backend) and React (TypeScript frontend).

## 🚀 Features

### Core Functionality
- **Task Management**: Create, edit, delete, and view tasks with priority levels and due dates
- **Project Organization**: Organize tasks into projects with custom colors and descriptions
- **Status Tracking**: Track task progress with To Do, In Progress, and Done statuses
- **Due Date Management**: Set and track due dates with overdue indicators
- **Dashboard Overview**: Get insights with task statistics and today's tasks
- **Offline First**: All data stored locally using SQLite database

### Technical Features
- **Modern UI**: Clean and responsive design with Tailwind CSS
- **Component Library**: Reusable UI components documented with Storybook
- **Type Safety**: Full TypeScript implementation for both frontend and backend
- **Local Storage**: SQLite database for offline functionality
- **Cross-Platform**: Works on macOS, Windows, and Linux

## 🛠 Tech Stack

### Backend (Tauri Container)
- **Rust** - High-performance backend with Tauri framework
- **SQLite** - Embedded database for local data persistence
- **Serde** - JSON serialization/deserialization
- **Chrono** - Date and time handling
- **UUID** - Unique identifier generation

### Frontend
- **React 18** - Modern React with functional components and hooks
- **TypeScript** - Type-safe JavaScript development
- **Tailwind CSS** - Utility-first CSS framework
- **React Router** - Client-side routing
- **Lucide React** - Beautiful SVG icons
- **Date-fns** - Date manipulation and formatting

### Development Tools
- **Vite** - Fast build tool and development server
- **Storybook** - Component development and documentation
- **pnpm** - Fast and efficient package manager

## 📁 Project Structure

```
promanagerapp/
├── src/                    # React frontend source
│   ├── components/         # Reusable UI components
│   │   ├── layout/        # Layout components (Sidebar, etc.)
│   │   └── ui/            # Base UI components (Button, Card, etc.)
│   ├── pages/             # Page components
│   │   ├── Dashboard.tsx   # Main dashboard view
│   │   ├── ProjectsPage.tsx
│   │   ├── TasksPage.tsx
│   │   └── ProjectDetailPage.tsx
│   ├── api.ts             # Tauri API communication
│   ├── types.ts           # TypeScript type definitions
│   ├── utils.ts           # Utility functions
│   └── index.css          # Global styles
├── src-tauri/             # Rust backend source
│   ├── src/
│   │   ├── commands.rs    # Tauri command handlers
│   │   ├── database.rs    # SQLite database operations
│   │   ├── models.rs      # Data models and types
│   │   └── main.rs        # Application entry point
│   ├── Cargo.toml         # Rust dependencies
│   └── tauri.conf.json    # Tauri configuration
├── .storybook/            # Storybook configuration
├── public/                # Static assets
└── dist/                  # Built frontend files
```

## 🚀 Getting Started

### Prerequisites
- [Node.js](https://nodejs.org/) (v16 or higher)
- [Rust](https://rustup.rs/) (latest stable version)
- [pnpm](https://pnpm.io/) package manager

### Installation

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd promanager/promanagerapp
   ```

2. **Install frontend dependencies**
   ```bash
   pnpm install
   ```

3. **Install Tauri CLI** (if not already installed)
   ```bash
   cargo install tauri-cli
   ```

### Development

1. **Start the development server**
   ```bash
   pnpm tauri dev
   ```
   This will start both the React development server and the Tauri application.

2. **Run Storybook** (optional, for component development)
   ```bash
   pnpm storybook
   ```

### Building for Production

1. **Build the application**
   ```bash
   pnpm tauri build
   ```
   This creates platform-specific installers in `src-tauri/target/release/bundle/`

## 📊 Database Schema

### Projects Table
```sql
CREATE TABLE projects (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    color TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
```

### Tasks Table
```sql
CREATE TABLE tasks (
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
);
```

## 🎨 UI Components

The application includes a comprehensive set of reusable UI components:

- **Button** - Primary, secondary, and danger variants with different sizes
- **Card** - Container component with configurable padding
- **Sidebar** - Navigation component with active state management
- **Dashboard** - Statistics cards and task overview widgets

All components are documented and can be viewed in Storybook.

## 🔧 Available Scripts

```bash
# Development
pnpm dev              # Start Vite development server
pnpm tauri dev        # Start Tauri development mode

# Building
pnpm build            # Build frontend for production
pnpm tauri build      # Build complete application

# Storybook
pnpm storybook        # Start Storybook development server
pnpm build-storybook  # Build Storybook for deployment
```

## 🎯 Roadmap

### Phase 1 (Current)
- ✅ Basic task and project management
- ✅ SQLite database integration
- ✅ Modern React UI with TypeScript
- ✅ Storybook component library

### Phase 2 (Future Enhancements)
- 🔲 Task reminders and notifications
- 🔲 Task tags and categories
- 🔲 Calendar view integration
- 🔲 Data export/import functionality
- 🔲 Search and filtering capabilities
- 🔲 Keyboard shortcuts
- 🔲 Dark mode support

### Phase 3 (Advanced Features)
- 🔲 Cloud synchronization
- 🔲 Team collaboration features
- 🔲 Time tracking
- 🔲 Reporting and analytics
- 🔲 Plugin system

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Tauri](https://tauri.app/) - For the excellent desktop app framework
- [React](https://reactjs.org/) - For the powerful UI library
- [Tailwind CSS](https://tailwindcss.com/) - For the utility-first CSS framework
- [Lucide](https://lucide.dev/) - For the beautiful icon set

---

**ProManager** - Building productivity, one task at a time. 🚀