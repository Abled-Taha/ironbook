# Iron Book Desktop App

Linux/Windows desktop client for Project Iron Book, written in C# with
Avalonia.

## Layout

- `Views/` / `ViewModels/` — UI views and their view models
- `Assets/` — application assets
- `Program.cs` / `App.axaml` / `App.axaml.cs` — application entry point
- `ironbook.csproj` — .NET project file
- `app.manifest` — Windows application manifest
- `tests/` — tests

## Development

Complete the root setup first (`scripts/ironbook.sh setup`).
Run `dev` and `test` tasks from the repository root, e.g. `mise run //apps/desktop/dev`.
