#!/bin/bash
# Test order-by option

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing order-by option..."
PASS=0
FAIL=0

# Test 1: Order by createdAt works
echo -n "Test 1: Order by createdAt... "
output_created=$(./target/debug/linears --out json list issue --first 5 --order-by createdAt 2>&1)
exit_code1=$?
if [ "$exit_code1" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $exit_code1)"
    echo "Output: $output_created"
    ((FAIL++))
fi

# Test 2: Order by updatedAt works
echo -n "Test 2: Order by updatedAt... "
output_updated=$(./target/debug/linears --out json list issue --first 5 --order-by updatedAt 2>&1)
exit_code2=$?
if [ "$exit_code2" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $exit_code2)"
    echo "Output: $output_updated"
    ((FAIL++))
fi

# Test 3: Order by priority works
echo -n "Test 3: Order by priority... "
output_priority=$(./target/debug/linears --out json list issue --first 5 --order-by priority 2>&1)
exit_code3=$?
if [ "$exit_code3" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $exit_code3)"
    ((FAIL++))
fi

# Test 4: Invalid order-by field handled gracefully
echo -n "Test 4: Invalid order-by field handled... "
output_invalid=$(./target/debug/linears --out json list issue --first 1 --order-by invalidField 2>&1)
exit_invalid=$?
# Should fail with non-crash exit code
if [ "$exit_invalid" -ne 139 ]; then
    echo "PASS (exit code $exit_invalid)"
    ((PASS++))
else
    echo "FAIL (crash)"
    ((FAIL++))
fi

# Test 5: Order by id works
echo -n "Test 5: Order by id... "
output_id=$(./target/debug/linears --out json list issue --first 5 --order-by id 2>&1)
exit_id=$?
if [ "$exit_id" -ne 139 ]; then
    echo "PASS (exit code $exit_id)"
    ((PASS++))
else
    echo "FAIL (crash)"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All order-by tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
