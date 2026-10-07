# Security

This document describes how Iron Book protects credentials, tokens, and user
data. For reporting a vulnerability, see the [Security Policy](SECURITY.md)
(private disclosure only — never open a public issue for a vulnerability).

## Authentication

- **Passwords** are hashed with **Argon2** (memory-hard, salted per user) in
  `apps/api/src/services/auth.rs`. Cleartext passwords are never stored and
  never logged.
- **Sessions** are stateful and opaque: on login/registration the API issues a
  random token and stores only its **SHA-256 hash** in the `sessions` table.
  A database read alone therefore cannot replay anyone's session.
- Every session row carries an `active` flag and an `expires_at` timestamp,
  so sessions can be **revoked server-side** and expire without any client
  cooperation.
- Sessions reference `users(id)` with `ON DELETE CASCADE`: deleting a user
  destroys all of their sessions.
- Failed authentication attempts are logged (`warn!` on invalid tokens), so
  abuse is visible in the structured logs.

## Authorization

- Access is checked in the service layer (`apps/api/src/services/`), not just
  at the route: handlers in `views/`/`grpc/` delegate to services, which
  verify the caller before touching the database.
- User-scoped data is tied to `user_id`; there is no ambient authority — every
  request must present a valid session token.
- Privileged operations (e.g. registration) additionally require a valid
  **client API token** (see below), so arbitrary third parties cannot create
  accounts even if they reach the endpoint.

## API tokens

- Per-client tokens live in the `clients` table (`api_token`, unique per
  client) and are verified on sensitive operations via `verify_api_token`.
- Tokens are generated randomly (32 alphanumeric characters) at client
  provisioning time.
- Treat API tokens like passwords: store them in the deployment environment,
  never in source control, and rotate them if they are ever exposed. (Token
  lifecycle management — rotation endpoints, scopes, and audit of token use —
  is tracked as future work in the issue tracker.)

## Secret management

- All secrets (`DATABASE_URL`, client API tokens, …) are supplied through
  **environment variables**, loaded with `dotenvy`.
- `apps/api/.env.example` documents variable **names only**; it contains no
  real credentials and must never be turned into a real `.env` in the repo.
- Production values are injected at deploy time (see
  `infra/docker/compose/prod.yaml`), not baked into images or scripts.

## Threat model

| Threat | Mitigation in place |
| ------ | ------------------- |
| Password database theft | Argon2 hashes only; no cleartext or reversible encryption |
| Session token theft from DB | Only SHA-256 hashes stored; hashes cannot be replayed |
| Stolen/lost session token | `expires_at` bounds lifetime; `active` flag allows revocation |
| Account creation abuse | Registration requires a valid per-client API token |
| Credential stuffing / brute force | Failed attempts are logged for detection; per-user salts defeat rainbow tables |
| Cross-origin abuse | CORS restricted via `ALLOWED_ORIGINS` (`ALLOW_ALL_ORIGINS` defaults to false) |
| Secret leakage via repo | Env-only secrets; example file carries placeholders |

Out of scope / assumptions: transport security (TLS) is expected to be
terminated by the deployment environment; the API itself does not implement
rate limiting yet, so brute-force protection currently relies on monitoring
the auth logs.

## Audit logging

- Logging uses `tracing` with a JSON subscriber and file appender
  (`apps/api/src/log/`), producing machine-readable, timestamped records.
- Security-relevant events are logged explicitly: registration attempts
  (`info!` with username/email), invalid API tokens and failed credential
  checks (`warn!`).
- Logs contain no passwords or tokens — only identifiers needed for
  investigation.

## Security practices

- **Private disclosure:** vulnerabilities are reported through GitHub's
  private vulnerability reporting or the contact in the repository profile —
  never as public issues or PRs ([Security Policy](SECURITY.md)).
- **Supported versions:** only the latest release / current `main` receives
  security updates.
- **Dependencies:** the Rust API pins dependencies in `Cargo.lock`; updates
  are reviewed like any other code change.
- **Defense in depth:** hashing at rest (Argon2, SHA-256), verification in
  the service layer, revocation in the data model, and logging on top — no
  single layer is trusted alone.
