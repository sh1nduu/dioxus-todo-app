#![allow(non_snake_case)]

use dioxus::{html::input_data::keyboard_types::Key, prelude::*};

#[derive(Props)]
pub struct ControlProps<'a> {
    on_toggle_all: EventHandler<'a, ()>,
    on_add: EventHandler<'a, String>,
}

pub fn Control<'a>(cx: Scope<'a, ControlProps<'a>>) -> Element<'a> {
    let draft = use_state(cx, || "".to_string());

    let handle_input = move |evt: Event<FormData>| {
        draft.set(evt.value.clone());
    };
    let handle_keydown = move |evt: Event<KeyboardData>| {
        if evt.key() != Key::Enter {
            return;
        }
        cx.props.on_add.call(draft.to_string());
        draft.set("".to_string());
    };

    render!(
        div { class: "flex justify-cente items-center border-b border-gray-300",
            label {
                r#for: "toggle-all",
                class: "w-[60px] h-[34px] text-[0] transform rotate-90 translate-x-3 translate-y-2 before:p-4 before:content-['❯'] before:text-[22px] before:text-gray-300",
                "Mark as complete"
            }
            input {
                id: "toggle-all",
                class: "w-[1px] h-[1px] border-none opacity-0",
                r#type: "checkbox",
                oninput: move |_| cx.props.on_toggle_all.call(())
            }
            input {
                class: "w-full border-none text-gray-500 text-2xl font-light outline-none p-4",
                placeholder: "What needs to be done?",
                autofocus: true,
                value: "{draft}",
                oninput: handle_input,
                onkeydown: handle_keydown
            }
        }
    )
}
