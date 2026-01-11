#!/bin/bash
# Test invalid YAML input handling

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing invalid YAML input handling..."
PASS=0
FAIL=0

# Test 1: Invalid YAML returns non-zero exit code
echo -n "Test 1: Invalid YAML returns non-zero exit code... "
output=$(./target/debug/linears create issue --input 'title: @invalid: yaml: :bad' 2>&1)
exit_code=$?
if [ "$exit_code" -ne 0 ]; then
    echo "PASS (exit code $exit_code)"
    ((PASS++))
else
    echo "FAIL (exit code 0)"
    ((FAIL++))
fi

# Test 2: Error message mentions parse error
echo -n "Test 2: Error message mentions parse error... "
if echo "$output" | grep -qi "parse\|yaml\|invalid\|error\|expected"; then
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
    echo "All invalid YAML tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
