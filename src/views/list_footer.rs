#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(PartialEq, Clone, Copy)]
pub enum FilterState {
    All,
    Active,
    Completed,
}

#[derive(Props)]
pub struct ListFooterProps<'a> {
    pub active_items_count: usize,
    pub selected_filter: &'a FilterState,
    pub show_clear_completed: bool,
    pub on_select_filter: EventHandler<'a, FilterState>,
    pub on_clear_completed: EventHandler<'a, ()>,
}

pub fn ListFooter<'a>(cx: Scope<'a, ListFooterProps<'a>>) -> Element<'a> {
    let selected = |filter: &FilterState| {
        if filter == cx.props.selected_filter {
            "border border-gray-300"
        } else {
            ""
        }
    };

    cx.render(rsx! {
        footer { class: "flex justify-between items-center bg-white drop-shadow text-gray-500 text-l h-12",
            div { class: "pl-4 ml-4",
                strong { "{cx.props.active_items_count}" }
                " items left"
            }
            ul { class: "flex justify-evenly",
                [
                    (FilterState::All, "All", "/#"),
                    (FilterState::Active, "Active", "/#active"),
                    (FilterState::Completed, "Completed", "/#completed"),
                ].iter().map(|(state, text, url)| rsx! {
                    li {
                        class: selected(state),
                        a {
                            href: "{url}",
                            class: "p-2",
                            onclick: |_| cx.props.on_select_filter.call(state.clone()),
                            "{text}"
                        }
                    }
                })
            }
            div { class: "mr-4 flex flex-row-reverse w-[124px]",
                if cx.props.show_clear_completed {
                    rsx! {
                        button {
                            class: "decoration-slate-500 hover:underline",
                            onclick: |_| cx.props.on_clear_completed.call(()),
                            "Clear Completed"
                        }
                    }
                }
            }
        }
    })
}
