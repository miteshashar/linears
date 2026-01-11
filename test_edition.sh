#!/bin/bash
# Test Rust 2024 edition configuration

echo "Testing Rust 2024 edition..."
PASS=0
FAIL=0

# Test 1: Verify main Cargo.toml has edition = "2024"
echo -n "Test 1: Main crate uses edition 2024... "
if grep -q 'edition = "2024"' Cargo.toml; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 2: Verify xtask Cargo.toml has edition = "2024"
echo -n "Test 2: xtask crate uses edition 2024... "
if grep -q 'edition = "2024"' xtask/Cargo.toml; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 3: Verify rust-version is at least 1.85
echo -n "Test 3: Main crate rust-version >= 1.85... "
if grep -q 'rust-version = "1.8[5-9]"\|rust-version = "1.9' Cargo.toml; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 4: Verify xtask rust-version is at least 1.85
echo -n "Test 4: xtask rust-version >= 1.85... "
if grep -q 'rust-version = "1.8[5-9]"\|rust-version = "1.9' xtask/Cargo.toml; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 5: Project builds successfully
echo -n "Test 5: Project compiles with 2024 edition... "
if ./init.sh build > /dev/null 2>&1; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All Rust 2024 edition tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
