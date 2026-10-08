# Architecture

This directory documents the architecture of Project Iron Book — a
**polyglot monorepo** ("A Digital Financial Ledger") made of 7 integrated
parts, each in its own tech stack, all managed through one toolchain. Start
with the [root README](https://github.com/ironbook-labs/ironbook/blob/main/README.md)
for onboarding, and the [Security](https://github.com/ironbook-labs/ironbook/blob/main/docs/SECURITY.md)
document for authentication and the threat model.

## Contents

| Document | What it covers |
| -------- | -------------- |
| [System architecture](https://github.com/ironbook-labs/ironbook/blob/main/docs/architecture/system_architecture.md) | The 7 parts of the monorepo, the mise toolchain, and the `scripts/ironbook.sh` tooling |
| [Component relationships](https://github.com/ironbook-labs/ironbook/blob/main/docs/architecture/component_relationships.md) | How the sub-projects relate: clients, API, database, website, contracts, infrastructure |
| [Data flow](https://github.com/ironbook-labs/ironbook/blob/main/docs/architecture/data_flow.md) | Setup flow, build/release flow, environment handling, and a typical runtime request |
| [Major design decisions](https://github.com/ironbook-labs/ironbook/blob/main/docs/architecture/major_design_decisions.md) | Why the project is shaped this way |
| [Database architecture](https://github.com/ironbook-labs/ironbook/blob/main/docs/architecture/database_architecture.md) | PostgreSQL as the single source of truth: migrations and schema design |
| [Client/API architecture](https://github.com/ironbook-labs/ironbook/blob/main/docs/architecture/client-api_architecture.md) | How the clients (web, Android, desktop) talk to the Rust backend |
