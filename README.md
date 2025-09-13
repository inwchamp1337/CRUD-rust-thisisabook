# CRUD Rust - thisisabook

A small CRUD example web application written in Rust. This project demonstrates a simple REST API backed by a database, with structured code in `src/` and a `Makefile`/`docker-compose.yml` for convenience.

**Project layout**
- `Cargo.toml`: Cargo manifest
- `src/`: application source code (`main.rs`, `routes.rs`, `handlers.rs`, `models.rs`, `db.rs`, `config.rs`)
- `scripts/seed.py`: helper to seed the database
- `docker-compose.yml`: optional dockerized services
- `Makefile`: convenience tasks

**Prerequisites**
- Rust (stable) and Cargo
- Python 3 (only if you intend to run `scripts/seed.py`)
- Docker & Docker Compose (optional)

Quick checks:

```bash
rustc --version
cargo --version
python --version   # optional (for seed script)
docker --version   # optional
```

Getting started (local)

1. Build the project:

```bash
cargo build
```

2. Run the app:

```bash
cargo run
```

By default the server should start and listen on the address printed in the console (see `src/main.rs` for the exact listen address).

Seeding the database

If you need sample data, run the seed script:

```bash
python scripts/seed.py
```

Docker

If you prefer to run services using Docker Compose:

```bash
docker compose up --build
```

API Overview

This repository implements standard CRUD routes. Check `src/routes.rs` and `src/handlers.rs` for the exact paths and handler implementations.

- GET    /items        — list items
- GET    /items/:id    — get single item
- POST   /items        — create item
- PUT    /items/:id    — update item
- DELETE /items/:id    — delete item

Adjust the path names above if your project uses different resource names (see `src/routes.rs`).

Notes about `Cargo.lock`

- This repository currently includes `Cargo.lock` in `.gitignore`.
- If this project is an application (binary), it is recommended to commit `Cargo.lock` to ensure reproducible builds across environments. If you want that, remove `Cargo.lock` from `.gitignore` and commit the file.

Contributing

Feel free to open issues or PRs. For development workflow, use branches and run `cargo fmt` and `cargo clippy` before opening a PR.

License

This project does not include a license file. Add one if you intend to make it open source.
