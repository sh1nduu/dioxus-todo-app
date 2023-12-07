use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::Route;

#[derive(PartialEq, Props)]
pub(crate) struct BlogProps {
    id: i32,
}

pub(crate) fn Blog(cx: Scope<BlogProps>) -> Element {
    render! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {cx.props.id}"
    }
}