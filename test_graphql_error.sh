#!/bin/bash
# Test GraphQL error exit code

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing GraphQL error handling..."
PASS=0
FAIL=0

# Test 1: GraphQL error returns exit code 4
echo -n "Test 1: GraphQL error returns exit code 4... "
output=$(./target/debug/linears get issue nonexistent-uuid-12345 2>&1)
exit_code=$?
if [ "$exit_code" -eq 4 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $exit_code, expected 4)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 2: Error message contains meaningful info
echo -n "Test 2: Error message contains GraphQL error details... "
if echo "$output" | grep -qi "error\|not found\|invalid"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All GraphQL error tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
