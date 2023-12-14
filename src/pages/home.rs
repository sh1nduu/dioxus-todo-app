#![allow(non_snake_case)]

use dioxus::{html::input_data::keyboard_types::Key, prelude::*};
use dioxus_fullstack::prelude::*;

#[cfg(feature = "ssr")]
use crate::server::AppModule;
use crate::{domain::TodoItem, layouts::Layout};

#[derive(PartialEq, Props)]
pub(crate) struct HomeProps {}

pub(crate) fn Home(cx: Scope<HomeProps>) -> Element {
    let initial_todo_items = use_server_future(cx, (), |_| async { get_todos().await })?.value();
    let Ok(todo_items) = initial_todo_items.as_deref() else {
        return cx.render(rsx! {"Failed!"});
    };
    let todo_items = use_state(cx, move || todo_items.to_owned());
    let draft = use_state(cx, || "".to_string());

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
                    let Some(idx) =todo_items.iter().position(|item| item.id == id) else {
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
    let todo_checked_bg_style = |item: &TodoItem| {
        if item.checked {
            "bg-[url('checked.svg')]"
        } else {
            "bg-[url('circle.svg')]"
        }
    };
    let todo_checked_label_style = |item: &TodoItem| {
        if item.checked {
            "line-through decoration-slate-500"
        } else {
            ""
        }
    };

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
                        for todo_item in todo_items.iter() {
                            li { class: "border-b border-gray-300 group",
                                div { class: "flex text-gray-500 text-2xl font-light bg-tranparent items-center",
                                    input {
                                        class: "h-[40px] w-[60px] ml-4 appearance-none border-none outline-none bg-no-repeat {todo_checked_bg_style(todo_item)} bg-[center_left]",
                                        r#type: "checkbox",
                                        onclick: move |_| { handle_toggle_todo(todo_item.id) }
                                    }
                                    label { class: "py-4 pl-1 w-full {todo_checked_label_style(todo_item)}",
                                        "{todo_item.contents}"
                                    }
                                    button {
                                        class: "w-[40px] h-[40px] mr-4 group-hover:bg-[url('cross.svg')] bg-no-repeat bg-center",
                                        onclick: move |_| { handle_delete_todo(todo_item.id) }
                                    }
                                }
                            }
                        }
                    }
                }
                footer { class: "flex items-center bg-white drop-shadow text-gray-500 text-l h-12",
                    div { class: "w-1/3 pl-4 ml-4",
                        strong { "3" }
                        " items left"
                    }
                    ul { class: "w-1/3 flex justify-evenly",
                        li { class: "border border-gray-300", a { href: "#/", class: "p-2", "All" } }
                        li { a { href: "#/active", class: "p-2", "Active" } }
                        li { a { href: "#/completed", class: "p-2", "Completed" } }
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

fn server_err(e: anyhow::Error) -> ServerFnError {
    ServerFnError::ServerError(format!("{}", e))
}
