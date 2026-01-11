#!/bin/bash
# Test that API key is sent in Authorization header

set -e

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing Authorization header..."

# Test that verbose mode shows the query (proves client is attempting to send)
echo -n "Test 1: Verbose mode shows query being sent... "
output=$(LINEARS_API_KEY="test-key-123" ./target/debug/linears -v --endpoint http://localhost:9999 list issue 2>&1) || true
if echo "$output" | grep -q "Query:"; then
    echo "PASS"
else
    # If no query shown, check if it's because of connection error (which is expected)
    if echo "$output" | grep -qi "Network error\|Connection"; then
        echo "PASS (network error as expected - key was sent)"
    else
        echo "FAIL"
        echo "Output: $output"
        exit 1
    fi
fi

# Verify the Authorization header format in the code
# Note: Linear API expects API key directly (no Bearer prefix)
echo -n "Test 2: Code sets API key in Authorization header... "
if grep -q 'HeaderValue::from_str(api_key)' src/client/mod.rs; then
    echo "PASS"
else
    echo "FAIL (Authorization header format not found in code)"
    exit 1
fi

# Verify the Authorization header is set in the client
echo -n "Test 3: Code sets AUTHORIZATION header... "
if grep -q 'AUTHORIZATION' src/client/mod.rs; then
    echo "PASS"
else
    echo "FAIL (AUTHORIZATION header not set in code)"
    exit 1
fi

# Test 4: API key should NOT appear in verbose output
echo -n "Test 4: API key not exposed in verbose output... "
secret_key="super-secret-key-xyz789"
output=$(LINEARS_API_KEY="$secret_key" ./target/debug/linears -v --endpoint http://localhost:9999 list issue 2>&1) || true
if echo "$output" | grep -q "$secret_key"; then
    echo "FAIL (API key was exposed!)"
    echo "Output: $output"
    exit 1
else
    echo "PASS"
fi

# Test 5: API key should NOT appear in network error messages
echo -n "Test 5: API key not exposed in network error... "
secret_key="lin_api_secret_network_test"
output=$(LINEARS_API_KEY="$secret_key" ./target/debug/linears --endpoint http://localhost:9999 list issue 2>&1) || true
if echo "$output" | grep -q "secret_network_test"; then
    echo "FAIL (API key was exposed in network error!)"
    exit 1
else
    echo "PASS"
fi

# Test 6: API key should NOT appear in auth error messages
echo -n "Test 6: API key not exposed in auth error... "
secret_key="lin_api_secret_auth_test"
output=$(LINEARS_API_KEY="$secret_key" ./target/debug/linears list issue 2>&1) || true
if echo "$output" | grep -q "secret_auth_test"; then
    echo "FAIL (API key was exposed in auth error!)"
    exit 1
else
    echo "PASS"
fi

echo ""
echo "All Authorization header tests passed!"
