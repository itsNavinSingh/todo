# 📝 Todo CLI

A fast, project‑scoped **command‑line Todo manager** written in **Rust**.
This tool helps you manage tasks **per project**, storing all data locally inside the project directory.

---

## ✨ Features

* 📁 **Project‑local todos** (stored in a hidden `.todo/` directory)
* ⚡ **Fast binary storage** using `bincode`
* 🧩 Modular command architecture
* 🎨 Colored terminal output
* 📤 Export tasks to **CSV** or **JSON**
* 🔍 Search, sort, and filter tasks
* 📊 Task statistics

---

## 📦 Installation

### From source

```bash
git clone https://github.com/itsNavinSingh/todo.git
cd todo
cargo build --release
```

(Optional) Install globally:

```bash
cargo install --path .
```

---

## 🚀 Getting Started

Inside your project directory:

```bash
todo init
```

This creates:

```text
.todo/
└── todos.bin
```

> The `.todo/` directory is searched **upward** from the current directory, so subdirectories automatically share the same todo list.

---

## 🧠 How It Works

* Tasks are stored in `.todo/todos.bin`
* Data is serialized using **bincode**
* Every command:

  1. Finds `.todo/` in the current or parent directories
  2. Reads tasks into memory
  3. Applies changes
  4. Writes back to disk

No database, no config files, no background services.

---

## 📌 Commands

### Initialize Todo

```bash
todo init
```

---

### Add a Task

```bash
todo add \
  --title "Fix bug" \
  --due "18-12-2025 14:00:00" \
  --priority high
```

Options:

* `--title, -t` — task title (required)
* `--due, -d` — due date (`DD-MM-YYYY HH:MM:SS`)
* `--priority, -p` — `high | medium | low`
* `--complete, -c` — mark task completed

---

### List Tasks

```bash
todo list
```

---

### Edit a Task

```bash
todo edit 2 --title "Updated title" --complete true
```

Editable fields:

* title
* priority
* due date
* completion status

---

### Delete a Task

```bash
todo delete 3
```

---

### Clear Completed Tasks

```bash
todo clear
```

---

### Reset Task IDs

```bash
todo reset
```

Renumbers tasks starting from `1`.

---

### Search Tasks

```bash
todo search bug
```

Searches task titles containing the given text.

---

### Sort Tasks

```bash
todo sort --by priority
```

Sort options:

* `id`
* `title`
* `complete`
* `priority`
* `creation`
* `due`

---

### Show Statistics

```bash
todo stats
```

Displays:

* Total tasks
* Completed vs pending
* Priority distribution

---

### Export Tasks

```bash
todo export --format csv --output tasks
```

Formats:

* `csv`
* `json`

---

### Remove Todo from Project

```bash
todo kill
```

Deletes the entire `.todo/` directory.

---

## 🗂️ Project Structure

```text
src/
├── main.rs            # Entry point
├── arguments.rs       # CLI argument definitions
├── tasks.rs           # Task data models
├── commandimpl/       # Command implementations
│   ├── add.rs
│   ├── edit.rs
│   ├── delete.rs
│   ├── list.rs
│   └── ...
└── utility/           # Helpers
    ├── finddir.rs     # Locate .todo directory
    └── conjucture.rs  # Serialization helpers
```

---

## 🛠️ Tech Stack

* **Rust 2024 Edition**
* [`clap`](https://crates.io/crates/clap) — CLI parsing
* [`bincode`](https://crates.io/crates/bincode) — binary serialization
* [`serde`](https://crates.io/crates/serde) — data serialization
* [`chrono`](https://crates.io/crates/chrono) — date & time
* [`colored`](https://crates.io/crates/colored) — terminal colors

---

## 🧪 Future Improvements

* Tags & labels
* Recurring tasks
* Undo / history
* Configurable date formats
* Task dependencies
* Git ignore helper for `.todo/`

---

## 📄 License

MIT License

---

## 👤 Author

**Navin Kumar Singh**
