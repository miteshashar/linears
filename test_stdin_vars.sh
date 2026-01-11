#!/bin/bash
# Test stdin vars for mutate

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing stdin vars for mutate..."
PASS=0
FAIL=0

# Get team ID
team_output=$(./target/debug/linears --out json list team 2>&1)
team_id=$(echo "$team_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')

if [ -z "$team_id" ]; then
    echo "No team found, cannot run stdin vars tests"
    exit 1
fi

echo "Using team ID: $team_id"

# Test 1: Mutate with stdin vars
echo -n "Test 1: Mutate with stdin vars... "
vars_output=$(echo "{\"input\":{\"title\":\"Stdin Vars Test $(date +%s)\",\"teamId\":\"$team_id\"}}" | ./target/debug/linears --out json mutate issueCreate --vars - 2>&1)
vars_exit=$?
vars_id=$(echo "$vars_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')
if [ "$vars_exit" -eq 0 ] && [ -n "$vars_id" ]; then
    echo "PASS (created $vars_id)"
    ((PASS++))
else
    echo "FAIL (exit code $vars_exit)"
    echo "Output: $vars_output"
    ((FAIL++))
fi

# Test 2: Empty vars from stdin
echo -n "Test 2: Empty vars handled... "
empty_output=$(echo "" | ./target/debug/linears --out json mutate issueCreate --vars - 2>&1)
empty_exit=$?
if [ "$empty_exit" -ne 139 ]; then
    echo "PASS (exit code $empty_exit)"
    ((PASS++))
else
    echo "FAIL (crash)"
    ((FAIL++))
fi

# Cleanup
echo -n "Cleaning up... "
if [ -n "$vars_id" ]; then
    ./target/debug/linears delete issue "$vars_id" > /dev/null 2>&1
fi
echo "Done"

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All stdin vars tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
