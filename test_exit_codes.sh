#!/bin/bash
# Test exit code behavior for linears CLI

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing exit codes..."
PASS=0
FAIL=0

# Test 1: Missing/empty API key should return exit code 2
echo -n "Test 1: Empty LINEAR_API_KEY returns exit code 2... "
LINEAR_API_KEY= ./target/debug/linears list issue > /dev/null 2>&1
exit_code=$?
if [ "$exit_code" -eq 2 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (expected 2, got $exit_code)"
    ((FAIL++))
fi

# Test 2: Commands that don't require API should work without key
echo -n "Test 2: 'resources' without API key returns exit code 0... "
LINEAR_API_KEY= ./target/debug/linears resources > /dev/null 2>&1
exit_code=$?
if [ "$exit_code" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (expected 0, got $exit_code)"
    ((FAIL++))
fi

# Test 3: 'ops' without API key should work
echo -n "Test 3: 'ops' without API key returns exit code 0... "
LINEAR_API_KEY= ./target/debug/linears ops > /dev/null 2>&1
exit_code=$?
if [ "$exit_code" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (expected 0, got $exit_code)"
    ((FAIL++))
fi

# Test 4: 'schema info' without API key should work
echo -n "Test 4: 'schema info' without API key returns exit code 0... "
LINEAR_API_KEY= ./target/debug/linears schema info > /dev/null 2>&1
exit_code=$?
if [ "$exit_code" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (expected 0, got $exit_code)"
    ((FAIL++))
fi

# Test 5: Error message should mention API key
echo -n "Test 5: Error message mentions LINEAR_API_KEY... "
result=$(LINEAR_API_KEY= ./target/debug/linears list issue 2>&1)
if echo "$result" | grep -q "LINEAR_API_KEY"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (doesn't mention LINEAR_API_KEY)"
    ((FAIL++))
fi

# Test 6: get command without API key returns exit code 2
echo -n "Test 6: 'get' without API key returns exit code 2... "
LINEAR_API_KEY= ./target/debug/linears get issue abc-123 > /dev/null 2>&1
exit_code=$?
if [ "$exit_code" -eq 2 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (expected 2, got $exit_code)"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
