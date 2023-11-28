use std::sync::Arc;

use axum::extract::Path;
use axum::response::Response;
use axum::{
    extract::State, http::StatusCode, response::IntoResponse, routing::get, routing::patch, Json,
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::domain;
use crate::domain::entities::Todo;
use crate::domain::traits::App;

#[derive(Debug, Serialize, Deserialize)]
struct TodoResponse {
    id: String,
    title: String,
    completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateTodoRequest {
    title: String,
}

pub fn router(app: Arc<impl App + 'static>) -> Router {
    Router::new()
        .route("/", get(get_todos).post(create_todo))
        .route("/:id", get(get_todo))
        .route("/:id/complete", patch(complete_todo))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .with_state(app)
}

async fn get_todos(
    State(app): State<Arc<impl App>>,
) -> Result<Json<Vec<TodoResponse>>, domain::errors::Error> {
    let todos = app.get_todos().await?;
    let todos = todos.into_iter().map(Into::into).collect();
    Ok(Json(todos))
}

async fn get_todo(
    State(app): State<Arc<impl App>>,
    Path(id): Path<String>,
) -> Result<Json<TodoResponse>, domain::errors::Error> {
    let todo = app.get_todo_by_id(id).await?;
    Ok(Json(todo.into()))
}

async fn create_todo(
    State(app): State<Arc<impl App>>,
    Json(todo): Json<CreateTodoRequest>,
) -> Result<Json<TodoResponse>, domain::errors::Error> {
    let todo = app.create_todo(todo.title).await?;
    Ok(Json(todo.into()))
}

async fn complete_todo(
    State(app): State<Arc<impl App>>,
    Path(id): Path<String>,
) -> Result<Json<TodoResponse>, domain::errors::Error> {
    let todo = app.complete_todo_by_id(id).await?;
    Ok(Json(todo.into()))
}

impl From<Todo> for TodoResponse {
    fn from(todo: Todo) -> Self {
        Self {
            id: todo.id().clone(),
            title: todo.title().clone(),
            completed: todo.completed(),
        }
    }
}

impl IntoResponse for domain::errors::Error {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            domain::errors::Error::NotFound => (StatusCode::NOT_FOUND, "not found"),
            domain::errors::Error::Internal => {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
