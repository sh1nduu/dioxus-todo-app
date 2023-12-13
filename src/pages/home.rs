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
        to_owned![draft, todo_items];
        cx.spawn(async move {
            if evt.key() != Key::Enter {
                return;
            }
            match add_todo(draft.to_string()).await {
                Ok(todo) => {
                    todo_items.make_mut().insert(todo_items.len(), todo);
                    draft.set("".to_string());
                }
                Err(e) => log::error!("Failed! {}", e),
            }
        })
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
                                        class: "h-[40px] w-[60px] ml-4 appearance-none border-none outline-none bg-no-repeat bg-[url('circle.svg')] bg-[center_left]",
                                        r#type: "checkbox"
                                    }
                                    label { class: "py-4 pl-1 w-full", "{todo_item.contents}" }
                                    button { class: "w-[40px] h-[40px] mr-4 group-hover:bg-[url('cross.svg')] bg-no-repeat bg-center" }
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

fn server_err(e: anyhow::Error) -> ServerFnError {
    ServerFnError::ServerError(format!("{}", e))
}
