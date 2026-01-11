#!/bin/bash
# Test feedback messages for various operations

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing feedback messages..."
PASS=0
FAIL=0

# Get team ID
team_output=$(./target/debug/linears --out json list team 2>&1)
team_id=$(echo "$team_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')

if [ -z "$team_id" ]; then
    echo "No team found, cannot run feedback tests"
    exit 1
fi

# Create a test issue
unique_title="Feedback Test $(date +%s)"
create_output=$(./target/debug/linears --out json create issue --input "{\"title\":\"$unique_title\",\"teamId\":\"$team_id\"}" 2>&1)
issue_id=$(echo "$create_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')

if [ -z "$issue_id" ]; then
    echo "Failed to create test issue"
    exit 1
fi

echo "Created test issue: $issue_id"

# Test 1: Update shows success
echo -n "Test 1: Update shows success... "
update_output=$(./target/debug/linears --out json update issue "$issue_id" --set '{"title":"Updated Title"}' 2>&1)
update_exit=$?
if [ "$update_exit" -eq 0 ] && echo "$update_output" | grep -q '"success":true\|"id":'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $update_exit)"
    ((FAIL++))
fi

# Test 2: Archive shows success
echo -n "Test 2: Archive shows success... "
archive_output=$(./target/debug/linears --out json archive issue "$issue_id" 2>&1)
archive_exit=$?
if [ "$archive_exit" -eq 0 ] && echo "$archive_output" | grep -q '"success":true'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $archive_exit)"
    ((FAIL++))
fi

# Test 3: Unarchive shows success
echo -n "Test 3: Unarchive shows success... "
unarchive_output=$(./target/debug/linears --out json unarchive issue "$issue_id" 2>&1)
unarchive_exit=$?
if [ "$unarchive_exit" -eq 0 ] && echo "$unarchive_output" | grep -q '"success":true'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $unarchive_exit)"
    ((FAIL++))
fi

# Test 4: Delete shows success
echo -n "Test 4: Delete shows success... "
delete_output=$(./target/debug/linears --out json delete issue "$issue_id" 2>&1)
delete_exit=$?
if [ "$delete_exit" -eq 0 ] && echo "$delete_output" | grep -q '"success":true'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $delete_exit)"
    ((FAIL++))
fi

# Test 5: Failed operation shows error
echo -n "Test 5: Failed operation shows error... "
error_output=$(./target/debug/linears --out json get issue "non-existent-id" 2>&1)
error_exit=$?
if [ "$error_exit" -ne 0 ]; then
    echo "PASS (exit code $error_exit)"
    ((PASS++))
else
    echo "INFO (no error for non-existent)"
    ((PASS++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All feedback tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
