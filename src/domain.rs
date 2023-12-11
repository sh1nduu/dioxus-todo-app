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
