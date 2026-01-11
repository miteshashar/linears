#!/bin/bash
# Test list team command functionality

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing list team functionality..."
PASS=0
FAIL=0

# Test 1: List team requires API key
echo -n "Test 1: List team without API key returns exit code 2... "
LINEAR_API_KEY= ./target/debug/linears list team > /dev/null 2>&1
exit_code=$?
if [ "$exit_code" -eq 2 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (expected 2, got $exit_code)"
    ((FAIL++))
fi

# Test 2: List team with valid API key returns data (exit code 0)
echo -n "Test 2: List team with valid API key returns exit code 0... "
output=$(./target/debug/linears --out json list team 2>&1)
exit_code=$?
if [ "$exit_code" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (expected 0, got $exit_code)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 3: List team returns JSON with teams field
echo -n "Test 3: List team returns JSON with teams field... "
output=$(./target/debug/linears --out json list team 2>&1)
if echo "$output" | grep -q "teams"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (no teams in output)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 4: Teams have nodes array
echo -n "Test 4: Teams response has nodes array... "
output=$(./target/debug/linears --out json list team 2>&1)
if echo "$output" | grep -q "nodes"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (no nodes in output)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 5: Teams have name field
echo -n "Test 5: Teams have name field... "
output=$(./target/debug/linears --out json list team 2>&1)
if echo "$output" | grep -q '"name"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (no name field in output)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 6: Teams have key field
echo -n "Test 6: Teams have key field... "
output=$(./target/debug/linears --out json list team 2>&1)
if echo "$output" | grep -q '"key"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (no key field in output)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 7: Response has pageInfo
echo -n "Test 7: Response has pageInfo... "
output=$(./target/debug/linears --out json list team 2>&1)
if echo "$output" | grep -q "pageInfo"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (no pageInfo in output)"
    echo "Output: $output"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All list team tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
