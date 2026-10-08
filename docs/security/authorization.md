# Authorization

- Access is checked in the service layer (`apps/api/src/services/`), not just
  at the route: handlers in `views/`/`grpc/` delegate to services, which
  verify the caller before touching the database.
- User-scoped data is tied to `user_id`; there is no ambient authority — every
  request must present a valid session token.
- Privileged operations (e.g. registration) additionally require a valid
  **client API token** (see [API tokens](https://github.com/ironbook-labs/ironbook/blob/main/docs/security/api_tokens.md)), so arbitrary
  third parties cannot create accounts even if they reach the endpoint.


---

← [Security overview](https://github.com/ironbook-labs/ironbook/blob/main/docs/security/README.md)
