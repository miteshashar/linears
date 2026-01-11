#!/bin/bash
# Test verbose flag

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing verbose flag..."
PASS=0
FAIL=0

# Test 1: Verbose shows Query:
echo -n "Test 1: Verbose shows 'Query:' output... "
output=$(./target/debug/linears --verbose --out json list issue --first 1 2>&1)
if echo "$output" | grep -q "Query:"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 2: Verbose shows Variables:
echo -n "Test 2: Verbose shows 'Variables:' output... "
if echo "$output" | grep -q "Variables:"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 3: Query contains expected keywords
echo -n "Test 3: Query contains expected keywords... "
if echo "$output" | grep -q "query\|issues\|nodes"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 4: Short flag -v also works
echo -n "Test 4: Short flag -v also works... "
short_output=$(./target/debug/linears -v --out json list issue --first 1 2>&1)
if echo "$short_output" | grep -q "Query:"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $short_output"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All verbose tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
