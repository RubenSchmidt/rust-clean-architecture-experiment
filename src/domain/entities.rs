use uuid::Uuid;

pub struct Todo {
    id: String,
    title: String,
    completed: bool,
}

impl Todo {
    pub fn new(title: String) -> Self {
        Self {
            id: Uuid::now_v7().to_string(),
            title,
            completed: false,
        }
    }

    pub fn from_data_storage(id: String, title: String, completed: bool) -> Self {
        Self {
            id,
            title,
            completed,
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn completed(&self) -> bool {
        self.completed
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }
}
