# Client/API architecture

This document covers how the clients — [web](https://github.com/ironbook-labs/ironbook/blob/main/apps/web/README.md)
(Django), [Android](https://github.com/ironbook-labs/ironbook/blob/main/apps/android/README.md)
(Kotlin), [desktop](https://github.com/ironbook-labs/ironbook/blob/main/apps/desktop/README.md)
(C# with Avalonia) — communicate with the
[API](https://github.com/ironbook-labs/ironbook/blob/main/apps/api/README.md)
(Rust: Axum + tonic). See
[component relationships](https://github.com/ironbook-labs/ironbook/blob/main/docs/architecture/component_relationships.md)
for where this fits in the whole project.

- The API binary exposes **two surfaces from one process**: REST (Axum) and
  gRPC (tonic). Both are thin adapters over the same `services` layer, so
  behavior is identical regardless of transport.
- The shared contract files in
  [`contracts/proto/`](https://github.com/ironbook-labs/ironbook/blob/main/contracts/proto)
  are compiled into the Rust binary at build time (`build.rs` +
  `tonic-prost-build`) and serve as the source of truth for client
  implementations.
- Clients authenticate per user (session tokens) and, for privileged
  operations such as registration, per client (API tokens from the `clients`
  table). See the
  [Security](https://github.com/ironbook-labs/ironbook/blob/main/docs/SECURITY.md)
  document for the full model.
- CORS is configurable (`ALLOWED_ORIGINS`, `ALLOW_ALL_ORIGINS` — documented in
  [`apps/api/.env.example`](https://github.com/ironbook-labs/ironbook/blob/main/apps/api/.env.example))
  rather than open by default.
- A typical request travels: transport (`src/views/` for REST, `src/grpc/`
  for gRPC) → business logic (`src/services/`) → persistence (`src/db/` via
  `sqlx` against PostgreSQL) → structured error responses (`src/errors.rs`).
