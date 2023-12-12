use async_trait::async_trait;
use axum::{extract::FromRef, http};
use dioxus_fullstack::prelude::{Axum, DioxusServerContext, FromServerContext};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::{convert::Infallible, env};

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
}

#[async_trait]
impl<S> axum::extract::FromRequestParts<S> for TodoRepositoryImpl {
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<TodoRepositoryImpl>()
            .cloned()
            .ok_or_else(|| unreachable!())
    }
}

pub async fn establish() -> anyhow::Result<SqlitePool> {
    Ok(SqlitePool::connect(&env::var("DATABASE_URL")?).await?)
}

pub async fn add_todo(pool: &SqlitePool, contents: &'_ str) -> anyhow::Result<TodoItem> {
    let mut conn = pool.acquire().await?;
    let id = sqlx::query!(
        "INSERT INTO todo_items ( contents ) VALUES ( ?1 )",
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
        .last_insert_rowid();

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
