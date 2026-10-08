# Audit logging

- Logging uses `tracing` with a JSON subscriber and file appender
  (`apps/api/src/log/`), producing machine-readable, timestamped records.
- Security-relevant events are logged explicitly: registration attempts
  (`info!` with username/email), invalid API tokens and failed credential
  checks (`warn!`).
- Logs contain no passwords or tokens — only identifiers needed for
  investigation.


---

← [Security overview](https://github.com/ironbook-labs/ironbook/blob/main/docs/security/README.md)
