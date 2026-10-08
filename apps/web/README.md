# Iron Book Web Client

Web client for Project Iron Book, built with Python and Django.

## Layout

- `ironbook/` / `web/` — Django project and apps
- `templates/` / `static/` / `theme/` — templates, static assets, Tailwind theme
- `manage.py` — Django management entry point
- `tests.py` — Django tests
- `pyproject.toml` / `uv.lock` — Python dependencies (managed with `uv`)

## Development

Complete the root setup first (`scripts/ironbook.sh setup`).
All tasks can be run from the repository root:

| Task | Description |
| --- | --- |
| `mise run //apps/web/dev` | Run the Django dev server with Tailwind (port 8080) |
| `mise run //apps/web/test` | Run the test suite (`pytest`) |
