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

### Node.js (for Husky Git hooks)
```bash
# Install Node.js (if not already installed)
# Via Homebrew:
brew install node

# Or download from: https://nodejs.org/
```

## Quick Start

```bash
# Clone and setup
git clone <repository-url>
cd tdd-huffman

# Install dependencies (including Husky hooks)
npm install

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

# Run all checks
make check
```

## Development

### Git Hooks (Husky)

This project uses Husky for Git hooks to ensure code quality:

- **Pre-commit**: Automatically runs `make format` and `make lint`
- **Pre-push**: Automatically runs `make test`

The hooks are automatically installed when you run `npm install`.

### Manual Commands

```bash
# Format code
make format

# Lint code
make lint

# Run all checks manually
make check

# Clean
cargo clean
```

## License

To be determined
