dev:
    dx build --features web --release
    cargo run --features ssr --release

dev-ui:
    dx build --features web
    dx serve --features ssr --hot-reload --platform desktop

fmt:
    dx fmt