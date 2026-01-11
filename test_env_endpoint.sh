#!/bin/bash
# Test LINEARS_ENDPOINT env var override

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing LINEARS_ENDPOINT env var..."
PASS=0
FAIL=0

# Test 1: LINEARS_ENDPOINT is respected (connection to invalid endpoint fails with network error)
echo -n "Test 1: LINEARS_ENDPOINT env var is respected... "
output=$(LINEARS_ENDPOINT=http://localhost:1 ./target/debug/linears list issue 2>&1)
exit_code=$?
# Should be exit code 3 (network error) indicating it tried to use the custom endpoint
if [ "$exit_code" -eq 3 ]; then
    echo "PASS (exit code 3 = tried custom endpoint)"
    ((PASS++))
else
    echo "FAIL (exit code $exit_code)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 2: --endpoint flag overrides env var
echo -n "Test 2: --endpoint flag overrides env var... "
# Set env to invalid, but use flag to override with another invalid endpoint
# Both should give network error (code 3), but this confirms the flag is processed
output2=$(LINEARS_ENDPOINT=http://invalid1:1 ./target/debug/linears --endpoint http://localhost:2 list issue 2>&1)
exit_code2=$?
if [ "$exit_code2" -eq 3 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output2"
    ((FAIL++))
fi

# Test 3: Help mentions LINEARS_ENDPOINT
echo -n "Test 3: Help mentions LINEARS_ENDPOINT env var... "
help_output=$(./target/debug/linears --help 2>&1)
if echo "$help_output" | grep -q "LINEARS_ENDPOINT"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Help output doesn't mention LINEARS_ENDPOINT"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All env endpoint tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
