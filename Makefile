dev-server:
	docker-compose -f compose/docker-compose.dev-database.yml up -d
	RUST_LOG=debug cargo watch -x run

setup:
	docker-compose -f compose/docker-compose.dev-database.yml up -d
	cargo install diesel_cli
	diesel migration run