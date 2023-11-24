use async_trait::async_trait;

use crate::domain::{
    entities::Todo,
    errors::Error,
    traits::{App, Repo},
};

pub struct AppImpl {
    repo: Box<dyn Repo>,
}

impl AppImpl {
    pub fn new(repo: Box<dyn Repo>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl App for AppImpl {
    async fn get_todos(&self) -> Result<Vec<Todo>, Error> {
        self.repo.get_todos().await
    }

    async fn create_todo(&self, title: String) -> Result<Todo, Error> {
        let todo = Todo::new(title);
        self.repo.insert_todo(todo).await
    }

    async fn get_todo_by_id(&self, id: String) -> Result<Todo, Error> {
        self.repo.get_todo_by_id(id).await
    }

    async fn complete_todo_by_id(&self, id: String) -> Result<Todo, Error> {
        let todo = self
            .repo
            .update_todo_by_id(id, |mut todo| {
                todo.complete();
                todo
            })
            .await?;

        Ok(todo)
    }
}
