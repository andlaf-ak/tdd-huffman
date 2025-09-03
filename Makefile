.PHONY: build run test test-unit test-e2e format lint check

# Default target
all: test

# Build the project
build:
	cargo build

# Run the project
run:
	cargo run

# Run unit and integration tests
test-unit:
	cargo test

# Run end-to-end tests
test-e2e: build
	./scripts/e2e_test.sh

# Run all tests (unit + e2e)
test: test-unit test-e2e

# Format code
format:
	cargo fmt

# Lint code
lint:
	cargo clippy -- -D warnings

# Run all checks (format, lint, test)
check: format lint test
	@echo "✅ All checks passed!"
