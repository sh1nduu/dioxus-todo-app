use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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
    async fn add(&self, contents: &'_ str) -> anyhow::Result<TodoItem>;
    async fn delete(&self, id: i64) -> anyhow::Result<bool>;
    async fn toggle(&self, id: i64) -> anyhow::Result<Option<TodoItem>>;
    async fn clear_completed(&self, ids: &[i64]) -> anyhow::Result<bool>;
}
