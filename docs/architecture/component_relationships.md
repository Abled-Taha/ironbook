# Component relationships

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

## Clients

The three clients — [web](https://github.com/ironbook-labs/ironbook/blob/main/apps/web/README.md)
(Django), [Android](https://github.com/ironbook-labs/ironbook/blob/main/apps/android/README.md)
(Kotlin), [desktop](https://github.com/ironbook-labs/ironbook/blob/main/apps/desktop/README.md)
(C# with Avalonia) — all go through the API. No client holds database
credentials; the API is the only component that talks to the database.

## API and database

[apps/api](https://github.com/ironbook-labs/ironbook/blob/main/apps/api/README.md)
(Rust: Axum + tonic) exposes REST and gRPC surfaces from one binary and
persists through `sqlx` against PostgreSQL. See
[database architecture](https://github.com/ironbook-labs/ironbook/blob/main/docs/architecture/database_architecture.md)
and
[client/API architecture](https://github.com/ironbook-labs/ironbook/blob/main/docs/architecture/client-api_architecture.md)
for details.

## Shared contracts

The contract files in
[`contracts/proto/`](https://github.com/ironbook-labs/ironbook/blob/main/contracts/proto)
(`auth.proto`, `users.proto`, `system.proto`) are the source of truth for the
API surface: they are compiled into the Rust binary at build time
(`build.rs` + `tonic-prost-build`) and shared with client implementations,
so REST and gRPC stay consistent.

## Project website

[apps/home](https://github.com/ironbook-labs/ironbook/blob/main/apps/home/README.md)
(Next.js) is the public project website and docs; it is standalone and does
not depend on the API.

## Infrastructure

Environment definitions live in
[`infra/docker/compose/`](https://github.com/ironbook-labs/ironbook/blob/main/infra/docker/compose)
(`dev.yaml`, `prod.yaml`), managed through
[`scripts/ironbook.sh`](https://github.com/ironbook-labs/ironbook/blob/main/scripts/ironbook.sh)
— see the [scripts README](https://github.com/ironbook-labs/ironbook/blob/main/scripts/README.md).
