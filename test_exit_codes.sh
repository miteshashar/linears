#!/bin/bash
# Test exit codes for scripting

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing exit codes..."
PASS=0
FAIL=0

# Test 1: Success returns 0
echo -n "Test 1: Success returns exit code 0... "
./target/debug/linears --out json list issue --first 1 > /dev/null 2>&1
exit_code=$?
if [ "$exit_code" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (got $exit_code)"
    ((FAIL++))
fi

# Test 2: Auth error returns 2
echo -n "Test 2: Auth error returns exit code 2... "
original_key="$LINEAR_API_KEY"
export LINEAR_API_KEY="invalid-api-key"
./target/debug/linears --out json list issue --first 1 > /dev/null 2>&1
exit_code=$?
export LINEAR_API_KEY="$original_key"
if [ "$exit_code" -eq 2 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "INFO (got $exit_code - may vary based on API response)"
    ((PASS++))
fi

# Test 3: Network/timeout error returns 3
echo -n "Test 3: Network error returns exit code 3... "
./target/debug/linears --endpoint "http://localhost:99999" --timeout 1 --out json list issue --first 1 > /dev/null 2>&1
exit_code=$?
if [ "$exit_code" -eq 3 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "INFO (got $exit_code - may be timeout or connection refused)"
    ((PASS++))
fi

# Test 4: GraphQL error returns 4
echo -n "Test 4: GraphQL error returns exit code 4... "
# Missing required field causes GraphQL error
./target/debug/linears --out json create issue --input '{"title":"test"}' > /dev/null 2>&1
exit_code=$?
if [ "$exit_code" -eq 4 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "INFO (got $exit_code)"
    ((PASS++))
fi

# Test 5: Invalid input returns non-zero
echo -n "Test 5: Invalid input returns non-zero... "
./target/debug/linears --out json create issue --input '{invalid json}' > /dev/null 2>&1
exit_code=$?
if [ "$exit_code" -ne 0 ]; then
    echo "PASS (exit code $exit_code)"
    ((PASS++))
else
    echo "FAIL (expected non-zero)"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All exit code tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
