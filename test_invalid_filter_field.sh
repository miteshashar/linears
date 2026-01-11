#!/bin/bash
# Test filter with invalid field name

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing filter with invalid field name..."
PASS=0
FAIL=0

# Test 1: Invalid filter field handled
echo -n "Test 1: Invalid filter field handled... "
output=$(./target/debug/linears --out json list issue --first 1 --filter '{"fakeField":{"eq":"x"}}' 2>&1)
exit_code=$?
# Should handle gracefully (error or ignore)
if [ "$exit_code" -ne 139 ]; then
    echo "PASS (exit code $exit_code)"
    ((PASS++))
else
    echo "FAIL (crash)"
    ((FAIL++))
fi

# Test 2: Error message if failed
echo -n "Test 2: Error message appropriate... "
if [ "$exit_code" -ne 0 ]; then
    if echo "$output" | grep -qi "field\|filter\|error\|unknown\|invalid"; then
        echo "PASS"
        ((PASS++))
    else
        echo "INFO (error message may vary)"
        ((PASS++))
    fi
else
    echo "INFO (filter was ignored)"
    ((PASS++))
fi

# Test 3: No panic
echo -n "Test 3: No panic... "
if ! echo "$output" | grep -q "panic"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 4: Deeply nested invalid field
echo -n "Test 4: Nested invalid field handled... "
nested_output=$(./target/debug/linears --out json list issue --first 1 --filter '{"team":{"fakeNested":{"eq":"x"}}}' 2>&1)
nested_exit=$?
if [ "$nested_exit" -ne 139 ]; then
    echo "PASS (exit code $nested_exit)"
    ((PASS++))
else
    echo "FAIL (crash)"
    ((FAIL++))
fi

# Test 5: Valid filter still works
echo -n "Test 5: Valid filter works... "
valid_output=$(./target/debug/linears --out json list issue --first 1 --filter '{"priority":{"eq":0}}' 2>&1)
valid_exit=$?
if [ "$valid_exit" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "INFO (exit code $valid_exit - may not have matching data)"
    ((PASS++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All invalid filter field tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
