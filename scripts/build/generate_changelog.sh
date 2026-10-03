#!/usr/bin/env bash

generate_changelog() {
    local version="${1:-}"

    require_version "$version" || return 1

    mkdir -p "$OUTPUT_DIR"

    if compgen -G "$ROOT_DIR/docs/changelog/$version*.md" > /dev/null; then
        cp "$ROOT_DIR/docs/changelog/$version"*.md "$OUTPUT_DIR"
    else
        cp "$ROOT_DIR/docs/changelog/unreleased.md" "$OUTPUT_DIR"
    fi

    echo "✔ Generated changelog."
}
