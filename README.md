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

## Running the Program

After building the project, you can use the Huffman compression utility:

```bash
# Build the executable
cargo build --release

# The executable will be at target/release/huffman
```

### Command Line Usage

The program supports two main operations: compression and decompression.

#### Compression
```bash
# Compress a file
./target/release/huffman --compress input.txt -o compressed.huf
# or using short form
./target/release/huffman -c input.txt -o compressed.huf
```

#### Decompression
```bash
# Decompress a file
./target/release/huffman --decompress compressed.huf -o output.txt
# or using short form
./target/release/huffman -d compressed.huf -o output.txt
```

### Command Line Options

| Option | Short | Description | Required |
|--------|-------|-------------|----------|
| `--compress` | `-c` | Compress the input file | Yes (or `-d`) |
| `--decompress` | `-d` | Decompress the input file | Yes (or `-c`) |
| `--output` | `-o` | Specify output file | Yes |
| `<input>` | | Input file path (positional argument) | Yes |

### Examples

```bash
# Compress a text file
./target/release/huffman -c document.txt -o document.huf

# Decompress back to original
./target/release/huffman -d document.huf -o restored.txt

# Get help
./target/release/huffman --help
```

The program will display compression statistics including original size, compressed size, and compression ratio.

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
