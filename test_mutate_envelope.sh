#!/bin/bash
# Test JSON output envelope format for mutate command

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing mutate JSON output envelope format..."
PASS=0
FAIL=0

# Get team ID first
team_output=$(./target/debug/linears --out json list team 2>&1)
team_id=$(echo "$team_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')

if [ -z "$team_id" ]; then
    echo "No team found, cannot run mutate tests"
    exit 1
fi

# Create an issue via mutate
echo -n "Creating test issue... "
output=$(./target/debug/linears --out json mutate issueCreate --vars "{\"input\":{\"title\":\"Mutate Envelope Test $(date +%s)\",\"teamId\":\"$team_id\"}}" 2>&1)
issue_id=$(echo "$output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')

# Test 1: Mutate has 'op' field
echo -n "Test 1: Mutate has 'op' field... "
if echo "$output" | grep -q '"op"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 2: Mutate has 'operation' field
echo -n "Test 2: Mutate has 'operation' field... "
if echo "$output" | grep -q '"operation"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 3: Mutate has 'result' field
echo -n "Test 3: Mutate has 'result' field... "
if echo "$output" | grep -q '"result"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 4: Result has 'success' field
echo -n "Test 4: Result has 'success' field... "
if echo "$output" | grep -q '"success"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Cleanup - delete the issue
if [ -n "$issue_id" ]; then
    echo -n "Cleaning up... "
    ./target/debug/linears --out json delete issue "$issue_id" > /dev/null 2>&1
    echo "Done"
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All mutate envelope tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
