#!/bin/bash
# Test network error exit code

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing network error handling..."
PASS=0
FAIL=0

# Test 1: Network error with invalid endpoint returns exit code 3
echo -n "Test 1: Network error returns exit code 3... "
output=$(./target/debug/linears --endpoint http://localhost:1 list issue 2>&1)
exit_code=$?
if [ "$exit_code" -eq 3 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $exit_code, expected 3)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 2: Error message mentions connection
echo -n "Test 2: Error message mentions connection... "
if echo "$output" | grep -qi "connection\|connect"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (no connection mention)"
    echo "Output: $output"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All network error tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
