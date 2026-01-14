# cargo-rustapi

The official CLI tool for the RustAPI framework. Scaffold new projects, run development servers, and manage database migrations.

## Installation

`ash
cargo install cargo-rustapi
`

## Features

- **Project Scaffolding**: Create new projects with 
ew command, choosing from templates like pi, web, or ull.
- **Development Server**: Run your project with un command, supporting hot-reloading (via cargo-watch integration if available).
- **Code Generation**: Generate handlers, models, and CRUD operations with generate.
- **Database Management**: (Planned) Simple wrappers around migration tools.

## Usage

### Create a New Project

`ash
# Interactive mode
cargo rustapi new my-project

# With template
cargo rustapi new my-project --template api

# With features
cargo rustapi new my-project --features jwt,cors
`

### Run Development Server

`ash
# Run with auto-reload
cargo rustapi run

# Run on specific port
cargo rustapi run --port 8080
`

### Generate Code

`ash
# Generate a new handler
cargo rustapi generate handler users

# Generate a model
cargo rustapi generate model User

# Generate CRUD endpoints (Handlers + Models + Tests)
cargo rustapi generate crud users
`

## Templates

- minimal: Bare bones setup.
- pi: REST API structure with handlers and models.
- web: Includes ustapi-view and 	emplates folder.
- ull: Complete setup with Auth, DB (SQLx), and more.

## License

MIT OR Apache-2.0
