#[cfg(feature = "ssr")]
mod db;
mod domain;
mod layouts;
mod pages;

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use pages::{Blog, Home};

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}
