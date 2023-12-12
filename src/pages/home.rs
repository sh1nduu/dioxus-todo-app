#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

use crate::{domain::TodoItem, layouts::Layout};

#[derive(PartialEq, Props)]
pub(crate) struct HomeProps {}

pub(crate) fn Home(cx: Scope<HomeProps>) -> Element {
    let todo_items = use_server_future(cx, (), |_| async { get_todos().await })?.value();

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
                            autofocus: true
                        }
                    }
                    ul { class: "",
                        if let Ok(todo_items) = todo_items.as_deref() {
                            rsx! {
                                for todo_item in todo_items {
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
    Ok(vec![
        TodoItem::new(1, "foobar"),
        TodoItem::new(2, "buzz"),
        TodoItem::new(3, "hoge"),
    ])
}

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    println!("Server received: {}", data);

    Ok(())
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}

fn server_err(e: anyhow::Error) -> ServerFnError {
    ServerFnError::ServerError(format!("{}", e))
}
