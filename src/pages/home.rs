#![allow(non_snake_case)]

use dioxus::{html::input_data::keyboard_types::Key, prelude::*};
use dioxus_fullstack::prelude::*;

#[cfg(feature = "ssr")]
use crate::server::AppModule;
use crate::{domain::TodoItem, layouts::Layout};

#[derive(PartialEq, Clone, Copy)]
enum FilterState {
    All,
    Active,
    Completed,
}

#[derive(PartialEq, Props)]
pub(crate) struct HomeProps {}

pub(crate) fn Home(cx: Scope<HomeProps>) -> Element {
    let initial_todo_items = use_server_future(cx, (), |_| async { get_todos().await })?.value();
    let Ok(todo_items) = initial_todo_items.as_deref() else {
        return cx.render(rsx! {"Failed!"});
    };
    let todo_items = use_state(cx, move || todo_items.to_owned());
    let draft = use_state(cx, || "".to_string());
    let filter_state = use_state(cx, || FilterState::All);

    let filtered_todos: Vec<_> = todo_items
        .iter()
        .filter(|item| match **filter_state {
            FilterState::All => true,
            FilterState::Active => !item.checked,
            FilterState::Completed => item.checked,
        })
        .collect();

    let handle_header_input = move |evt: Event<FormData>| {
        draft.set(evt.value.clone());
    };
    let handle_header_keydown = move |evt: Event<KeyboardData>| {
        if evt.key() != Key::Enter {
            return;
        }
        to_owned![draft, todo_items];
        cx.spawn(async move {
            match add_todo(draft.to_string()).await {
                Ok(todo) => {
                    todo_items.make_mut().insert(todo_items.len(), todo);
                    draft.set("".to_string());
                }
                Err(e) => log::error!("Failed! {}", e),
            }
        })
    };
    let handle_delete_todo = move |id| {
        to_owned![todo_items];
        cx.spawn(async move {
            match delete_todo(id).await {
                Ok(true) => todo_items.make_mut().retain(|item| item.id != id),
                Ok(false) => log::info!("Failed to delete item (id: {})", id),
                Err(e) => log::error!("Failed! {}", e),
            }
        })
    };
    let handle_toggle_todo = move |id| {
        to_owned![todo_items];
        cx.spawn(async move {
            match toggle_todo(id).await {
                Ok(Some(new_todo_item)) => {
                    let Some(idx) = todo_items.iter().position(|item| item.id == id) else {
                        log::debug!("The item already removed (id: {})", id);
                        return;
                    };
                    let _ = std::mem::replace(&mut todo_items.make_mut()[idx], new_todo_item);
                }
                Ok(None) => log::error!("The item is missing! (id: {})", id),
                Err(e) => log::error!("Failed! {}", e),
            }
        })
    };
    let handle_clear_completed = move |_| {
        let checked_ids: Vec<_> = todo_items
            .iter()
            .filter(|item| item.checked)
            .map(|item| item.id)
            .collect();
        to_owned![todo_items];
        cx.spawn(async move {
            match clear_completed_todos(checked_ids).await {
                Ok(true) => todo_items.make_mut().retain(|item| !item.checked),
                Ok(false) => log::warn!("No items are removed"),
                Err(e) => log::error!("Failed! {}", e),
            }
        })
    };
    let handle_select_filter = move |selected: FilterState| filter_state.set(selected);
    let active_items_count = todo_items.iter().filter(|item| !item.checked).count();
    let show_clear_completed = active_items_count != todo_items.len();

    cx.render(rsx! {
        Layout { 
            section { class: "w-2/3",
                header { class: "",
                    h1 { class: "mb-4 text-8xl text-gray-300 font-thin text-center drop-shadow-sm",
                        "todos"
                    }
                }
                section { class: "bg-white drop-shadow",
                    div { class: "flex justify-cente items-center border-b border-gray-300",
                        label {
                            r#for: "toggle-all",
                            class: "w-[60px] h-[34px] text-[0] transform rotate-90 translate-x-3 translate-y-2 before:p-4 before:content-['❯'] before:text-[22px] before:text-gray-300",
                            "Mark as complete"
                        }
                        input {
                            id: "toggle-all",
                            class: "w-[1px] h-[1px] border-none opacity-0",
                            r#type: "checkbox"
                        }
                        input {
                            class: "w-full border-none text-gray-500 text-2xl font-light outline-none p-4",
                            placeholder: "What needs to be done?",
                            autofocus: true,
                            value: "{draft}",
                            oninput: handle_header_input,
                            onkeydown: handle_header_keydown
                        }
                    }
                    ul { class: "",
                        filtered_todos.iter().map(|todo_item| rsx! {
                            TodoRow {
                                key: "{todo_item.id}",
                                todo_item: todo_item,
                                on_toggle: handle_toggle_todo,
                                on_delete: handle_delete_todo,
                            }
                        })
                    }
                    ListFooter {
                        active_items_count: active_items_count,
                        selected_filter: filter_state,
                        show_clear_completed: show_clear_completed,
                        on_select_filter: handle_select_filter,
                        on_clear_completed: handle_clear_completed
                    }
                }
            }
        }
    })
}

#[derive(Props)]
struct TodoRowProps<'a> {
    todo_item: &'a TodoItem,
    on_toggle: EventHandler<'a, i64>,
    on_delete: EventHandler<'a, i64>,
}

fn TodoRow<'a>(cx: Scope<'a, TodoRowProps<'a>>) -> Element<'a> {
    let todo_item = cx.props.todo_item;
    let todo_checked_bg_style = if todo_item.checked {
        "bg-[url('checked.svg')]"
    } else {
        "bg-[url('circle.svg')]"
    };
    let todo_checked_label_style = if todo_item.checked {
        "line-through decoration-slate-500"
    } else {
        ""
    };

    render!(
        li { class: "border-b border-gray-300 group",
            div { class: "flex text-gray-500 text-2xl font-light bg-tranparent items-center",
                input {
                    class: "h-[40px] w-[60px] ml-4 appearance-none border-none outline-none bg-no-repeat bg-[center_left] {todo_checked_bg_style}",
                    r#type: "checkbox",
                    onclick: move |_| cx.props.on_toggle.call(todo_item.id)
                }
                label { class: "py-4 pl-1 w-full {todo_checked_label_style}", "{todo_item.contents}" }
                button {
                    class: "w-[40px] h-[40px] mr-4 group-hover:bg-[url('cross.svg')] bg-no-repeat bg-center",
                    onclick: move |_| cx.props.on_delete.call(todo_item.id)
                }
            }
        }
    )
}

#[derive(Props)]
struct ListFooterProps<'a> {
    active_items_count: usize,
    selected_filter: &'a FilterState,
    show_clear_completed: bool,
    on_select_filter: EventHandler<'a, FilterState>,
    on_clear_completed: EventHandler<'a, ()>,
}

fn ListFooter<'a>(cx: Scope<'a, ListFooterProps<'a>>) -> Element<'a> {
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

#[server]
async fn get_todos() -> Result<Vec<TodoItem>, ServerFnError> {
    let todo_repository = extract::<AppModule, _>().await?.todo_repository;
    todo_repository.list().await.map_err(server_err)
}

#[server]
async fn add_todo<'a>(contents: String) -> Result<TodoItem, ServerFnError> {
    let todo_repository = extract::<AppModule, _>().await?.todo_repository;
    todo_repository.add(&contents).await.map_err(server_err)
}

#[server]
async fn delete_todo(id: i64) -> Result<bool, ServerFnError> {
    let todo_repository = extract::<AppModule, _>().await?.todo_repository;
    todo_repository.delete(id).await.map_err(server_err)
}

#[server]
async fn toggle_todo(id: i64) -> Result<Option<TodoItem>, ServerFnError> {
    let todo_repository = extract::<AppModule, _>().await?.todo_repository;
    todo_repository.toggle(id).await.map_err(server_err)
}
#[server]
async fn clear_completed_todos(ids: Vec<i64>) -> Result<bool, ServerFnError> {
    let todo_repository = extract::<AppModule, _>().await?.todo_repository;
    todo_repository
        .clear_completed(&ids)
        .await
        .map_err(server_err)
}

fn server_err(e: anyhow::Error) -> ServerFnError {
    ServerFnError::ServerError(format!("{}", e))
}
