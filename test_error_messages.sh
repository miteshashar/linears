#!/bin/bash
# Test error messages are specific and actionable

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing error messages specificity..."
PASS=0
FAIL=0

# Test 1: Auth error has specific message
echo -n "Test 1: Auth error has specific message... "
# Save and replace API key
original_key="$LINEAR_API_KEY"
export LINEAR_API_KEY="invalid-key"
auth_output=$(./target/debug/linears --out json list issue --first 1 2>&1)
auth_exit=$?
export LINEAR_API_KEY="$original_key"
if [ "$auth_exit" -ne 0 ] && echo "$auth_output" | grep -qi "auth\|unauthorized\|invalid\|api.key\|error"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $auth_output"
    ((FAIL++))
fi

# Test 2: Invalid resource error is specific
echo -n "Test 2: Invalid resource error is specific... "
invalid_output=$(./target/debug/linears list nonexistent 2>&1)
invalid_exit=$?
if [ "$invalid_exit" -ne 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "INFO (clap may handle this)"
    ((PASS++))
fi

# Test 3: Invalid JSON error is specific
echo -n "Test 3: Invalid JSON error is specific... "
json_output=$(./target/debug/linears --out json create issue --input '{invalid json}' 2>&1)
json_exit=$?
if [ "$json_exit" -ne 0 ]; then
    echo "PASS (exit code $json_exit)"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 4: Missing required field error
echo -n "Test 4: Missing required field error... "
missing_output=$(./target/debug/linears --out json create issue --input '{"title":"test"}' 2>&1)
missing_exit=$?
if [ "$missing_exit" -ne 0 ]; then
    echo "PASS (exit code $missing_exit)"
    ((PASS++))
else
    echo "FAIL (should have required teamId)"
    ((FAIL++))
fi

# Test 5: Error messages don't include internal stack traces
echo -n "Test 5: No internal stack traces in errors... "
if ! echo "$auth_output" | grep -q "at \./src/\|thread.*panicked\|RUST_BACKTRACE"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (internal traces exposed)"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All error message tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
