# Apps

The applications of Project Iron Book. Each app lives in its own directory,
uses its own tech stack, and has a dedicated README with setup and development
notes.

| App | Tech | Description |
| --- | --- | --- |
| `api/` | Rust (Axum) | Backend API |
| `web/` | Python (Django) | Web client |
| `android/` | Kotlin | Android app |
| `desktop/` | C# (Avalonia) | Linux/Windows desktop app |
| `home/` | Next.js | Project website |

## Getting started

Complete the root [README](../README.md) setup first
(`scripts/ironbook.sh setup`). Each app defines `dev` and `test` tasks —
run them from the repository root, no need to `cd` into the app directory:

```sh
mise run //apps/api/test
mise run //apps/web/dev
```
