# Setup
## env file
create a .env file and change these default values 
```
# PostgreSQL settings
POSTGRES_USER=postgres
POSTGRES_PASSWORD=postgres
POSTGRES_DB=financedb
POSTGRES_HOST=localhost
POSTGRES_PORT=6500

DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DB}

# pgAdmin settings
PGADMIN_DEFAULT_EMAIL=example@email.com
PGADMIN_DEFAULT_PASSWORD=mysecretpassword
```
## sqlx database setup
Install the sqlx-cli [here](https://crates.io/crates/sqlx-cli)

start the server and database server with `docker compose up -d`

create the database
`sqlx database create`

run migrations
`sqlx migrate run`
