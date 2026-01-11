#!/bin/bash
# Test ops command

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing ops command..."
PASS=0
FAIL=0

# Test 1: Ops command runs successfully
echo -n "Test 1: Ops command runs... "
output=$(./target/debug/linears ops 2>&1)
exit_code=$?
if [ "$exit_code" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $exit_code)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 2: Output includes common ops
echo -n "Test 2: Output includes issueCreate, issueUpdate, issueDelete... "
if echo "$output" | grep -q "issueCreate" && echo "$output" | grep -q "issueUpdate" && echo "$output" | grep -q "issueDelete"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 3: JSON output works
echo -n "Test 3: JSON output works... "
json_output=$(./target/debug/linears --out json ops 2>&1)
if echo "$json_output" | grep -q '"operations"' && echo "$json_output" | grep -q '"count"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $json_output"
    ((FAIL++))
fi

# Test 4: JSON output includes common ops
echo -n "Test 4: JSON output includes common ops... "
if echo "$json_output" | grep -q '"issueCreate"' && echo "$json_output" | grep -q '"issueUpdate"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $json_output"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All ops tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
