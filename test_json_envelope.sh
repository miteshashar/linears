#!/bin/bash
# Test JSON output envelope format

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing JSON output envelope format..."
PASS=0
FAIL=0

# Test 1: List has resource field
echo -n "Test 1: List has 'resource' field... "
output=$(./target/debug/linears --out json list issue --first 1 2>&1)
if echo "$output" | grep -q '"resource"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 2: List resource equals 'issue'
echo -n "Test 2: Resource equals 'issue'... "
if echo "$output" | grep -q '"resource":"issue"' || echo "$output" | grep -q '"resource": "issue"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 3: List has operation field
echo -n "Test 3: List has 'operation' field... "
if echo "$output" | grep -q '"operation"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 4: List has nodes array
echo -n "Test 4: List has 'nodes' array... "
if echo "$output" | grep -q '"nodes"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 5: List has pageInfo
echo -n "Test 5: List has 'pageInfo'... "
if echo "$output" | grep -q '"pageInfo"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 6: pageInfo has hasNextPage
echo -n "Test 6: pageInfo has 'hasNextPage'... "
if echo "$output" | grep -q '"hasNextPage"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 7: pageInfo has endCursor
echo -n "Test 7: pageInfo has 'endCursor'... "
if echo "$output" | grep -q '"endCursor"'; then
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
    echo "All JSON envelope tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
