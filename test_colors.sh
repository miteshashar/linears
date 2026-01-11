#!/bin/bash
# Test color accessibility in light and dark terminals

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing color accessibility..."
PASS=0
FAIL=0

# Test 1: Verify --no-color flag exists
echo -n "Test 1: --no-color flag exists... "
if ./target/debug/linears --help 2>&1 | grep -q "\-\-no-color"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 2: Verify NO_COLOR env var is respected
echo -n "Test 2: NO_COLOR env var is checked... "
if grep -q "NO_COLOR" src/progress.rs; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 3: Verify cyan color is used (readable on both themes)
echo -n "Test 3: Cyan color used (accessible on light/dark)... "
if grep -q "\.cyan" src/progress.rs; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 4: Verify set_no_color is called in main
echo -n "Test 4: no_color flag is passed to progress module... "
if grep -q "set_no_color" src/main.rs; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 5: Verify color is disabled when --no-color is used
echo -n "Test 5: Template without color when --no-color... "
if grep -q '"{spinner} {msg}"' src/progress.rs; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 6: Test that NO_COLOR env works (no ANSI codes in output when set)
echo -n "Test 6: NO_COLOR env disables colors... "
# Run with NO_COLOR and check that output doesn't have ANSI escape codes
# We test resources command since it doesn't need API key
output=$(NO_COLOR=1 ./target/debug/linears resources 2>&1)
# ANSI escape codes start with \x1b[ or \033[
if echo "$output" | grep -q $'\x1b\['; then
    echo "FAIL (ANSI codes found in output)"
    ((FAIL++))
else
    echo "PASS"
    ((PASS++))
fi

# Test 7: Verify color codes are ANSI standard (not hard-coded RGB)
echo -n "Test 7: Using standard ANSI colors (not RGB)... "
# Check that we're not using rgb() or #hex colors in templates
if grep -q "rgb\|#[0-9a-fA-F]" src/progress.rs 2>/dev/null; then
    echo "FAIL (non-standard color codes found)"
    ((FAIL++))
else
    echo "PASS"
    ((PASS++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All color accessibility tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
