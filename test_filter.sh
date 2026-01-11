#!/bin/bash
# Test filter functionality

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing filter functionality..."
PASS=0
FAIL=0

# Get team ID first
team_output=$(./target/debug/linears --out json list team 2>&1)
team_id=$(echo "$team_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"\([^"]*\)"/\1/')

if [ -z "$team_id" ]; then
    echo "No team found, skipping filter tests"
    exit 0
fi

# Test 1: Filter by team ID works
echo -n "Test 1: Filter by team ID works... "
output=$(./target/debug/linears --out json list issue --filter "{\"team\":{\"id\":{\"eq\":\"$team_id\"}}}" 2>&1)
exit_code=$?
if [ "$exit_code" -eq 0 ] && echo "$output" | grep -q "nodes"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $exit_code)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 2: Filter with verbose shows filter in query
echo -n "Test 2: Verbose shows filter in query... "
output=$(./target/debug/linears -v --out json list issue --filter '{"priority":{"lte":2}}' 2>&1)
if echo "$output" | grep -q "filter"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (filter not in verbose output)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 3: Priority filter works
echo -n "Test 3: Priority filter works... "
output=$(./target/debug/linears --out json list issue --filter '{"priority":{"lte":2}}' 2>&1)
exit_code=$?
if [ "$exit_code" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $exit_code)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 4: Filter returns pageInfo
echo -n "Test 4: Filter returns pageInfo... "
output=$(./target/debug/linears --out json list issue --filter '{"priority":{"lte":2}}' 2>&1)
if echo "$output" | grep -q "pageInfo"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (no pageInfo)"
    echo "Output: $output"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All filter tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
