# API tokens

- Per-client tokens live in the `clients` table (`api_token`, unique per
  client) and are verified on sensitive operations via `verify_api_token`.
- Tokens are generated randomly (32 alphanumeric characters) at client
  provisioning time.
- Treat API tokens like passwords: store them in the deployment environment,
  never in source control, and rotate them if they are ever exposed. (Token
  lifecycle management — rotation endpoints, scopes, and audit of token use —
  is tracked as future work in the issue tracker.)


---

← [Security overview](https://github.com/ironbook-labs/ironbook/blob/main/docs/security/README.md)
