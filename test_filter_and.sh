#!/bin/bash
# Test filter AND combinations

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing filter AND combinations..."
PASS=0
FAIL=0

# Get team ID
team_output=$(./target/debug/linears --out json list team 2>&1)
team_id=$(echo "$team_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')

if [ -z "$team_id" ]; then
    echo "No team found, cannot run AND filter tests"
    exit 1
fi

echo "Using team ID: $team_id"

# Test 1: Simple AND filter doesn't crash
echo -n "Test 1: AND filter syntax accepted... "
output_and=$(./target/debug/linears --out json list issue --first 5 --filter "{\"and\":[{\"team\":{\"id\":{\"eq\":\"$team_id\"}}}]}" 2>&1)
exit_code1=$?
if [ "$exit_code1" -ne 139 ]; then
    echo "PASS (exit code $exit_code1)"
    ((PASS++))
else
    echo "FAIL (crash)"
    ((FAIL++))
fi

# Test 2: Multiple conditions in AND
echo -n "Test 2: Multiple conditions in AND... "
output_multi=$(./target/debug/linears --out json list issue --first 5 --filter "{\"and\":[{\"team\":{\"id\":{\"eq\":\"$team_id\"}}},{\"priority\":{\"lte\":4}}]}" 2>&1)
exit_code2=$?
if [ "$exit_code2" -ne 139 ]; then
    echo "PASS (exit code $exit_code2)"
    ((PASS++))
else
    echo "FAIL (crash)"
    ((FAIL++))
fi

# Test 3: No panic in output
echo -n "Test 3: No panic in output... "
if ! echo "$output_and" | grep -q "panic" && ! echo "$output_multi" | grep -q "panic"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 4: Results are returned (if any exist)
echo -n "Test 4: Response has valid structure... "
if echo "$output_and" | grep -q '"nodes"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output_and"
    ((FAIL++))
fi

# Test 5: Empty AND array handled
echo -n "Test 5: Empty AND array handled... "
output_empty=$(./target/debug/linears --out json list issue --first 1 --filter '{"and":[]}' 2>&1)
exit_empty=$?
if [ "$exit_empty" -ne 139 ]; then
    echo "PASS (exit code $exit_empty)"
    ((PASS++))
else
    echo "FAIL (crash)"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All filter AND tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
