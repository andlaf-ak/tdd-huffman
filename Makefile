.PHONY: build run test format lint

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
