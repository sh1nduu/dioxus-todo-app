use std::sync::Arc;
use async_trait::async_trait;
use axum::http;
use std::convert::Infallible;

use axum::Extension;
use dioxus_fullstack::prelude::*;

use crate::{db::{self, TodoRepositoryImpl}, domain::TodoRepository};

#[derive(Clone)]
pub struct AppModule {
    pub todo_repository: Arc<dyn TodoRepository>,
}

#[async_trait]
impl<S> axum::extract::FromRequestParts<S> for AppModule {
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<AppModule>()
            .cloned()
            .ok_or_else(|| unreachable!())
    }
}

pub async fn launch() -> anyhow::Result<()> {
    let pool = db::establish().await?;
    let app_module = AppModule {
        todo_repository: Arc::new(TodoRepositoryImpl::new(&pool)),
    };

    let app = axum::Router::new()
        .serve_dioxus_application("", ServeConfigBuilder::new(crate::app, ()))
        .layer(Extension(app_module));

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}