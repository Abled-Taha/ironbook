# System architecture

Iron Book is a **polyglot monorepo**: all 7 parts of the product live in one
repository, each in its own tech stack, all managed through one toolchain
(`mise` tasks + `scripts/ironbook.sh`). The onboarding promise from the
[root README](https://github.com/ironbook-labs/ironbook/blob/main/README.md)
is three steps — install the prerequisites, clone the repo, run
`scripts/ironbook.sh setup`.

## The 7 parts

| Part | Location | Tech | Status | Role |
| ---- | -------- | ---- | ------ | ---- |
| API | [`apps/api/`](https://github.com/ironbook-labs/ironbook/blob/main/apps/api/README.md) | Rust (Axum + tonic) | Live | Backend: REST + gRPC, business logic, persistence |
| Web client | [`apps/web/`](https://github.com/ironbook-labs/ironbook/blob/main/apps/web/README.md) | Python (Django) | Live | Browser client |
| Android app | [`apps/android/`](https://github.com/ironbook-labs/ironbook/blob/main/apps/android/README.md) | Kotlin | Live | Mobile client |
| Desktop app | [`apps/desktop/`](https://github.com/ironbook-labs/ironbook/blob/main/apps/desktop/README.md) | C# (Avalonia) | Live | Linux/Windows client |
| Database | — | PostgreSQL | Live | Primary data store |
| Cache | — | Redis | Planned | Caching layer |
| Project website | [`apps/home/`](https://github.com/ironbook-labs/ironbook/blob/main/apps/home/README.md) | Next.js | Live | Public site and docs |

Each app has a dedicated README with setup and development notes; see the
[apps overview](https://github.com/ironbook-labs/ironbook/blob/main/apps/README.md).

## Tooling

### mise — the task runner

The repository root holds a [`mise.toml`](https://github.com/ironbook-labs/ironbook/blob/main/mise.toml),
and each sub-project defines its own `mise.toml` (e.g.
[`apps/api/mise.toml`](https://github.com/ironbook-labs/ironbook/blob/main/apps/api/mise.toml)).
Any task for any sub-project can be run from the repository root, e.g.
`mise run //apps/api/test` — there is no need to `cd` into the app directory,
and per-app `setup` tasks do not need to be run separately after the root
setup script.

### scripts/ironbook.sh — the custom bash tooling

Everything is driven through a single entry point,
[`scripts/ironbook.sh`](https://github.com/ironbook-labs/ironbook/blob/main/scripts/ironbook.sh),
documented in the [scripts README](https://github.com/ironbook-labs/ironbook/blob/main/scripts/README.md):

| Command | Purpose |
| ------- | ------- |
| `setup` | First-time developer setup (see [data flow](https://github.com/ironbook-labs/ironbook/blob/main/docs/architecture/data_flow.md)) |
| `build` | Build the project artifacts |
| `release` | Cut and sign a release |
| `update-version` | Bump version numbers |
| `get-tree` / `get-codebase` | Developer utilities (repo tree, codebase dump) |

Implementation layout:

| Directory | Purpose |
| --------- | ------- |
| [`scripts/lib/`](https://github.com/ironbook-labs/ironbook/blob/main/scripts/lib) | Shared shell helpers (`vars.sh`, `utility.sh`, `help.sh`) sourced by the entry point |
| [`scripts/setup/`](https://github.com/ironbook-labs/ironbook/blob/main/scripts/setup) | First-time setup: `system.sh`, `mise.sh`, `docker.sh`, `environment.sh`, `project.sh`, `git-hooks.sh` |
| [`scripts/build/`](https://github.com/ironbook-labs/ironbook/blob/main/scripts/build) | Build, packaging, changelog generation |
| [`scripts/release/`](https://github.com/ironbook-labs/ironbook/blob/main/scripts/release) | Versioning, signing, publishing |
| [`scripts/install/`](https://github.com/ironbook-labs/ironbook/blob/main/scripts/install) | Installer scripts used by the curl-based install in the root README |
| [`scripts/tools/`](https://github.com/ironbook-labs/ironbook/blob/main/scripts/tools) | `get_tree`, `get_codebase` utilities |

### Environment handling

Environment variables are the single mechanism for configuration across all
parts. Variable names are documented in
[`.env.example`](https://github.com/ironbook-labs/ironbook/blob/main/.env.example)
at the repo root (and per-app where needed, e.g.
[`apps/api/.env.example`](https://github.com/ironbook-labs/ironbook/blob/main/apps/api/.env.example));
real values are supplied at deploy time and never committed. The setup script
(`scripts/ironbook.sh setup`, via `scripts/setup/environment.sh`) handles
environment preparation automatically — see
[data flow](https://github.com/ironbook-labs/ironbook/blob/main/docs/architecture/data_flow.md).

### Shared contracts and infrastructure

- Shared API contracts live in
  [`contracts/proto/`](https://github.com/ironbook-labs/ironbook/blob/main/contracts/proto)
  (`auth.proto`, `users.proto`, `system.proto`) and are the source of truth
  for server and clients alike.
- Infrastructure definitions live in
  [`infra/docker/compose/`](https://github.com/ironbook-labs/ironbook/blob/main/infra/docker/compose)
  (`dev.yaml`, `prod.yaml`).

See [component relationships](https://github.com/ironbook-labs/ironbook/blob/main/docs/architecture/component_relationships.md)
for how these pieces connect.
