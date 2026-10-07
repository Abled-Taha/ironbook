# Iron Book API

Backend API for Project Iron Book, written in Rust with Axum.

## Layout

- `src/` — application source (binary: `ironbook_api`)
- `http/` — HTTP layer (routes/handlers)
- `migrations/` — SQLx database migrations (PostgreSQL)
- `tests/` — integration tests
- `.sqlx/` — offline query metadata for SQLx
- `Cargo.toml` / `Cargo.lock` — Rust dependencies

## Development

Complete the root setup first (Mise + Docker, `scripts/ironbook.sh setup`).
All tasks are defined in `mise.toml`:

| Task | Description |
| --- | --- |
| `mise run setup` | Check the toolchain and apply pending DB migrations |
| `mise run dev` | Run the API dev server with auto-reload (`cargo watch`) |
| `mise run test` | Run unit tests (`cargo test`) |
| `mise run test-full` | Run all tests, including DB integration tests |
| `mise run db:up` / `mise run db:down` | Start/stop the `ironbook_db` Postgres service |
| `mise run db:migrate` | Apply pending migrations |
| `mise run db:reset` | Reset the database and re-apply migrations |

Copy `.env.example` to `.env` and adjust the values before running.
