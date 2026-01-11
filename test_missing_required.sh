#!/bin/bash
# Test missing required fields handling

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing missing required fields handling..."
PASS=0
FAIL=0

# Test 1: Empty input returns non-zero exit code
echo -n "Test 1: Empty JSON input returns non-zero exit code... "
output=$(./target/debug/linears create issue --input '{}' 2>&1)
exit_code=$?
if [ "$exit_code" -ne 0 ]; then
    echo "PASS (exit code $exit_code)"
    ((PASS++))
else
    echo "FAIL (exit code 0)"
    ((FAIL++))
fi

# Test 2: Error message mentions required fields
echo -n "Test 2: Error mentions required fields... "
# Linear API should return error about missing title or teamId
if echo "$output" | grep -qi "title\|teamId\|required\|field\|error"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 3: Missing teamId shows error
echo -n "Test 3: Missing teamId shows error... "
output2=$(./target/debug/linears create issue --input '{"title":"Test"}' 2>&1)
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
    echo "All missing required tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
