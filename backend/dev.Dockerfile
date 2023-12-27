
# Use the rust image as the base image
FROM rust:latest

RUN apt-get update
RUN apt-get install nodejs npm -y

COPY /backend /app/backend
COPY /common /app/common/

WORKDIR /app/backend


RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-watch

# install sqlx manager
RUN cargo install sqlx-cli --no-default-features --features postgres
