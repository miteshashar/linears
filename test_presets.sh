#!/bin/bash
# Test field selection presets

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing field selection presets..."
PASS=0
FAIL=0

# Test 1: Minimal preset works
echo -n "Test 1: Minimal preset works... "
output=$(./target/debug/linears --out json list issue --preset minimal 2>&1)
exit_code=$?
if [ "$exit_code" -eq 0 ] && echo "$output" | grep -q "nodes"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $exit_code)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 2: Default preset works
echo -n "Test 2: Default preset works... "
output=$(./target/debug/linears --out json list issue --preset default 2>&1)
exit_code=$?
if [ "$exit_code" -eq 0 ] && echo "$output" | grep -q "nodes"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $exit_code)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 3: Wide preset works
echo -n "Test 3: Wide preset works... "
output=$(./target/debug/linears --out json list issue --preset wide 2>&1)
exit_code=$?
if [ "$exit_code" -eq 0 ] && echo "$output" | grep -q "nodes"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $exit_code)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 4: Minimal has fewer fields than default
echo -n "Test 4: Minimal has fewer fields than default... "
minimal_output=$(./target/debug/linears -v --out json list issue --preset minimal 2>&1)
default_output=$(./target/debug/linears -v --out json list issue --preset default 2>&1)
minimal_len=$(echo "$minimal_output" | grep -A 20 "Query:" | wc -c)
default_len=$(echo "$default_output" | grep -A 20 "Query:" | wc -c)
if [ "$minimal_len" -lt "$default_len" ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (minimal: $minimal_len chars, default: $default_len chars)"
    ((FAIL++))
fi

# Test 5: Wide has more fields than default
echo -n "Test 5: Wide has more fields than default... "
wide_output=$(./target/debug/linears -v --out json list issue --preset wide 2>&1)
wide_len=$(echo "$wide_output" | grep -A 20 "Query:" | wc -c)
if [ "$wide_len" -gt "$default_len" ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (wide: $wide_len chars, default: $default_len chars)"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All preset tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
