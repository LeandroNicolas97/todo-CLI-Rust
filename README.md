# todo-rust-cli-server

A Rust workspace with two projects that share the same task logic:

- **todo-cli** — CLI todo list, runnable from any terminal
- **todo-server** — Web interface served on `localhost:3000`

## Installation

Clone the repository:

```bash
git clone https://github.com/LeandroNicolas97/todo-rust-cli-server.git
cd todo-rust-cli-server
```

### CLI

```bash
cargo install --path todo-cli
```

Make sure `~/.cargo/bin` is in your PATH. Add this to your `~/.zshrc` or `~/.bashrc`:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

### Server

```bash
cargo run -p todo-server
```

Then open `http://localhost:3000` in your browser.

## CLI Usage

```bash
todo add "Buy milk"       # Add a task
todo list                 # List all tasks
todo list --pending       # List pending tasks
todo list --done          # List completed tasks
todo done 1               # Mark task 1 as done
todo delete 1             # Delete task 1
```
