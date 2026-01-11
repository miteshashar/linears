#!/bin/bash
# Test mutate universal command

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing mutate command..."
PASS=0
FAIL=0

# Get team ID first
team_output=$(./target/debug/linears --out json list team 2>&1)
team_id=$(echo "$team_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"\([^"]*\)"/\1/')

if [ -z "$team_id" ]; then
    echo "No team found, cannot run mutate tests"
    exit 1
fi

# Test 1: Mutate issueCreate works
echo -n "Test 1: Mutate issueCreate creates issue... "
output=$(./target/debug/linears --out json mutate issueCreate --vars "{\"input\":{\"title\":\"Via mutate $(date +%s)\",\"teamId\":\"$team_id\"}}" 2>&1)
exit_code=$?
issue_id=$(echo "$output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"\([^"]*\)"/\1/')
if [ "$exit_code" -eq 0 ] && echo "$output" | grep -q "success"; then
    echo "PASS (issue_id: $issue_id)"
    ((PASS++))
else
    echo "FAIL (exit code $exit_code)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 2: Cleanup - delete the created issue
if [ -n "$issue_id" ]; then
    echo -n "Test 2: Mutate issueDelete cleans up... "
    delete_output=$(./target/debug/linears --out json mutate issueDelete --vars "{\"id\":\"$issue_id\"}" 2>&1)
    if echo "$delete_output" | grep -q "success"; then
        echo "PASS"
        ((PASS++))
    else
        echo "FAIL"
        echo "Output: $delete_output"
        ((FAIL++))
    fi
else
    echo "Test 2: SKIP (no issue ID to delete)"
    ((PASS++))
fi

# Test 3: Verbose shows query
echo -n "Test 3: Verbose shows the mutation query... "
output=$(./target/debug/linears -v --out json mutate issueCreate --vars "{\"input\":{\"title\":\"Test\",\"teamId\":\"$team_id\"}}" 2>&1)
# Extract issue ID to cleanup
cleanup_id=$(echo "$output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"\([^"]*\)"/\1/')
if echo "$output" | grep -q "Query:"; then
    echo "PASS"
    ((PASS++))
    # Cleanup
    if [ -n "$cleanup_id" ]; then
        ./target/debug/linears --out json mutate issueDelete --vars "{\"id\":\"$cleanup_id\"}" > /dev/null 2>&1
    fi
else
    echo "FAIL (no Query: in verbose output)"
    echo "Output: $output"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All mutate tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
