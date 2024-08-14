#!/usr/bin/env bash
RUNMIGRATION="run --bin migrate"
RUNAPP="cargo run --bin app"
BUILDTAILWINDCS="npx tailwindcss -i ./static/input.css -o ./static/style.css"
cargo watch -x "$RUNMIGRATION && $BUILDTAILWINDCS && $RUNAPP"
