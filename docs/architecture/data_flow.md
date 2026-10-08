# Data flow

## Setup flow

Developer onboarding is three steps (see the
[root README](https://github.com/ironbook-labs/ironbook/blob/main/README.md)
and [supported platforms](https://github.com/ironbook-labs/ironbook/blob/main/docs/setup/supported_platforms.md)):

1. Install the prerequisites (Mise, Docker / Docker Compose — installed
   automatically by the setup where possible).
2. Clone the repo.
3. Run `scripts/ironbook.sh setup`.

The `setup` command sources
[`scripts/setup/setup.sh`](https://github.com/ironbook-labs/ironbook/blob/main/scripts/setup/setup.sh),
which runs, in order:

| Script | Responsibility |
| ------ | -------------- |
| `system.sh` | OS-level prerequisites |
| `mise.sh` | Toolchain installation via mise |
| `docker.sh` | Container services (PostgreSQL, etc.) |
| `environment.sh` | Environment file preparation from `.env.example` |
| `project.sh` | Per-project bootstrapping |
| `git-hooks.sh` | Git hook installation |

Sub-project `setup` tasks do not need to be executed separately afterwards —
everything is handled by the root setup script.

## Build and release flow

[`scripts/ironbook.sh`](https://github.com/ironbook-labs/ironbook/blob/main/scripts/ironbook.sh)
routes `build` and `release` to the
[build](https://github.com/ironbook-labs/ironbook/blob/main/scripts/build) and
[release](https://github.com/ironbook-labs/ironbook/blob/main/scripts/release)
directories: build, packaging, and changelog generation, then version bumping,
signing, and publishing. Installers are produced from
[`scripts/install/`](https://github.com/ironbook-labs/ironbook/blob/main/scripts/install)
and distributed via the curl-based install documented in the root README.

## Environment flow

Configuration reaches every part of the system through environment variables:

- Variable names are documented in
  [`.env.example`](https://github.com/ironbook-labs/ironbook/blob/main/.env.example)
  at the repo root, and per-app where needed (e.g.
  [`apps/api/.env.example`](https://github.com/ironbook-labs/ironbook/blob/main/apps/api/.env.example)
  documents `DATABASE_URL`, `ALLOWED_ORIGINS`, `ALLOW_ALL_ORIGINS`).
- The setup script prepares environment files automatically; real values are
  supplied at deploy time and never committed to the repo.
- The API loads variables at startup via `dotenvy`; clients read their own
  environment per platform.

## Runtime request flow (high level)

A typical user request travels: **client** (web / Android / desktop) →
**API** (REST via Axum or gRPC via tonic) → **PostgreSQL**. The API validates
input, verifies credentials, applies business logic, and persists through
`sqlx`; a central error module maps failures to consistent API error
responses. See
[client/API architecture](https://github.com/ironbook-labs/ironbook/blob/main/docs/architecture/client-api_architecture.md)
for the full breakdown.
