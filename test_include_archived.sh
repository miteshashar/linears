#!/bin/bash
# Test --include-archived functionality

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing --include-archived functionality..."
PASS=0
FAIL=0

# Test 1: List without --include-archived works
echo -n "Test 1: List without --include-archived works... "
output=$(./target/debug/linears --out json list issue 2>&1)
exit_code=$?
if [ "$exit_code" -eq 0 ] && echo "$output" | grep -q "nodes"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $exit_code)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 2: List with --include-archived works
echo -n "Test 2: List with --include-archived works... "
output=$(./target/debug/linears --out json list issue --include-archived 2>&1)
exit_code=$?
if [ "$exit_code" -eq 0 ] && echo "$output" | grep -q "nodes"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $exit_code)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 3: Verbose shows includeArchived in variables
echo -n "Test 3: Verbose shows includeArchived when flag used... "
output=$(./target/debug/linears -v --out json list issue --include-archived 2>&1)
if echo "$output" | grep -q "includeArchived"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (includeArchived not in verbose output)"
    echo "Output: $output"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All --include-archived tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
