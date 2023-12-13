use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TodoItem {
    pub id: i64,
    pub checked: bool,
    pub contents: String,
}

impl TodoItem {
    pub fn new(id: i64, contents: &'_ str) -> Self {
        Self {
            id,
            checked: false,
            contents: contents.to_owned(),
        }
    }
}

#[async_trait]
pub trait TodoRepository: Send + Sync + 'static {
    async fn list(&self) -> anyhow::Result<Vec<TodoItem>>;
}
