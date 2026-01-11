#!/bin/bash
# Test list user command functionality

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing list user functionality..."
PASS=0
FAIL=0

# Test 1: List user requires API key
echo -n "Test 1: List user without API key returns exit code 2... "
LINEARS_API_KEY= ./target/debug/linears list user > /dev/null 2>&1
exit_code=$?
if [ "$exit_code" -eq 2 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (expected 2, got $exit_code)"
    ((FAIL++))
fi

# Test 2: List user with valid API key returns data (exit code 0)
echo -n "Test 2: List user with valid API key returns exit code 0... "
output=$(./target/debug/linears --out json list user 2>&1)
exit_code=$?
if [ "$exit_code" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (expected 0, got $exit_code)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 3: List user returns JSON with users field
echo -n "Test 3: List user returns JSON with users field... "
output=$(./target/debug/linears --out json list user 2>&1)
if echo "$output" | grep -q "users"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (no users in output)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 4: Users have nodes array
echo -n "Test 4: Users response has nodes array... "
output=$(./target/debug/linears --out json list user 2>&1)
if echo "$output" | grep -q "nodes"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (no nodes in output)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 5: Users have name field
echo -n "Test 5: Users have name field... "
output=$(./target/debug/linears --out json list user 2>&1)
if echo "$output" | grep -q '"name"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (no name field in output)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 6: Users have email field
echo -n "Test 6: Users have email field... "
output=$(./target/debug/linears --out json list user 2>&1)
if echo "$output" | grep -q '"email"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (no email field in output)"
    echo "Output: $output"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All list user tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
