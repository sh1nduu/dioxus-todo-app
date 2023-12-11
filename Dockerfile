FROM rust:1.71-slim-bookworm

WORKDIR /workspace

ENV NODE_MAJOR 21
RUN apt-get update \
    && apt-get install -y \
        ca-certificates \
        curl \
        gnupg \
    && mkdir -p /etc/apt/keyrings \
    && curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key \
        | gpg --dearmor -o /etc/apt/keyrings/nodesource.gpg \
    && echo "deb [signed-by=/etc/apt/keyrings/nodesource.gpg] https://deb.nodesource.com/node_$NODE_MAJOR.x nodistro main" \
        | tee /etc/apt/sources.list.d/nodesource.list \
    && curl -fsSL https://apt.llvm.org/llvm-snapshot.gpg.key \
        | gpg --dearmor -o /etc/apt/keyrings/apt.llvm.org.gpg \
    && echo "deb [signed-by=/etc/apt/keyrings/apt.llvm.org.gpg] https://apt.llvm.org/bookworm/ llvm-toolchain-bookworm  main" \
        | tee /etc/apt/sources.list.d/apt.llvm.org.list \
    && apt-get update \
    && apt-get install -y \
        nodejs \
        libssl-dev \
        pkg-config \
        build-essential \
        wget \
        git \
        software-properties-common \
        lsb-release \
        clang \
        lld \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown

RUN cargo install sqlx-cli --no-default-features --features rustls,sqlite \
    && cargo install just \
    && cargo install dioxus-cli --locked
