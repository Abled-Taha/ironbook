# Authentication

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


---

← [Security overview](https://github.com/ironbook-labs/ironbook/blob/main/docs/security/README.md)
