#!/bin/bash
# Test UUID format for ID parameter

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing UUID ID parameter..."
PASS=0
FAIL=0

# Get a valid issue UUID first
list_output=$(./target/debug/linears --out json list issue --first 1 2>&1)
issue_uuid=$(echo "$list_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')

if [ -z "$issue_uuid" ]; then
    echo "No issue found, cannot run UUID tests"
    exit 1
fi

echo "Found issue UUID: $issue_uuid"

# Test 1: Get issue by UUID returns success
echo -n "Test 1: Get issue by UUID returns success... "
output=$(./target/debug/linears --out json get issue "$issue_uuid" 2>&1)
exit_code=$?
if [ "$exit_code" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $exit_code)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 2: Returned entity has matching UUID
echo -n "Test 2: Returned entity has matching UUID... "
if echo "$output" | grep -q "$issue_uuid"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 3: Entity has expected fields
echo -n "Test 3: Entity has expected fields (title, id)... "
if echo "$output" | grep -q '"title"' && echo "$output" | grep -q '"id"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All UUID tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
