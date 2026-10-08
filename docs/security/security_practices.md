# Security practices

- **Private disclosure:** vulnerabilities are reported through GitHub's
  private vulnerability reporting or the contact in the repository profile —
  never as public issues or PRs ([Security Policy](https://github.com/ironbook-labs/ironbook/blob/main/SECURITY.md)).
- **Supported versions:** only the latest release / current `main` receives
  security updates.
- **Dependencies:** the Rust API pins dependencies in `Cargo.lock`; updates
  are reviewed like any other code change.
- **Defense in depth:** hashing at rest (Argon2, SHA-256), verification in
  the service layer, revocation in the data model, and logging on top — no
  single layer is trusted alone.


---

← [Security overview](https://github.com/ironbook-labs/ironbook/blob/main/docs/security/README.md)
