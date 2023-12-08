dev:
    dx build --features web
    cargo run --features ssr

dev-ui:
    dx build --features web
    dx serve --features ssr --hot-reload --platform desktop

fmt:
    dx fmt