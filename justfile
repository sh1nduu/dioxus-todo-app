dev:
    npx tailwindcss -i ./input.css -o ./public/tailwind.css
    dx build --features web --release
    cargo run --features ssr --release

dev-ui:
    dx build --features web
    dx serve --features ssr --hot-reload --platform desktop

fmt:
    cargo fmt
    dx fmt

prepare:
    cargo sqlx prepare -- --all-features