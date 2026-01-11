#!/bin/bash
# Test empty search term handling

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing empty search term handling..."
PASS=0
FAIL=0

# Test 1: Empty string search doesn't crash
echo -n "Test 1: Empty string search doesn't crash... "
output=$(./target/debug/linears --out json search issue "" 2>&1)
exit_code=$?
# Key: no crash (exit code 139 = SIGSEGV)
if [ "$exit_code" -ne 139 ]; then
    echo "PASS (exit code $exit_code)"
    ((PASS++))
else
    echo "FAIL (crash)"
    ((FAIL++))
fi

# Test 2: No panic in output
echo -n "Test 2: No panic in output... "
if ! echo "$output" | grep -q "panic"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 3: Whitespace-only search doesn't crash
echo -n "Test 3: Whitespace-only search handled... "
ws_output=$(./target/debug/linears --out json search issue "   " 2>&1)
ws_exit=$?
if [ "$ws_exit" -ne 139 ]; then
    echo "PASS (exit code $ws_exit)"
    ((PASS++))
else
    echo "FAIL (crash)"
    ((FAIL++))
fi

# Test 4: Valid search term works
echo -n "Test 4: Valid search term works... "
valid_output=$(./target/debug/linears --out json search issue "test" 2>&1)
valid_exit=$?
if [ "$valid_exit" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $valid_exit)"
    ((FAIL++))
fi

# Test 5: Special characters in search handled
echo -n "Test 5: Special chars in search handled... "
special_output=$(./target/debug/linears --out json search issue "test@#$" 2>&1)
special_exit=$?
if [ "$special_exit" -ne 139 ]; then
    echo "PASS (exit code $special_exit)"
    ((PASS++))
else
    echo "FAIL (crash)"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All empty search tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
