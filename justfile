dev:
    dx build --features web --release
    cargo run --features ssr

fmt:
    dx fmt