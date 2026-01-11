#!/bin/bash
# Test invalid filter JSON handling

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing invalid filter JSON handling..."
PASS=0
FAIL=0

# Test 1: Invalid filter JSON returns non-zero exit code
echo -n "Test 1: Invalid filter JSON returns non-zero exit code... "
output=$(./target/debug/linears list issue --filter '{bad json}' 2>&1)
exit_code=$?
if [ "$exit_code" -ne 0 ]; then
    echo "PASS (exit code $exit_code)"
    ((PASS++))
else
    echo "FAIL (exit code 0)"
    ((FAIL++))
fi

# Test 2: Error message mentions filter/parse error
echo -n "Test 2: Error message mentions filter/parse error... "
if echo "$output" | grep -qi "filter\|parse\|json\|invalid\|error"; then
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
    echo "All invalid filter tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
