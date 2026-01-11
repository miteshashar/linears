#!/bin/bash
# Test JSON error output envelope format

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing error JSON output envelope format..."
PASS=0
FAIL=0

# Test 1: Error output is valid JSON
echo -n "Test 1: Error output is valid JSON... "
output=$(./target/debug/linears --out json get issue nonexistent-id 2>&1)
if echo "$output" | python3 -c "import sys,json; json.load(sys.stdin)" 2>/dev/null; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 2: Error has 'error' object
echo -n "Test 2: Error has 'error' object... "
if echo "$output" | grep -q '"error"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 3: Error has 'kind' field
echo -n "Test 3: Error has 'kind' field... "
if echo "$output" | grep -q '"kind"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 4: Error has 'message' field
echo -n "Test 4: Error has 'message' field... "
if echo "$output" | grep -q '"message"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 5: Kind is 'graphql' for GraphQL errors
echo -n "Test 5: Kind is 'graphql' for GraphQL errors... "
if echo "$output" | grep -q '"kind":"graphql"' || echo "$output" | grep -q '"kind": "graphql"'; then
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
    echo "All error envelope tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
