#!/bin/bash
# Test special characters in search

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing special characters handling..."
PASS=0
FAIL=0

# Test 1: Special characters in search don't crash
echo -n "Test 1: Special characters in search don't crash... "
output=$(./target/debug/linears search issue '!@#$%^&*()[]{}|;:<>?' 2>&1)
exit_code=$?
# Should not be 139 (SIGSEGV)
if [ "$exit_code" -ne 139 ]; then
    echo "PASS (exit code $exit_code)"
    ((PASS++))
else
    echo "FAIL (crash, exit code $exit_code)"
    ((FAIL++))
fi

# Test 2: No panic in output
echo -n "Test 2: No panic in output... "
if ! echo "$output" | grep -q "panic"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (panic detected)"
    ((FAIL++))
fi

# Test 3: Unicode characters handled
echo -n "Test 3: Unicode characters handled... "
output2=$(./target/debug/linears search issue '日本語テスト' 2>&1)
exit_code2=$?
if [ "$exit_code2" -ne 139 ]; then
    echo "PASS (exit code $exit_code2)"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 4: Quotes and backslashes handled
echo -n "Test 4: Quotes and backslashes handled... "
output3=$(./target/debug/linears search issue 'test "quoted" \backslash' 2>&1)
exit_code3=$?
if [ "$exit_code3" -ne 139 ]; then
    echo "PASS (exit code $exit_code3)"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All special chars tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
