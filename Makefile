# Makefile

# COUNT ?= 10

dev:
	cargo run

watch:
	cargo watch -x run

build:
	cargo build --release

test:
	cargo test

clean:
	cargo clean

docker-up:
	docker-compose up -d


