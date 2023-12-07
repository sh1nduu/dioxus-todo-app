use dioxus_fullstack::prelude::*;
use dioxus_router::prelude::*;
use dioxus::prelude::*;

use crate::Route;

#[derive(PartialEq, Props)]
pub(crate) struct HomeProps {
}

pub(crate) fn Home(cx: Scope<HomeProps>) -> Element {
    let mut count = use_state(cx, || 0);

    let text = use_state(cx, || "...".to_string());

    cx.render(rsx! {
        Link { to: Route::Blog { id: *count.get() }, "Go to blog" }
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }

            button {
                onclick: move |_| {
                    to_owned![text];
                    async move {
                        if let Ok(data) = get_server_data().await {
                            println!("Client received: {}", data);
                            text.set(data.clone());
                            post_server_data(data).await.unwrap();
                        }
                    }
                },
                "Run server function!"
            }
            "Server said: {text}"
        }
    })
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
