mod domain;
mod layouts;
mod pages;

#[cfg(feature = "ssr")]
mod db;
#[cfg(feature = "ssr")]
mod server;

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use log::LevelFilter;

use crate::pages::{Blog, Home};

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

#[component]
pub fn app(cx: Scope) -> Element {
    cx.render(rsx! { Router::<Route> {} })
}

pub fn launch() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    #[cfg(feature = "web")]
    {
        dioxus_web::launch_cfg(app, dioxus_web::Config::new().hydrate(true));
    }
    #[cfg(feature = "ssr")]
    {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                let _ = server::launch().await;
            });
    }
}
