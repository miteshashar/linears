#!/bin/bash
# Test very long input handling

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing very long input handling..."
PASS=0
FAIL=0

# Generate a 10KB string
long_string=$(python3 -c "print('a' * 10240)")

# Test 1: Long filter doesn't crash (timeout 10 seconds)
echo -n "Test 1: Long filter input doesn't crash... "
output=$(timeout 10 ./target/debug/linears list issue --filter "$long_string" 2>&1)
exit_code=$?
# Exit code can be non-zero (error) but should not be 139 (SIGSEGV) or 124 (timeout)
if [ "$exit_code" -ne 139 ] && [ "$exit_code" -ne 124 ]; then
    echo "PASS (exit code $exit_code)"
    ((PASS++))
else
    echo "FAIL (crash or timeout, exit code $exit_code)"
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

# Test 3: Long search term doesn't crash
echo -n "Test 3: Long search term doesn't crash... "
long_search=$(python3 -c "print('test' * 2560)")
output2=$(timeout 10 ./target/debug/linears search issue "$long_search" 2>&1)
exit_code2=$?
if [ "$exit_code2" -ne 139 ] && [ "$exit_code2" -ne 124 ]; then
    echo "PASS (exit code $exit_code2)"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All long input tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
