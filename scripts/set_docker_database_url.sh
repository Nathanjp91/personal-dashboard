#!/bin/bash
export POSTGRES_HOST=postgres # This is the name of the service in docker-compose.yml
export DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DB}
