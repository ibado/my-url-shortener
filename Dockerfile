FROM rust:latest as builder

WORKDIR /usr/src/my-url-shortener
COPY src src
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml
COPY migrations migrations
COPY .sqlx .sqlx

ENV SQLX_OFFLINE true

RUN rustup override set nightly
RUN cargo build --release

FROM gcr.io/distroless/cc-debian10

COPY --from=builder /usr/src/my-url-shortener/target/release/my-url-shortener /usr/local/bin/my-url-shortener
COPY static /usr/local/bin/static

WORKDIR /usr/local/bin
CMD ["my-url-shortener"]
