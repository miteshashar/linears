#!/bin/bash
# Test resources command

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing resources command..."
PASS=0
FAIL=0

# Test 1: Resources command runs successfully
echo -n "Test 1: Resources command runs... "
output=$(./target/debug/linears resources 2>&1)
exit_code=$?
if [ "$exit_code" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $exit_code)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 2: Output includes common resources
echo -n "Test 2: Output includes issue, team, user, project... "
if echo "$output" | grep -q "issue" && echo "$output" | grep -q "team" && echo "$output" | grep -q "user" && echo "$output" | grep -q "project"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 3: JSON output works
echo -n "Test 3: JSON output works... "
json_output=$(./target/debug/linears --out json resources 2>&1)
if echo "$json_output" | grep -q '"resources"' && echo "$json_output" | grep -q '"count"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $json_output"
    ((FAIL++))
fi

# Test 4: JSON output includes common resources
echo -n "Test 4: JSON output includes common resources... "
if echo "$json_output" | grep -q '"issue"' && echo "$json_output" | grep -q '"team"'; then
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
    echo "All resources tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
