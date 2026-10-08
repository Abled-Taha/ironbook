# Database architecture

PostgreSQL is the single source of truth. The API is the only component that
connects to it — no client holds database credentials. Schema is versioned
with `sqlx`-managed migrations in
[`apps/api/migrations/`](https://github.com/ironbook-labs/ironbook/blob/main/apps/api/migrations):

| Migration | Contents |
| --------- | -------- |
| `0001_initial.sql` | `users` (id, username, email, Argon2 `password_hash`), `sessions` (user FK with `ON DELETE CASCADE`, `token_hash`, `active`, `expires_at`) |
| `0002_add_clients_table.sql` | `clients` (name, owner email, unique `api_token`) |

## Design notes

- Passwords are never stored; only Argon2 hashes.
- Session tokens are never stored in cleartext; only SHA-256 hashes, so a
  database read alone cannot replay a session.
- Sessions carry `active` + `expires_at`, which makes server-side revocation
  and expiry possible without client cooperation.
- Deleting a user cascades to their sessions.
- An offline query cache (`.sqlx/`) keeps compile-time query checking working
  without a live database.
- Database services are provisioned through the Docker Compose definitions in
  [`infra/docker/compose/`](https://github.com/ironbook-labs/ironbook/blob/main/infra/docker/compose),
  started during `scripts/ironbook.sh setup` — see
  [data flow](https://github.com/ironbook-labs/ironbook/blob/main/docs/architecture/data_flow.md).

See the [Security](https://github.com/ironbook-labs/ironbook/blob/main/docs/SECURITY.md)
document for the authentication model built on this schema.
