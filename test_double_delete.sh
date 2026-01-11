#!/bin/bash
# Test double delete handling

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing double delete handling..."
PASS=0
FAIL=0

# Get team ID
team_output=$(./target/debug/linears --out json list team 2>&1)
team_id=$(echo "$team_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')

if [ -z "$team_id" ]; then
    echo "No team found, cannot run delete tests"
    exit 1
fi

# Create a test issue
unique_title="Double Delete Test $(date +%s)"
echo "Creating test issue with title: $unique_title"

create_output=$(./target/debug/linears --out json create issue --input "{\"title\":\"$unique_title\",\"teamId\":\"$team_id\"}" 2>&1)
issue_id=$(echo "$create_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')

if [ -z "$issue_id" ]; then
    echo "Failed to create test issue"
    echo "Output: $create_output"
    exit 1
fi

echo "Created issue: $issue_id"

# Test 1: First delete succeeds
echo -n "Test 1: First delete succeeds... "
delete_output1=$(./target/debug/linears --out json delete issue "$issue_id" 2>&1)
exit_code1=$?
if [ "$exit_code1" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $exit_code1)"
    echo "Output: $delete_output1"
    ((FAIL++))
fi

# Test 2: Second delete fails with appropriate error (not crash)
echo -n "Test 2: Second delete handled gracefully (no crash)... "
delete_output2=$(./target/debug/linears --out json delete issue "$issue_id" 2>&1)
exit_code2=$?
# Should get exit code 4 (GraphQL error) not 139 (SIGSEGV)
if [ "$exit_code2" -ne 139 ]; then
    echo "PASS (exit code $exit_code2)"
    ((PASS++))
else
    echo "FAIL (crash, exit code $exit_code2)"
    ((FAIL++))
fi

# Test 3: Second delete returns success (Linear delete is idempotent)
echo -n "Test 3: Second delete returns success (idempotent)... "
# Linear's delete is idempotent - returns success even for already-deleted items
if echo "$delete_output2" | grep -q '"success":true'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $delete_output2"
    ((FAIL++))
fi

# Test 4: No panic in output
echo -n "Test 4: No panic in output... "
if ! echo "$delete_output2" | grep -q "panic"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (panic detected)"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All double delete tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
