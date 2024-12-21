# Default recipe, when nothing's selected
[private]
default:
  just --list --list-submodules

# Build glazy with the tracing feature enabled.
build:
  cargo build

# Build zjstatus with tracing and start a zellij session with the dev layout.
run target="glazy":
  RUST_LOG=debug cargo run -p {{target}} -- --config ./config.kdl

# Watch and run tests with nextest.
test:
  RUST_LOG=trace cargo watch -x "nextest run --lib"

# Lint with clippy and cargo audit.
lint:
  cargo clippy --all-features --lib
  cargo audit

# Create and push a new release version.
release:
  #!/usr/bin/env bash
  export VERSION="$( git cliff --bumped-version )"
  cargo set-version "${VERSION:1}"
  direnv exec . cargo build --release
  git commit -am "chore: bump version to $VERSION"
  git tag -m "$VERSION" "$VERSION"
  git push origin main
  git push origin "$VERSION"
