#!/bin/bash
# Test identifier format for ID parameter

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing identifier ID parameter..."
PASS=0
FAIL=0

# Get a valid issue identifier first
list_output=$(./target/debug/linears --out json list issue --first 1 2>&1)
issue_identifier=$(echo "$list_output" | grep -o '"identifier":"[^"]*"' | head -1 | sed 's/"identifier":"//;s/"//')

if [ -z "$issue_identifier" ]; then
    echo "No issue identifier found, cannot run identifier tests"
    exit 1
fi

echo "Found issue identifier: $issue_identifier"

# Test 1: Get issue by identifier returns success
echo -n "Test 1: Get issue by identifier returns success... "
output=$(./target/debug/linears --out json get issue "$issue_identifier" 2>&1)
exit_code=$?
if [ "$exit_code" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $exit_code)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 2: Returned entity has matching identifier
echo -n "Test 2: Returned entity has matching identifier... "
if echo "$output" | grep -q "$issue_identifier"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 3: Auto-detection worked (identifier is in response)
echo -n "Test 3: Response contains identifier field... "
if echo "$output" | grep -q '"identifier"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All identifier tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
