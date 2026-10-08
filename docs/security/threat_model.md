# Threat model

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


---

← [Security overview](https://github.com/ironbook-labs/ironbook/blob/main/docs/security/README.md)
