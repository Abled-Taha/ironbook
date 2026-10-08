# Major design decisions

1. **Polyglot monorepo over micro-repos.** One repo, one setup script
   (`scripts/ironbook.sh setup`), per-app `mise` tasks runnable from the
   repository root (e.g. `mise run //apps/api/test`). The onboarding promise
   is three steps, two of which are prerequisites — see the
   [root README](https://github.com/ironbook-labs/ironbook/blob/main/README.md).
2. **One toolchain for everything.** `mise` (root + per-app `mise.toml`) plus
   the custom `scripts/ironbook.sh` entry point cover setup, build, release,
   and developer utilities for all 7 parts — no per-stack build scripts to
   keep in sync.
3. **Proto-first contracts.** [`contracts/proto/`](https://github.com/ironbook-labs/ironbook/blob/main/contracts/proto)
   is compiled into the server and shared with clients, so the API surface is
   defined once and REST and gRPC stay consistent.
4. **API as the single gateway.** The API is the only component that talks to
   the database; every client (web, Android, desktop) goes through it. No
   client holds database credentials — see
   [component relationships](https://github.com/ironbook-labs/ironbook/blob/main/docs/architecture/component_relationships.md).
5. **One binary, two transports.** The API exposes REST (Axum) and gRPC
   (tonic) from one process, both thin adapters over the same services layer
   — no duplicated logic.
6. **Stateful sessions, hashed tokens.** Sessions live in the database with
   hashes, expiry, and a revocation flag, trading a small lookup cost for
   server-side control over every issued token. See the
   [Security](https://github.com/ironbook-labs/ironbook/blob/main/docs/SECURITY.md)
   document and
   [database architecture](https://github.com/ironbook-labs/ironbook/blob/main/docs/architecture/database_architecture.md).
7. **Secrets in the environment, never in the repo.** `.env.example` documents
   variable names only; real values are supplied at deploy time. The setup
   script prepares environment files automatically.
8. **Reproducibility.** Docker Compose definitions for dev and prod
   ([`infra/docker/compose/`](https://github.com/ironbook-labs/ironbook/blob/main/infra/docker/compose)),
   an offline sqlx query cache (`.sqlx/`), and mise-managed toolchains keep
   builds working on any machine.
