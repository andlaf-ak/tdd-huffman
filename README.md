# TDD Huffman

A Test-Driven Development implementation of Huffman compression in Rust.

## Prerequisites

Install Rust and the required components. Choose one of the following methods:

### Option 1: Via Homebrew (macOS)
```bash
brew install rust

# Note: You may need to install clippy separately
# If `cargo clippy --version` fails, install via rustup:
# rustup component add clippy
```

### Option 2: Via rustup (Cross-platform)
```bash
# Install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install stable toolchain and components
rustup default stable
rustup component add rustfmt clippy
```

### Verify Installation
```bash
cargo --version
rustfmt --version
cargo clippy --version
```

## Quick Start

```bash
# Build
make build

# Run
make run

# Test
make test

# Format
make format

# Lint
make lint
```

## Development

```bash
# Format code
make format

# Lint code
make lint

# Clean
cargo clean
```

## License

To be determined
