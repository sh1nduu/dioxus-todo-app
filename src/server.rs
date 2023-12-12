use axum::Extension;
use dioxus_fullstack::prelude::*;

use crate::db::{self, TodoRepositoryImpl};

#[derive(Clone)]
pub struct AppState {
    pub todo_repo: TodoRepositoryImpl,
}

pub async fn launch() -> anyhow::Result<()> {
    let pool = db::establish().await?;

    let app = axum::Router::new()
        .serve_dioxus_application("", ServeConfigBuilder::new(crate::app, ()))
        .layer(Extension(TodoRepositoryImpl::new(&pool)));

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
