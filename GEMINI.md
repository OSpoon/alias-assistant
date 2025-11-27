# Gemini Project Context: alias-assistant

## Project Overview

This project is a desktop application named "alias-assistant" built using the Tauri framework. The frontend is developed with Vue.js 3 and TypeScript, while the backend is written in Rust.

The application is structured as a standard Tauri project, with the frontend code in the `src` directory and the backend Rust code in the `src-tauri` directory.

### Key Technologies:
- **Tauri:** A framework for building lightweight, cross-platform desktop applications using web technologies.
- **Vue.js 3:** A progressive JavaScript framework for building user interfaces.
- **TypeScript:** A statically typed superset of JavaScript.
- **Vite:** A fast build tool and development server for modern web projects.
- **Rust:** A systems programming language used for the backend logic.
- **pnpm:** The package manager used for frontend dependencies.

## Building and Running

The project uses `pnpm` for frontend package management and `cargo` (via the Tauri CLI) for the backend.

### Development

To run the application in development mode, use the following command:

```bash
pnpm tauri dev
```

This command will:
1.  Start the Vite development server for the Vue.js frontend (as defined in `beforeDevCommand` in `src-tauri/tauri.conf.json`).
2.  Launch the Tauri application window, which will load the frontend from the Vite server.

### Building

To build the application for production, use the following command:

```bash
pnpm tauri build
```

This command will:
1.  Build the Vue.js frontend and place the output in the `dist` directory (as defined in `beforeBuildCommand` in `src-tauri/tauri.conf.json`).
2.  Build the Rust backend and bundle it with the frontend into a standalone desktop application.

## Development Conventions

### Frontend
- The frontend code is located in the `src` directory.
- The main Vue.js component is `src/App.vue`.
- The entry point for the frontend is `src/main.ts`.
- Frontend dependencies are managed in `package.json` using `pnpm`.

### Backend
- The backend code is located in the `src-tauri` directory.
- The main entry point for the backend is `src-tauri/src/main.rs`.
- The core backend logic and Tauri setup are in `src-tauri/src/lib.rs`.
- Backend dependencies are managed in `src-tauri/Cargo.toml`.

### Communication
- The frontend and backend can communicate using Tauri's command system. An example `greet` command is defined in `src-tauri/src/lib.rs`.
