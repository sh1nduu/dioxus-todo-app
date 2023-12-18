use async_trait::async_trait;
use sqlx::{prelude::FromRow, SqlitePool};
use std::{env, fmt::format};

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
    async fn toggle(&self, id: i64) -> anyhow::Result<Option<TodoItem>> {
        toggle_todo(&self.pool, id).await
    }
    async fn toggle_all(&self, checked: bool) -> anyhow::Result<bool> {
        toggle_all(&self.pool, checked).await
    }
    async fn clear_completed(&self, ids: &[i64]) -> anyhow::Result<bool> {
        clear_completed(&self.pool, ids).await
    }
}

pub async fn establish() -> anyhow::Result<SqlitePool> {
    Ok(SqlitePool::connect(&env::var("DATABASE_URL")?).await?)
}

#[derive(Debug, FromRow)]
struct SqlTodo {
    id: i64,
    checked: bool,
    contents: String,
}

impl From<SqlTodo> for TodoItem {
    fn from(value: SqlTodo) -> Self {
        TodoItem {
            id: value.id,
            checked: value.checked,
            contents: value.contents,
        }
    }
}

#[derive(Debug, FromRow)]
struct OptionalSqlTodo {
    id: Option<i64>,
    checked: Option<bool>,
    contents: Option<String>,
}

impl From<OptionalSqlTodo> for Option<TodoItem> {
    fn from(value: OptionalSqlTodo) -> Self {
        let OptionalSqlTodo {
            id: Some(id),
            checked: Some(checked),
            contents: Some(contents),
        } = value else { return None };
        Some(TodoItem {
            id,
            checked,
            contents,
        })
    }
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
    let items = sqlx::query_as!(SqlTodo, "SELECT id, checked, contents FROM todo_items")
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(Into::into)
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

pub async fn toggle_todo(pool: &SqlitePool, id: i64) -> anyhow::Result<Option<TodoItem>> {
    let updated = sqlx::query_as!(OptionalSqlTodo, "UPDATE todo_items SET checked = (1 - checked) WHERE id = ?1 RETURNING id, checked, contents", id)
        .fetch_one(pool)
        .await?
        .into();

    Ok(updated)
}

pub async fn toggle_all(pool: &SqlitePool, checked: bool) -> anyhow::Result<bool> {
    let updated = sqlx::query_as!(
        OptionalSqlTodo,
        "UPDATE todo_items SET checked = ?1",
        checked
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(updated > 0)
}

pub async fn clear_completed(pool: &SqlitePool, ids: &[i64]) -> anyhow::Result<bool> {
    let params = ids
        .iter()
        .enumerate()
        .map(|(i, _)| format!("?{}", i + 1))
        .collect::<Vec<_>>()
        .join(", ");
    let sql = format!("DELETE FROM todo_items WHERE id IN ( {} )", params);
    let mut query = sqlx::query(&sql);
    for id in ids {
        query = query.bind(id);
    }
    let rows_affected = query.execute(pool).await?.rows_affected();

    Ok(rows_affected > 0)
}
