_default:
  just all

# Check code format
fmt:
  cargo fmt --check

# Check and generate the documentation
doc *args:
  RUSTDOCFLAGS='-D warnings' cargo doc --locked {{args}}

# Type check
check *args:
  cargo check --locked --all-targets {{args}}

# Lint with clippy
clippy *args:
  cargo clippy --locked {{args}} -- -D warnings

# Run tests
test *args:
  cargo test --locked {{args}}

# Run all combinations of checks and features
all *args:
  bash check-all.bash {{args}}
