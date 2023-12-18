# TodoMVC (using Dioxus Fullstack)

This project is a TodoMVC (Todo List) application implemented with Dioxus Fullstack, a Rust-based SPA (Single Page Application) framework. 

# Development

It is recommended to use devcontainer.

1. Declare the database URL

    ```bash
    export DATABASE_URL="sqlite:data/todos.db"
    ```

2. Create the database.

    ```bash
    sqlx db create
    ```

3. Run sql migrations

    ```bash
    sqlx migrate run
    ```

4. Build tailwindcss

    ```bash
    npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch
    ```

5. Launch the Dioxus Fullstack app:

    ```bash
    dx build --features web --release
    cargo run --features ssr --release
    ```
