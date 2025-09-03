.PHONY: build run test format lint check

# Default target
all: test

# Build the project
build:
	cargo build

# Run the project
run:
	cargo run

# Run all tests (unit, property, and e2e)
test: build
	@echo "🧪 Running unit and property tests..."
	cargo test
	@echo "🔗 Running end-to-end tests..."
	./scripts/e2e_test.sh
	@echo "✅ All tests passed!"

# Format code
format:
	cargo fmt

# Lint code
lint:
	cargo clippy -- -D warnings

# Run all checks (format, lint, test)
check: format lint test
	@echo "✅ All checks passed!"
