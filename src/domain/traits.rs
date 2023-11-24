use async_trait::async_trait;

use super::entities::Todo;

#[async_trait]
pub trait Repo: Send + Sync {
    async fn get_todos(&self) -> Result<Vec<Todo>, super::errors::Error>;
    async fn insert_todo(&self, title: Todo) -> Result<Todo, super::errors::Error>;
    async fn get_todo_by_id(&self, id: String) -> Result<Todo, super::errors::Error>;
    async fn update_todo_by_id(
        &self,
        id: String,
        update_fn: fn(Todo) -> Todo,
    ) -> Result<Todo, super::errors::Error>;
}

#[async_trait]
pub trait App: Send + Sync {
    async fn get_todos(&self) -> Result<Vec<Todo>, super::errors::Error>;
    async fn create_todo(&self, title: String) -> Result<Todo, super::errors::Error>;
    async fn get_todo_by_id(&self, id: String) -> Result<Todo, super::errors::Error>;
    async fn complete_todo_by_id(&self, id: String) -> Result<Todo, super::errors::Error>;
}
