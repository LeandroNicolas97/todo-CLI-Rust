# todo CLI
A full CLI todo list in Rust, runnable from any directory.


  ## Installation

  Clone the repository and install with Cargo:

  \`\`\`bash
  git clone https://github.com/LeandroNicolas97/todo-CLI-Rust.git
  cd todo-CLI-Rust
  cargo install --path .
  \`\`\`

  Make sure `~/.cargo/bin` is in your PATH. Add this to your `~/.zshrc` or `~/.bashrc`:

  \`\`\`bash
  export PATH="$HOME/.cargo/bin:$PATH"
  \`\`\`

  ## Usage

  \`\`\`bash
  todo add "Buy milk"       # Add a task
  todo list                 # List all tasks
  todo list --pending       # List pending tasks
  todo list --done          # List completed tasks
  todo done 1               # Mark task 1 as done
  todo delete 1             # Delete task 1
  \`\`\`
