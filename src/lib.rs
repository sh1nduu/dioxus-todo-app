mod layouts;
mod pages;

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use pages::{Blog, Home};

pub fn hello() -> String {
    "Hello!".to_string()
}

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}
