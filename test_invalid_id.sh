#!/bin/bash
# Test invalid ID handling

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing invalid ID handling..."
PASS=0
FAIL=0

# Test 1: Invalid ID doesn't crash (non-zero exit, no panic)
echo -n "Test 1: Invalid ID doesn't crash... "
output=$(./target/debug/linears get issue 'not-a-uuid-or-identifier' 2>&1)
exit_code=$?
if [ "$exit_code" -ne 0 ]; then
    # No crash, graceful exit
    echo "PASS (exit code $exit_code)"
    ((PASS++))
else
    echo "FAIL (expected non-zero exit code)"
    ((FAIL++))
fi

# Test 2: Error message is shown
echo -n "Test 2: Error message is shown... "
if echo "$output" | grep -qi "error\|not found"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 3: No panic message in output
echo -n "Test 3: No panic in output... "
if ! echo "$output" | grep -q "panic"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (panic detected)"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All invalid ID tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
