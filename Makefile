.PHONY: build run test format lint check

# Default target
all: test

# Build the project
build:
	cargo build

# Run the project
run:
	cargo run

# Run tests
test:
	cargo test

# Format code
format:
	cargo fmt

# Lint code
lint:
	cargo clippy -- -D warnings

# Run all checks (format, lint, test)
check: format lint test
	@echo "✅ All checks passed!"
