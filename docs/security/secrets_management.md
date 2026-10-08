# Secret management

- All secrets (`DATABASE_URL`, client API tokens, …) are supplied through
  **environment variables**, loaded with `dotenvy`.
- `apps/api/.env.example` documents variable **names only**; it contains no
  real credentials and must never be turned into a real `.env` in the repo.
- Production values are injected at deploy time (see
  `infra/docker/compose/prod.yaml`), not baked into images or scripts.


---

← [Security overview](https://github.com/ironbook-labs/ironbook/blob/main/docs/security/README.md)
