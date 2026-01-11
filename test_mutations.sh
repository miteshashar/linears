#!/bin/bash
# Test mutation commands (create, update, delete)

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing mutation commands..."
PASS=0
FAIL=0

# Get team ID first (needed for creating issues)
echo -n "Test 0: Getting team ID... "
team_output=$(./target/debug/linears --out json list team 2>&1)
# Extract first team ID using grep and sed
team_id=$(echo "$team_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"\([^"]*\)"/\1/')
if [ -n "$team_id" ]; then
    echo "PASS (team_id: $team_id)"
    ((PASS++))
else
    echo "FAIL (no team ID found)"
    echo "Output: $team_output"
    ((FAIL++))
    # Skip remaining tests if no team ID
    echo ""
    echo "Results: $PASS passed, $FAIL failed"
    echo "Cannot continue without team ID!"
    exit 1
fi

# Test 1: Create command requires input
echo -n "Test 1: Create without input returns error... "
output=$(./target/debug/linears create issue 2>&1)
exit_code=$?
if [ "$exit_code" -ne 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (expected error, got exit code $exit_code)"
    ((FAIL++))
fi

# Test 2: Create issue with valid input
echo -n "Test 2: Create issue with valid input... "
create_output=$(./target/debug/linears --out json create issue --input "{\"title\":\"Test Issue LINEARS_CLI_TEST_$(date +%s)\",\"teamId\":\"$team_id\"}" 2>&1)
exit_code=$?
if [ "$exit_code" -eq 0 ] && echo "$create_output" | grep -q "success"; then
    echo "PASS"
    ((PASS++))
    # Extract issue ID for cleanup
    issue_id=$(echo "$create_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"\([^"]*\)"/\1/')
else
    echo "FAIL (exit code $exit_code)"
    echo "Output: $create_output"
    ((FAIL++))
fi

# Test 3: Update issue (if we got an issue ID)
if [ -n "$issue_id" ]; then
    echo -n "Test 3: Update issue... "
    update_output=$(./target/debug/linears --out json update issue "$issue_id" --set '{"title":"Updated Test Issue"}' 2>&1)
    exit_code=$?
    if [ "$exit_code" -eq 0 ] && echo "$update_output" | grep -q "success"; then
        echo "PASS"
        ((PASS++))
    else
        echo "FAIL (exit code $exit_code)"
        echo "Output: $update_output"
        ((FAIL++))
    fi

    # Test 4: Archive issue
    echo -n "Test 4: Archive issue... "
    archive_output=$(./target/debug/linears --out json archive issue "$issue_id" 2>&1)
    exit_code=$?
    if [ "$exit_code" -eq 0 ] && echo "$archive_output" | grep -q "success"; then
        echo "PASS"
        ((PASS++))
    else
        echo "FAIL (exit code $exit_code)"
        echo "Output: $archive_output"
        ((FAIL++))
    fi

    # Test 5: Unarchive issue
    echo -n "Test 5: Unarchive issue... "
    unarchive_output=$(./target/debug/linears --out json unarchive issue "$issue_id" 2>&1)
    exit_code=$?
    if [ "$exit_code" -eq 0 ] && echo "$unarchive_output" | grep -q "success"; then
        echo "PASS"
        ((PASS++))
    else
        echo "FAIL (exit code $exit_code)"
        echo "Output: $unarchive_output"
        ((FAIL++))
    fi

    # Test 6: Delete issue (cleanup)
    echo -n "Test 6: Delete issue (cleanup)... "
    delete_output=$(./target/debug/linears --out json delete issue "$issue_id" 2>&1)
    exit_code=$?
    if [ "$exit_code" -eq 0 ] && echo "$delete_output" | grep -q "success"; then
        echo "PASS"
        ((PASS++))
    else
        echo "FAIL (exit code $exit_code)"
        echo "Output: $delete_output"
        ((FAIL++))
    fi
else
    echo "Skipping tests 3-6 (no issue ID from create test)"
    ((FAIL+=4))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All mutation tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
