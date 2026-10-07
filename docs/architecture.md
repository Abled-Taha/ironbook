# Architecture

This document describes the system architecture of Project Iron Book: how the
parts fit together, how data flows, and the major design decisions behind the
layout. It complements the [README](../README.md) (setup and onboarding) and
the [Security](security.md) document (authentication and threat model).

## System architecture

Iron Book is a **polyglot monorepo**: every part of the product lives in one
repository, each in its own tech stack, all managed through one toolchain
(`mise` tasks + `scripts/ironbook.sh`).

| Part | Location | Tech | Role |
| ---- | -------- | ---- | ---- |
| API | `apps/api/` | Rust (Axum + tonic) | Backend: REST + gRPC, business logic, persistence |
| Web client | `apps/web/` | Python (Django) | Browser client |
| Android app | `apps/android/` | Kotlin | Mobile client |
| Desktop app | `apps/desktop/` | C# (Avalonia) | Linux/Windows client |
| Project website | `apps/home/` | Next.js | Public site and docs |
| Database | — | PostgreSQL | Primary data store |
| Cache | — | Redis | Planned |

The API is the only component that talks to the database. Every client (web,
Android, desktop) goes through the API; no client holds database credentials.

## Component relationships

```
                        ┌─────────────────┐
                        │  apps/home      │  Next.js project website / docs
                        │  (public site)  │
                        └─────────────────┘
        ┌───────────────────┬───────────────────┐
        │                   │                   │
┌───────────────┐   ┌───────────────┐   ┌───────────────┐
│ apps/web      │   │ apps/android  │   │ apps/desktop  │
│ Django        │   │ Kotlin        │   │ C# (Avalonia) │
└───────┬───────┘   └───────┬───────┘   └───────┬───────┘
        │                   │                   │
        └───────────────────┼───────────────────┘
                            │  REST + gRPC
                            ▼
                   ┌─────────────────┐
                   │ apps/api        │
                   │ Rust: Axum      │── views (REST handlers)
                   │       tonic     │── grpc (gRPC handlers)
                   └────────┬────────┘
                            │  sqlx
                            ▼
                   ┌─────────────────┐
                   │ PostgreSQL      │── sqlx migrations
                   └─────────────────┘
```

Shared API contracts live in `contracts/proto/` (`auth.proto`, `users.proto`,
`system.proto`) and are compiled into the Rust binary at build time
(`build.rs` + `tonic-prost-build`). The same contract files are the source of
truth for any client implementation, so REST and gRPC stay consistent.

Infrastructure definitions live in `infra/docker/compose/` (`dev.yaml`,
`prod.yaml`).

## Data flow

A typical request travels through the API in layers:

1. **Transport** — `src/views/` (REST, Axum handlers) or `src/grpc/`
   (gRPC, tonic services) receives the request. HTTP test fixtures for each
   surface live in `apps/api/http/` (`auth.http`, `users.http`, `system.http`).
2. **Business logic** — `src/services/` validates input, verifies credentials
   and API tokens, and issues session tokens.
3. **Persistence** — `src/db/` executes parameterized queries through `sqlx`
   against PostgreSQL, using a shared `PgPool` carried in `AppState`.
4. **Errors** — a central `src/errors.rs` maps failures to consistent
   API error responses.

Example — registration (`src/services/auth.rs`):
client-supplied API token is verified (`verify_api_token`) → username
uniqueness is checked → password is hashed with **Argon2** → user row is
created → an opaque session token is issued and only its **SHA-256 hash** is
stored in the `sessions` table with an expiry timestamp.

Configuration flows in through environment variables (`dotenvy`), documented
in `apps/api/.env.example` (`DATABASE_URL`, `ALLOWED_ORIGINS`,
`ALLOW_ALL_ORIGINS`, …). Logging is structured JSON via `tracing`
(`src/log/`), with auth-relevant events (registrations, invalid tokens)
emitted as `info!`/`warn!`.

## Database architecture

PostgreSQL is the single source of truth. Schema is versioned with
`sqlx`-managed migrations in `apps/api/migrations/`:

| Migration | Contents |
| --------- | -------- |
| `0001_initial.sql` | `users` (id, username, email, Argon2 `password_hash`), `sessions` (user FK with `ON DELETE CASCADE`, `token_hash`, `active`, `expires_at`) |
| `0002_add_clients_table.sql` | `clients` (name, owner email, unique `api_token`) |

Design notes:

- Passwords are never stored; only Argon2 hashes.
- Session tokens are never stored in cleartext; only SHA-256 hashes, so a
  database read alone cannot replay a session.
- Sessions carry `active` + `expires_at`, which makes server-side revocation
  and expiry possible without client cooperation.
- Deleting a user cascades to their sessions.
- An offline query cache (`.sqlx/`) keeps compile-time query checking working
  without a live database.

## Client/API architecture

- The API binary exposes **two surfaces from one process**: REST (Axum) and
  gRPC (tonic). Both are thin adapters over the same `services` layer, so
  behavior is identical regardless of transport.
- Clients authenticate per user (session tokens) and, for privileged
  operations such as registration, per client (API tokens from the `clients`
  table). See [Security](security.md) for the full model.
- CORS is configurable (`ALLOWED_ORIGINS`, `ALLOW_ALL_ORIGINS`) rather than
  open by default.

## Major design decisions

1. **Polyglot monorepo over micro-repos.** One repo, one setup script
   (`scripts/ironbook.sh setup`), per-app `mise` tasks runnable from the
   repository root (`mise run //apps/api/test`). The onboarding promise is
   three steps, two of which are prerequisites.
2. **Proto-first contracts.** `contracts/proto/` is compiled into the server
   and shared with clients, so the API surface is defined once.
3. **One binary, two transports.** REST for browser/tooling convenience,
   gRPC for efficient client communication — same services underneath, no
   duplicated logic.
4. **Stateful sessions, hashed tokens.** Sessions live in the database with
   hashes, expiry, and a revocation flag, trading a small lookup cost for
   server-side control over every issued token.
5. **Secrets in the environment, never in the repo.** `.env.example` documents
   variable names only; real values are supplied at deploy time.
6. **Reproducibility.** Docker Compose definitions for dev and prod, plus an
   offline sqlx cache, keep builds working on any machine.
