#!/bin/bash
echo 'waiting for postgres'
# set -e
until echo > /dev/tcp/postgres/5432; do sleep 1; done
echo 'attempting database creation'
sqlx database create
echo 'attempting migrations'
sqlx migrate run
cargo build --release
cargo watch -- cargo run