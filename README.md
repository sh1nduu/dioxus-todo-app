# Development

1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the tailwind css cli: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch
```
Launch the Dioxus Fullstack app:

```bash
dx build --features web --release
cargo run --features ssr --release
```

## Setup Database

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

## For Mac users (WIP)

Apple Clang doesn't support `wasm32-unknown-unknown`, so you need to install llvm.org Clang instead.

```
brew install llvm
```