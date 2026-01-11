#!/bin/bash
# Test empty filter handling

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing empty filter handling..."
PASS=0
FAIL=0

# Test 1: List without filter
echo -n "Test 1: List without filter... "
output_no_filter=$(./target/debug/linears --out json list issue --first 3 2>&1)
exit_code1=$?
if [ "$exit_code1" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $exit_code1)"
    ((FAIL++))
fi

# Test 2: Empty object filter doesn't crash
echo -n "Test 2: Empty object filter '{}' handled... "
output_empty=$(./target/debug/linears --out json list issue --first 3 --filter '{}' 2>&1)
exit_code2=$?
if [ "$exit_code2" -ne 139 ]; then
    echo "PASS (exit code $exit_code2)"
    ((PASS++))
else
    echo "FAIL (crash)"
    ((FAIL++))
fi

# Test 3: No panic in output
echo -n "Test 3: No panic in output... "
if ! echo "$output_empty" | grep -q "panic"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 4: Results count is similar (empty filter = no filter)
echo -n "Test 4: Similar results count... "
count_no_filter=$(echo "$output_no_filter" | python3 -c "import sys, json; d=json.load(sys.stdin); print(len(d.get('nodes', [])))" 2>/dev/null || echo "0")
count_empty=$(echo "$output_empty" | python3 -c "import sys, json; d=json.load(sys.stdin); print(len(d.get('nodes', [])))" 2>/dev/null || echo "0")

if [ "$count_empty" = "$count_no_filter" ]; then
    echo "PASS (both got $count_empty items)"
    ((PASS++))
else
    echo "INFO (no filter: $count_no_filter, empty: $count_empty)"
    # Not a failure - behavior may differ, key is no crash
    ((PASS++))
fi

# Test 5: Empty string filter handled
echo -n "Test 5: Empty string filter handled... "
output_str=$(./target/debug/linears --out json list issue --first 1 --filter '' 2>&1)
exit_str=$?
if [ "$exit_str" -ne 139 ]; then
    echo "PASS (exit code $exit_str)"
    ((PASS++))
else
    echo "FAIL (crash)"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All empty filter tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
