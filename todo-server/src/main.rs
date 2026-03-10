use axum::{
    extract::Path,
    response::{Html, Redirect},
    routing::{get, post},
    Form, Router,
};
use serde::Deserialize;
use tera::{Context, Tera};
use todo::storage::{load_tasks, save_tasks};
use todo::task::Task;
use tower_http::services::ServeDir;

#[derive(Deserialize)]
struct AddForm {
    description: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/add", post(add_task))
        .route("/done/{id}", post(mark_done))
        .route("/delete/{id}", post(delete_task))
        .nest_service("/static", ServeDir::new("todo-server/static"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Servidor corriendo en http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Html<String> {
    let tera = Tera::new("todo-server/templates/**/*").unwrap();
    let tasks = load_tasks();
    let mut ctx = Context::new();
    ctx.insert("tasks", &tasks);
    let html = tera.render("index.html", &ctx).unwrap();
    Html(html)
}

async fn add_task(Form(form): Form<AddForm>) -> Redirect {
    let mut tasks = load_tasks();
    let id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    tasks.push(Task::new(id, form.description));
    save_tasks(&tasks);
    Redirect::to("/")
}

async fn mark_done(Path(id): Path<u32>) -> Redirect {
    let mut tasks = load_tasks();
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.done = true;
    }
    save_tasks(&tasks);
    Redirect::to("/")
}

async fn delete_task(Path(id): Path<u32>) -> Redirect {
    let mut tasks = load_tasks();
    tasks.retain(|t| t.id != id);
    save_tasks(&tasks);
    Redirect::to("/")
}
