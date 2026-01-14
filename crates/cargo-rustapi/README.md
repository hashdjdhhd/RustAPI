# cargo-rustapi

**The official CLI tool for the RustAPI framework.**

Use this tool to scaffold new projects, generate code, and fast-track your development workflow.

## 📦 Installation

```bash
cargo install cargo-rustapi
```

## 🛠️ Usage

### Creating a New Project

Use the `new` command to generate a project structure.

```bash
# Interactive mode (Recommended)
cargo rustapi new my-app

# Quick start with specific template
cargo rustapi new my-app --template api
```

**Available Templates:**
- `minimal`: Basic `main.rs` and `Cargo.toml`.
- `api`: REST API structure with separated `handlers` and `models`.
- `web`: Web application with HTML templates (`rustapi-view`).
- `full`: Complete example with Database, Auth, and Docker support.

### Running Development Server

Run your application with hot-reloading (requires `cargo-watch`).

```bash
cargo rustapi run
```

### Code Generation

Save time by generating boilerplate.

```bash
# Generate a handler function and register it
cargo rustapi generate handler users

# Generate a database model
cargo rustapi generate model User

# Generate a full CRUD resource (Model + Handlers + Tests)
cargo rustapi generate crud product
```

### Managing Migrations (Planned)

```bash
cargo rustapi migrate run
cargo rustapi migrate revert
```
