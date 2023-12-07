#![allow(non_snake_case)]
#![allow(unused)]
use dioxus_fullstack::prelude::*;
use dioxus_router::prelude::*;
use dioxus::prelude::*;
use pages::{Blog, Home};

mod pages;

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

