# Iron Book Web Client

Web client for Project Iron Book, built with Python and Django.

## Layout

- `ironbook/` / `web/` — Django project and apps
- `templates/` / `static/` / `theme/` — templates, static assets, Tailwind theme
- `manage.py` — Django management entry point
- `tests.py` — Django tests
- `pyproject.toml` / `uv.lock` — Python dependencies (managed with `uv`)

## Development

Complete the root setup first (Mise + Docker, `scripts/ironbook.sh setup`).
All tasks are defined in `mise.toml`:

| Task | Description |
| --- | --- |
| `mise run setup` | Sync dependencies and collect static files |
| `mise run dev` | Run the Django dev server with Tailwind (port 8080) |
| `mise run test` | Run the test suite (`pytest`) |

Copy `.env.example` to `.env` and adjust the values before running.
