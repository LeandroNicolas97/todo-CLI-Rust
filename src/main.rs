mod task;
mod storage;

use clap::{Parser, Subcommand};
use colored::Colorize;
use task::Task;

#[derive(Parser)]
#[command(name = "todo", about = "Un CLI de tareas en Rust")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Agregar una nueva tarea
    Add { description: String },
    /// Listar todas las tareas
    List {
        /// Mostrar solo tareas pendientes
        #[arg(long)]
        pending: bool,
        /// Mostrar solo tareas completadas
        #[arg(long)]
        done: bool,
    },
    /// Marcar una tarea como completada
    Done { id: u32 },
    /// Eliminar una tarea
    Delete { id: u32 },
}

fn main() {
    let cli = Cli::parse();
    let mut tasks = storage::load_tasks();

    match cli.command {
        Command::Add { description } => {
            let id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
            let task = Task::new(id, description);
            println!("{}", format!("Tarea agregada: {}", task).green());
            tasks.push(task);
            storage::save_tasks(&tasks);
        }

        Command::List { pending, done } => {
            let filtered: Vec<&Task> = tasks.iter().filter(|t| {
                if pending { !t.done }
                else if done { t.done }
                else { true }
            }).collect();

            if filtered.is_empty() {
                println!("{}", "No hay tareas.".yellow());
            } else {
                for task in filtered {
                    let line = task.to_string();
                    if task.done {
                        println!("{}", line.green());
                    } else {
                        println!("{}", line.white());
                    }
                }
            }
        }

        Command::Done { id } => {
            match tasks.iter_mut().find(|t| t.id == id) {
                Some(task) => {
                    task.done = true;
                    println!("{}", format!("Tarea {} marcada como completada.", id).green());
                    storage::save_tasks(&tasks);
                }
                None => println!("{}", format!("No existe tarea con id {}.", id).red()),
            }
        }

        Command::Delete { id } => {
            let before = tasks.len();
            tasks.retain(|t| t.id != id);
            if tasks.len() < before {
                println!("{}", format!("Tarea {} eliminada.", id).red());
                storage::save_tasks(&tasks);
            } else {
                println!("{}", format!("No existe tarea con id {}.", id).red());
            }
        }
    }
}
