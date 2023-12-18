#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

#[cfg(feature = "ssr")]
use crate::server::AppModule;
use crate::{
    domain::TodoItem,
    layouts::Layout,
    views::{control::Control, list_footer::ListFooter, todo_entry::TodoEntry},
};

use super::list_footer::FilterState;

#[derive(PartialEq, Props)]
pub(crate) struct HomeProps {}

pub(crate) fn Home(cx: Scope<HomeProps>) -> Element {
    let initial_todo_items = use_server_future(cx, (), |_| async { get_todos().await })?.value();
    let Ok(todo_items) = initial_todo_items.as_deref() else {
        return cx.render(rsx! {"Failed!"});
    };
    let todo_items = use_state(cx, move || todo_items.to_owned());
    let filter_state = use_state(cx, || FilterState::All);

    let filtered_todos: Vec<_> = todo_items
        .iter()
        .filter(|item| match **filter_state {
            FilterState::All => true,
            FilterState::Active => !item.checked,
            FilterState::Completed => item.checked,
        })
        .collect();

    let handle_add_todo = move |draft: String| {
        to_owned![todo_items];
        cx.spawn(async move {
            match add_todo(draft).await {
                Ok(todo) => {
                    todo_items.make_mut().insert(todo_items.len(), todo);
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
                    Control { on_add: handle_add_todo }
                    ul { class: "",
                        filtered_todos.iter().map(|todo_item| rsx! {
                            TodoEntry {
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
