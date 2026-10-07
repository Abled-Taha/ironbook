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

Complete the root [README](../README.md) setup first (Mise + Docker, then
`scripts/ironbook.sh setup`). Each app additionally defines `setup`, `dev` and
`test` tasks in its own `mise.toml` — run them from inside the app directory:

```sh
cd apps/<app>
mise run setup
mise run dev
```
