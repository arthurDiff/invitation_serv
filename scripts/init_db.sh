#!/bin/bash
# set print commands
set -x
# exit if any fails
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
echo >&2 "Error: psql is not installed."
exit 1
fi

# setting db config
DB_USER="${POSTGRES_USER:=postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB=invite}"
DB_PORT="${POSGRES_PORT:=5431}"
DB_HOST="${POSTGRES_HOST:=localhost}"

#docker
if [ $RUN_DOCKER  = true ]
then
docker run\
    --name invite \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}":5432 \
    -d \
    postgres \
    postgres -N 1000
fi

# poll until container ready
export PGPASSWORD="${DB_PASSWORD}"
until psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c "\q"; do
    >&2 echo "Postgres still unavailable"
    sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT}! - Running Migrations now"

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}

sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"