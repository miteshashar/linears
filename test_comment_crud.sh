#!/bin/bash
# Test CRUD workflow for comments

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing comment CRUD workflow..."
PASS=0
FAIL=0

# Get team ID first (needed for creating issues)
team_output=$(./target/debug/linears --out json list team 2>&1)
team_id=$(echo "$team_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"\([^"]*\)"/\1/')

if [ -z "$team_id" ]; then
    echo "No team found, cannot run comment tests"
    exit 1
fi

# Step 1: Create a test issue
echo -n "Step 1: Creating test issue... "
issue_output=$(./target/debug/linears --out json create issue --input "{\"title\":\"Comment Test Issue $(date +%s)\",\"teamId\":\"$team_id\"}" 2>&1)
issue_id=$(echo "$issue_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"\([^"]*\)"/\1/')
if [ -n "$issue_id" ]; then
    echo "PASS (issue_id: $issue_id)"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $issue_output"
    ((FAIL++))
    exit 1
fi

# Step 2: Create comment
echo -n "Step 2: Creating comment... "
comment_output=$(./target/debug/linears --out json create comment --input "{\"issueId\":\"$issue_id\",\"body\":\"Test comment body $(date +%s)\"}" 2>&1)
comment_id=$(echo "$comment_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"\([^"]*\)"/\1/')
if [ -n "$comment_id" ] && echo "$comment_output" | grep -q "success"; then
    echo "PASS (comment_id: $comment_id)"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $comment_output"
    ((FAIL++))
fi

# Step 3: Get comment
if [ -n "$comment_id" ]; then
    echo -n "Step 3: Getting comment... "
    get_output=$(./target/debug/linears --out json get comment "$comment_id" 2>&1)
    if echo "$get_output" | grep -q "body"; then
        echo "PASS"
        ((PASS++))
    else
        echo "FAIL"
        echo "Output: $get_output"
        ((FAIL++))
    fi

    # Step 4: Update comment
    echo -n "Step 4: Updating comment... "
    update_output=$(./target/debug/linears --out json update comment "$comment_id" --set '{"body":"Updated comment body"}' 2>&1)
    if echo "$update_output" | grep -q "success"; then
        echo "PASS"
        ((PASS++))
    else
        echo "FAIL"
        echo "Output: $update_output"
        ((FAIL++))
    fi

    # Step 5: Delete comment
    echo -n "Step 5: Deleting comment... "
    delete_output=$(./target/debug/linears --out json delete comment "$comment_id" 2>&1)
    if echo "$delete_output" | grep -q "success"; then
        echo "PASS"
        ((PASS++))
    else
        echo "FAIL"
        echo "Output: $delete_output"
        ((FAIL++))
    fi
else
    echo "Skipping steps 3-5 (no comment ID)"
    ((FAIL+=3))
fi

# Cleanup: Delete the test issue
echo -n "Cleanup: Deleting test issue... "
cleanup_output=$(./target/debug/linears --out json delete issue "$issue_id" 2>&1)
if echo "$cleanup_output" | grep -q "success"; then
    echo "Done"
else
    echo "Warning: cleanup failed"
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All comment CRUD tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
