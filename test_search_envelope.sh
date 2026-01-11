#!/bin/bash
# Test JSON output envelope format for search command

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing search JSON output envelope format..."
PASS=0
FAIL=0

# Test 1: Search has resource field
echo -n "Test 1: Search has 'resource' field... "
output=$(./target/debug/linears --out json search issue test 2>&1)
if echo "$output" | grep -q '"resource"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 2: Search has operation field
echo -n "Test 2: Search has 'operation' field... "
if echo "$output" | grep -q '"operation"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 3: Search has strategy field
echo -n "Test 3: Search has 'strategy' field... "
if echo "$output" | grep -q '"strategy"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 4: Search has nodes field
echo -n "Test 4: Search has 'nodes' field... "
if echo "$output" | grep -q '"nodes"'; then
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
    echo "All search envelope tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
