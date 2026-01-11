#!/bin/bash
# Test invalid JSON input handling

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing invalid JSON input handling..."
PASS=0
FAIL=0

# Test 1: Invalid JSON returns non-zero exit code
echo -n "Test 1: Invalid JSON returns non-zero exit code... "
output=$(./target/debug/linears create issue --input '{invalid json}' 2>&1)
exit_code=$?
if [ "$exit_code" -ne 0 ]; then
    echo "PASS (exit code $exit_code)"
    ((PASS++))
else
    echo "FAIL (exit code 0)"
    ((FAIL++))
fi

# Test 2: Error message mentions parse error
echo -n "Test 2: Error message mentions parse/JSON error... "
if echo "$output" | grep -qi "parse\|json\|invalid\|error"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 3: Empty input also fails gracefully
echo -n "Test 3: Missing input shows helpful error... "
output2=$(./target/debug/linears create issue 2>&1)
exit_code2=$?
if [ "$exit_code2" -ne 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output2"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All invalid JSON tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
