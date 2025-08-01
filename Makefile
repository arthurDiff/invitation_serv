
run:
	cargo run | bunyan
check:
	cargo check
test:
	TEST_LOG=true cargo test | bunyan
coverate:
	cargo tarpaulin --ignore-tests
lint:
	cargo clippy -- -D warnings
format:
	cargo format
audit:
	cargo audit
unused-dep:
	cargo +nightly udeps

d ?= false #docker build will be skipped as default
init-db:
	RUN_DOCKER=$(d) ./scripts/init_db.sh
run-docker:
	docker compose -f docker-compose.local.yml up -d \
	&& ./scripts/init_db.sh
down-docker:
	docker compose -f docker-compose.local.yml down 
run-app:
	docker compose -f docker-compose.yml up
down-app:
	docker compose -f docker-compose.yml down --volumes
db:
	psql -h localhost -p 5432 -U postgres -d postgres
db-add:
	export DATABASE_URL=postgres://postgres:password@localhost:5432/invite \
	sqlx migrate add $(name)
db-migrate:
	export DATABASE_URL=postgres://postgres:password@localhost:5432/invite \
	sqlx migrate run



