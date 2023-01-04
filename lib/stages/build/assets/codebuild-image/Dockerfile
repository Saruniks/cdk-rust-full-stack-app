FROM rust:1.64

RUN apt-get update && \
    apt-get install -y nodejs && \
    apt-get install -y npm && \
    apt-get install -y openjdk-11-jdk && \
    npm install -g tailwindcss && \
    cargo install --locked trunk && \
    rustup target add wasm32-unknown-unknown && \
    cargo install --version 0.2.64 -f wasm-bindgen-cli;
