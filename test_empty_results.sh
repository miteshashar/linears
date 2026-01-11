#!/bin/bash
# Test empty results handling

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing empty results handling..."
PASS=0
FAIL=0

# Test 1: Search with non-matching term returns exit code 0
echo -n "Test 1: Search with non-matching term returns exit code 0... "
output=$(./target/debug/linears search issue "xyznonexistent123456789" 2>&1)
exit_code=$?
if [ "$exit_code" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $exit_code, expected 0)"
    ((FAIL++))
fi

# Test 2: JSON output shows empty nodes array
echo -n "Test 2: JSON output shows empty nodes array... "
json_output=$(./target/debug/linears --out json search issue "xyznonexistent123456789" 2>&1)
if echo "$json_output" | grep -q '"nodes":\[\]' || echo "$json_output" | grep -q '"nodes": \[\]'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $json_output"
    ((FAIL++))
fi

# Test 3: Table output handles empty results gracefully
echo -n "Test 3: Table output handles empty results gracefully... "
table_output=$(./target/debug/linears --out table search issue "xyznonexistent123456789" 2>&1)
exit_code3=$?
if [ "$exit_code3" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $exit_code3)"
    echo "Output: $table_output"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All empty results tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
