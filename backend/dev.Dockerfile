
# Use the rust image as the base image
FROM rust:latest

RUN apt-get update
RUN apt-get install nodejs npm -y

COPY /backend /app/backend
COPY /common /app/common/

WORKDIR /app/backend


RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk

# Install dependencies
RUN cargo build --release

# Run the directory with the trunk tool
CMD ["trunk", "serve", "--release", "--address", "0.0.0.0"]
