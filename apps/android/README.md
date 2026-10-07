# Iron Book Android App

Android client for Project Iron Book, written in Kotlin.

## Layout

- `app/` — the Android application module
- `build.gradle.kts` / `settings.gradle.kts` — Gradle build configuration
- `gradle/` / `gradlew` / `gradlew.bat` — Gradle wrapper (no local Gradle install needed)

## Development

Complete the root setup first (Mise + Docker, `scripts/ironbook.sh setup`).
See `mise.toml` in this directory for the `setup`, `dev` and `test` tasks.
You can also use the Gradle wrapper directly, e.g. `./gradlew build`.
