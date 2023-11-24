use crate::domain::errors::Error as AppError;
use crate::domain::{entities::Todo, traits::Repo};
use async_trait::async_trait;
use sqlx::FromRow;
use uuid::Uuid;

pub struct PgRepo {
    db: sqlx::PgPool,
}

impl PgRepo {
    pub fn new(db: sqlx::PgPool) -> Self {
        Self { db }
    }
    async fn _get_todo_by_id(&self, id: String) -> Result<PgTodo, AppError> {
        let uuid = Uuid::parse_str(&id)?;
        let todo = sqlx::query_as::<_, PgTodo>("SELECT * FROM todos WHERE id = $1")
            .bind(uuid)
            .fetch_one(&self.db)
            .await?;
        Ok(todo)
    }
}

#[derive(FromRow)]
struct PgTodo {
    id: Uuid,
    title: String,
    completed: bool,
}

#[async_trait]
impl Repo for PgRepo {
    async fn get_todos(&self) -> Result<Vec<Todo>, AppError> {
        let out = sqlx::query_as::<_, PgTodo>("SELECT * FROM todos ORDER BY id DESC")
            .fetch_all(&self.db)
            .await?
            .into_iter()
            .map(to_domain)
            .collect();
        Ok(out)
    }

    async fn insert_todo(&self, todo: Todo) -> Result<Todo, AppError> {
        let todo = from_domain(todo)?;
        let todo = sqlx::query_as::<_, PgTodo>(
            "INSERT INTO todos (id, title, completed) VALUES ($1, $2, false) RETURNING *",
        )
        .bind(todo.id)
        .bind(todo.title)
        .bind(todo.completed)
        .fetch_one(&self.db)
        .await?;
        Ok(to_domain(todo))
    }

    async fn get_todo_by_id(&self, id: String) -> Result<Todo, AppError> {
        self._get_todo_by_id(id).await.map(to_domain)
    }

    async fn update_todo_by_id(
        &self,
        id: String,
        update_fn: fn(Todo) -> Todo,
    ) -> Result<Todo, AppError> {
        let todo = self._get_todo_by_id(id).await?;
        let todo = update_fn(to_domain(todo));
        let todo = from_domain(todo)?;

        let todo = sqlx::query_as::<_, PgTodo>(
            "UPDATE todos SET (title, completed) = ($1, $2) WHERE id = $3 RETURNING *",
        )
        .bind(todo.title)
        .bind(todo.completed)
        .bind(todo.id)
        .fetch_one(&self.db)
        .await?;

        Ok(to_domain(todo))
    }
}

fn from_domain(todo: Todo) -> Result<PgTodo, AppError> {
    let id = Uuid::parse_str(&todo.id())?;
    Ok(PgTodo {
        id,
        title: todo.title().clone(),
        completed: todo.completed(),
    })
}

fn to_domain(todo: PgTodo) -> Todo {
    Todo::from_data_storage(todo.id.to_string(), todo.title, todo.completed)
}

impl From<uuid::Error> for AppError {
    fn from(inner: uuid::Error) -> Self {
        tracing::error!("uuid error: {:?}", inner);
        AppError::Internal
    }
}

/// This makes it possible to use `?` to automatically convert a `sqlx::Error`
/// into an `AppError`.
impl From<sqlx::Error> for AppError {
    fn from(inner: sqlx::Error) -> Self {
        tracing::error!("sqlx error: {:?}", inner);
        match inner {
            sqlx::Error::RowNotFound => return AppError::NotFound,
            _ => AppError::Internal,
        }
    }
}
