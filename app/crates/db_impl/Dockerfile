FROM rust:nightly as builder

RUN apt-get update && apt-get install -y libpq-dev

RUN USER=root cargo new --bin db-sync
WORKDIR /db-sync
COPY db-sync/Cargo.toml ./Cargo.toml
RUN cargo build

RUN rm src/*.rs
COPY db-sync/src ./src
COPY ./migrations ./migrations
COPY ./diesel.toml ./diesel.toml
RUN rm ./target/debug/deps/db_sync*
RUN cargo build

FROM buildpack-deps:stretch

COPY --from=builder /db-sync/target/debug/db-sync  /app/
ENV RUST_LOG=debug
EXPOSE 8000
ENTRYPOINT ["/app/db-sync"]
