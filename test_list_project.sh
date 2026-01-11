#!/bin/bash
# Test list project command functionality

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing list project functionality..."
PASS=0
FAIL=0

# Test 1: List project requires API key
echo -n "Test 1: List project without API key returns exit code 2... "
LINEARS_API_KEY= ./target/debug/linears list project > /dev/null 2>&1
exit_code=$?
if [ "$exit_code" -eq 2 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (expected 2, got $exit_code)"
    ((FAIL++))
fi

# Test 2: List project with valid API key returns data (exit code 0)
echo -n "Test 2: List project with valid API key returns exit code 0... "
output=$(./target/debug/linears --out json list project 2>&1)
exit_code=$?
if [ "$exit_code" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (expected 0, got $exit_code)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 3: List project returns JSON with projects field
echo -n "Test 3: List project returns JSON with projects field... "
output=$(./target/debug/linears --out json list project 2>&1)
if echo "$output" | grep -q "projects"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (no projects in output)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 4: Projects have nodes array
echo -n "Test 4: Projects response has nodes array... "
output=$(./target/debug/linears --out json list project 2>&1)
if echo "$output" | grep -q "nodes"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (no nodes in output)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 5: Projects have name field (or empty nodes)
echo -n "Test 5: Projects have name field (or empty nodes)... "
output=$(./target/debug/linears --out json list project 2>&1)
if echo "$output" | grep -q '"name"' || echo "$output" | grep -q '"nodes":\[\]'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (no name field or empty nodes in output)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 6: Projects have state field (or empty nodes)
echo -n "Test 6: Projects have state field (or empty nodes)... "
output=$(./target/debug/linears --out json list project 2>&1)
if echo "$output" | grep -q '"state"' || echo "$output" | grep -q '"nodes":\[\]'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (no state field or empty nodes in output)"
    echo "Output: $output"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All list project tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
