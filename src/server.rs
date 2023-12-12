use sqlx::{Sqlite, Pool};
use dioxus_fullstack::prelude::*;

use crate::db;

struct AppState {
    pool: Pool<Sqlite>,
}

pub async fn launch() -> anyhow::Result<()> {
    let pool = db::establish().await?;
    let state = AppState { pool };


    let app = axum::Router::new()
       .serve_dioxus_application("", ServeConfigBuilder::new(crate::app, ()));

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}