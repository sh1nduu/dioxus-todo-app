#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props)]
pub(crate) struct LayoutProps<'a> {
    children: Element<'a>,
}

pub(crate) fn Layout<'a>(cx: Scope<'a, LayoutProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        div { class: "bg-neutral-100 w-full h-screen flex justify-center", &cx.props.children }
    })
}
