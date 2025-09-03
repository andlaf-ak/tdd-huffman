#!/bin/bash

# End-to-end test script for Huffman compression
# Generates test files of various sizes, compresses them, decompresses them,
# and validates round-trip integrity using hash comparison

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test configuration
TEMP_DIR="temp_test_$$"
BINARY="./target/debug/huffman"

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[PASS]${NC} $1"
}

print_error() {
    echo -e "${RED}[FAIL]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

# Function to generate test content of specified size
generate_test_content() {
    local size=$1
    local output_file=$2
    
    if [ "$size" -eq 0 ]; then
        # Empty file
        touch "$output_file"
    elif [ "$size" -le 1000 ]; then
        # Small files: use repeated patterns for good compression
        local pattern="Lorem ipsum dolor sit amet, consectetur adipiscing elit. "
        yes "$pattern" | head -c "$size" > "$output_file"
    else
        # Larger files: create deterministic content
        {
            # First part: repetitive content (good compression)
            local repeat_size=$((size / 3))
            yes "The quick brown fox jumps over the lazy dog. " | head -c "$repeat_size"
            
            # Second part: alphabet pattern
            local alpha_size=$((size / 3))
            yes "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789" | head -c "$alpha_size"
            
            # Third part: mixed content
            local remaining=$((size - repeat_size - alpha_size))
            yes "Mixed content with numbers 12345 and symbols !@#$%^&*() for testing compression algorithms." | head -c "$remaining"
        } > "$output_file"
        
        # Ensure exact size
        truncate -s "$size" "$output_file" 2>/dev/null || {
            # Fallback if truncate is not available
            head -c "$size" "$output_file" > "${output_file}.tmp" && mv "${output_file}.tmp" "$output_file"
        }
    fi
}

# Function to calculate file hash
calculate_hash() {
    local file=$1
    if command -v sha256sum >/dev/null 2>&1; then
        sha256sum "$file" | cut -d' ' -f1
    elif command -v shasum >/dev/null 2>&1; then
        shasum -a 256 "$file" | cut -d' ' -f1
    elif command -v openssl >/dev/null 2>&1; then
        openssl dgst -sha256 "$file" | cut -d' ' -f2
    else
        print_error "No hash utility found (sha256sum, shasum, or openssl required)"
        exit 1
    fi
}

# Function to test compression for a specific file size
test_file_size() {
    local size=$1
    local test_name="size_${size}_bytes"
    
    print_status "Testing file size: $size bytes"
    
    # Generate test file
    local original_file="$TEMP_DIR/test_$test_name.txt"
    generate_test_content "$size" "$original_file"
    
    # Verify generated file size
    local actual_size=$(wc -c < "$original_file")
    if [ "$actual_size" -ne "$size" ]; then
        print_error "Generated file size mismatch: expected $size, got $actual_size"
        return 1
    fi
    
    # Calculate original hash
    local original_hash=$(calculate_hash "$original_file")
    print_status "Original file hash: ${original_hash:0:16}..."
    
    # Special handling for empty files (Huffman compression may not handle these)
    if [ "$size" -eq 0 ]; then
        print_warning "Skipping compression test for empty file (may not be supported)"
        print_success "Empty file test completed for $test_name"
        return 0
    fi
    
    # Compress the file
    local compressed_file="$TEMP_DIR/test_$test_name.huf"
    print_status "Compressing..."
    
    # Capture compression output and check for errors
    local compression_output
    if compression_output=$("$BINARY" -c "$original_file" -o "$compressed_file" 2>&1); then
        print_status "Compression completed successfully"
    else
        print_error "Compression failed for $test_name"
        print_error "Error output: $compression_output"
        return 1
    fi
    
    # Check if compressed file was created and has content
    if [ ! -f "$compressed_file" ]; then
        print_error "Compressed file was not created for $test_name"
        return 1
    fi
    
    local compressed_size=$(wc -c < "$compressed_file")
    if [ "$compressed_size" -eq 0 ]; then
        print_error "Compressed file is empty for $test_name"
        return 1
    fi
    
    # Calculate compression ratio
    local compression_ratio="N/A"
    if [ "$size" -gt 0 ] && command -v bc >/dev/null 2>&1; then
        compression_ratio=$(echo "scale=3; $compressed_size / $size" | bc -l 2>/dev/null || echo "N/A")
    fi
    
    print_status "Compressed size: $compressed_size bytes (ratio: $compression_ratio)"
    
    # Test decompression (round-trip)
    print_status "Testing decompression..."
    local decompressed_file="$TEMP_DIR/test_${test_name}_decompressed.txt"
    
    # Decompress using the binary's -d option
    local decompression_output
    if decompression_output=$("$BINARY" -d "$compressed_file" -o "$decompressed_file" 2>&1); then
        print_status "Decompression completed successfully"
    else
        print_error "Decompression failed for $test_name"
        print_error "Error output: $decompression_output"
        return 1
    fi
    
    # Verify decompressed file exists and has correct size
    if [ ! -f "$decompressed_file" ]; then
        print_error "Decompressed file was not created for $test_name"
        return 1
    fi
    
    local decompressed_size=$(wc -c < "$decompressed_file")
    if [ "$decompressed_size" -ne "$size" ]; then
        print_error "Decompressed file size mismatch for $test_name: expected $size, got $decompressed_size"
        return 1
    fi
    
    # Calculate decompressed hash and compare with original
    local decompressed_hash=$(calculate_hash "$decompressed_file")
    print_status "Decompressed file hash: ${decompressed_hash:0:16}..."
    
    if [ "$original_hash" = "$decompressed_hash" ]; then
        print_success "Round-trip test passed for $test_name (ratio: $compression_ratio)"
        return 0
    else
        print_error "Hash mismatch for $test_name!"
        print_error "  Original:     ${original_hash:0:32}..."
        print_error "  Decompressed: ${decompressed_hash:0:32}..."
        return 1
    fi
}

# Main test function
run_tests() {
    print_status "Starting end-to-end Huffman compression/decompression round-trip tests"
    
    # Create temporary directory
    mkdir -p "$TEMP_DIR"
    
    # Check if binary exists
    if [ ! -f "$BINARY" ]; then
        print_error "Binary not found: $BINARY"
        print_status "Building project first..."
        if ! cargo build; then
            print_error "Failed to build project"
            exit 1
        fi
        if [ ! -f "$BINARY" ]; then
            print_error "Binary still not found after build"
            exit 1
        fi
    fi
    
    # Test file sizes (in bytes)
    local test_sizes=(
        1          # Single byte
        10         # Very small
        50         # Small
        100        # Small with some repetition
        500        # Medium small
        1000       # 1KB
        5000       # 5KB
        10000      # 10KB
    )
    
    local passed=0
    local total=${#test_sizes[@]}
    
    for size in "${test_sizes[@]}"; do
        if test_file_size "$size"; then
            ((passed++))
        fi
        echo
    done
    
    # Summary
    echo "=================================================="
    print_status "Test Summary:"
    print_status "  Passed: $passed/$total"
    
    if [ "$passed" -eq "$total" ]; then
        print_success "All end-to-end round-trip tests passed! 🎉"
        cleanup
        exit 0
    else
        print_error "Some tests failed! ❌"
        cleanup
        exit 1
    fi
}

# Cleanup function
cleanup() {
    if [ -d "$TEMP_DIR" ]; then
        rm -rf "$TEMP_DIR"
    fi
}

# Trap to ensure cleanup on exit
trap cleanup EXIT

# Check for required tools
print_status "Checking required tools..."

if ! command -v yes >/dev/null 2>&1; then
    print_error "'yes' command not found"
    exit 1
fi

if ! command -v truncate >/dev/null 2>&1 && ! command -v head >/dev/null 2>&1; then
    print_error "Neither 'truncate' nor 'head' command found"
    exit 1
fi

# Run the tests
run_tests
