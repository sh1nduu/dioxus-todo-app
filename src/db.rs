use async_trait::async_trait;
use sqlx::SqlitePool;
use std::env;

use crate::domain::{TodoItem, TodoRepository};

#[derive(Clone)]
pub struct TodoRepositoryImpl {
    pool: SqlitePool,
}

impl TodoRepositoryImpl {
    pub fn new(pool: &SqlitePool) -> Self {
        Self { pool: pool.clone() }
    }
}

#[async_trait]
impl TodoRepository for TodoRepositoryImpl {
    async fn list(&self) -> anyhow::Result<Vec<TodoItem>> {
        list_todos(&self.pool).await
    }
    async fn add(&self, contents: &'_ str) -> anyhow::Result<TodoItem> {
        add_todo(&self.pool, contents).await
    }
    async fn delete(&self, id: i64) -> anyhow::Result<bool> {
        delete_todo(&self.pool, id).await
    }
}

pub async fn establish() -> anyhow::Result<SqlitePool> {
    Ok(SqlitePool::connect(&env::var("DATABASE_URL")?).await?)
}

pub async fn add_todo(pool: &SqlitePool, contents: &'_ str) -> anyhow::Result<TodoItem> {
    let mut conn = pool.acquire().await?;
    let id = sqlx::query!(
        "INSERT INTO todo_items ( contents, checked ) VALUES ( ?1 , 0 )",
        contents
    )
    .execute(&mut *conn)
    .await?
    .last_insert_rowid();

    Ok(TodoItem::new(id, contents))
}

pub async fn complete_todo(pool: &SqlitePool, id: i64) -> anyhow::Result<bool> {
    let rows_affected = sqlx::query!("UPDATE todo_items SET checked = TRUE WHERE id = ?1 ", id)
        .execute(pool)
        .await?
        .rows_affected();

    Ok(rows_affected > 0)
}

pub async fn list_todos(pool: &SqlitePool) -> anyhow::Result<Vec<TodoItem>> {
    let items = sqlx::query!("SELECT id, checked, contents FROM todo_items")
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|r| TodoItem {
            id: r.id,
            checked: r.checked,
            contents: r.contents,
        })
        .collect();

    Ok(items)
}

pub async fn delete_todo(pool: &SqlitePool, id: i64) -> anyhow::Result<bool> {
    let rows_affected = sqlx::query!("DELETE FROM todo_items WHERE id = ?1", id)
        .execute(pool)
        .await?
        .rows_affected();

    Ok(rows_affected > 0)
}
