#![allow(non_snake_case)]

use crate::domain::TodoItem;
use dioxus::prelude::*;

#[derive(Props)]
pub struct TodoentryProps<'a> {
    pub todo_item: &'a TodoItem,
    pub on_toggle: EventHandler<'a, i64>,
    pub on_delete: EventHandler<'a, i64>,
}

pub fn TodoEntry<'a>(cx: Scope<'a, TodoentryProps<'a>>) -> Element<'a> {
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
