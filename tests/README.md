# Test Organization

This project organizes tests into distinct folders for better clarity and maintainability.

## 📁 Directory Structure

```
tests/
├── unit/                   # Unit tests (40 tests)
│   ├── code_extraction_tests.rs
│   ├── frequency_map_tests.rs
│   ├── input_bit_stream_tests.rs
│   ├── node_selection_tests.rs
│   ├── output_bit_stream_tests.rs
│   └── tree_construction_tests.rs
└── property/               # Property-based tests (4 tests)
    ├── code_extraction_property_tests.rs
    ├── tree_construction_property_tests.rs
    └── code_extraction_tests.proptest-regressions
```

## 🧪 Test Types

### Unit Tests (`tests/unit/`)
- **Purpose**: Focused, deterministic scenarios
- **Characteristics**: Fast, specific inputs/outputs, edge cases
- **Count**: 40 tests across 6 files

### Property-Based Tests (`tests/property/`)
- **Purpose**: Comprehensive algorithmic validation
- **Characteristics**: Generated inputs, invariant testing, broader coverage
- **Count**: 4 tests across 2 files
- **Tool**: Uses [PropTest](https://github.com/AltSysrq/proptest)

## 🚀 Running Tests

### All Tests
```bash
cargo test
```

### Specific Test Files
```bash
# Run specific unit tests
cargo test --test code_extraction_tests
cargo test --test input_bit_stream_tests

# Run specific property tests
cargo test --test code_extraction_property_tests
cargo test --test tree_construction_property_tests
```

### By Test Type Pattern
```bash
# Run tests whose names contain "property"
cargo test property

# Run tests whose names contain specific functionality
cargo test bit_stream
cargo test extraction
```

## ⚙️ Configuration

Test discovery is configured in `Cargo.toml` using `[[test]]` sections to explicitly define the path for each test file, enabling tests to be organized in subdirectories while maintaining Cargo's standard test runner functionality.
