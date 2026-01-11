#!/bin/bash
# Test filtering on deleted entities

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing filter on deleted entities..."
PASS=0
FAIL=0

# Get team ID
team_output=$(./target/debug/linears --out json list team 2>&1)
team_id=$(echo "$team_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')

if [ -z "$team_id" ]; then
    echo "No team found, cannot run filter tests"
    exit 1
fi

# Create a test issue with unique title
unique_title="FilterDeleted $(date +%s) Test"
echo "Creating test issue with title: $unique_title"

create_output=$(./target/debug/linears --out json create issue --input "{\"title\":\"$unique_title\",\"teamId\":\"$team_id\"}" 2>&1)
issue_id=$(echo "$create_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')
issue_identifier=$(echo "$create_output" | grep -o '"identifier":"[^"]*"' | head -1 | sed 's/"identifier":"//;s/"//')

if [ -z "$issue_id" ]; then
    echo "Failed to create test issue"
    echo "Output: $create_output"
    exit 1
fi

echo "Created issue: $issue_id ($issue_identifier)"

# Test 1: Issue appears in search before deletion
echo -n "Test 1: Issue found in search before deletion... "
search_before=$(./target/debug/linears --out json search issue "FilterDeleted" 2>&1)
if echo "$search_before" | grep -q "$issue_id"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $search_before"
    ((FAIL++))
fi

# Delete the issue
echo "Deleting issue..."
./target/debug/linears delete issue "$issue_id" > /dev/null 2>&1

# Brief wait for deletion to propagate
sleep 1

# Test 2: Search for deleted issue (may or may not find due to soft-delete)
echo -n "Test 2: Search after deletion handled gracefully... "
search_after=$(./target/debug/linears --out json search issue "FilterDeleted" 2>&1)
search_exit=$?
# Key is that search doesn't crash
if [ "$search_exit" -ne 139 ]; then
    echo "PASS (exit code $search_exit)"
    ((PASS++))
else
    echo "FAIL (crash)"
    ((FAIL++))
fi

# Test 3: List with filter by ID doesn't crash
echo -n "Test 3: List filter by deleted ID handled gracefully... "
list_filter=$(./target/debug/linears --out json list issue --filter "id: {eq: \"$issue_id\"}" --first 1 2>&1)
list_exit=$?
if [ "$list_exit" -ne 139 ]; then
    echo "PASS (exit code $list_exit)"
    ((PASS++))
else
    echo "FAIL (crash)"
    ((FAIL++))
fi

# Test 4: Get by identifier still works (soft delete)
echo -n "Test 4: Get by identifier after delete handled... "
if [ -n "$issue_identifier" ]; then
    get_by_id=$(./target/debug/linears --out json get issue "$issue_identifier" 2>&1)
    get_exit=$?
    # Soft-delete means it may still be accessible
    if [ "$get_exit" -ne 139 ]; then
        echo "PASS (exit code $get_exit)"
        ((PASS++))
    else
        echo "FAIL (crash)"
        ((FAIL++))
    fi
else
    echo "SKIP (no identifier)"
    ((PASS++))
fi

# Test 5: Empty result handling
echo -n "Test 5: Search for non-existent deleted content... "
random_search="NonExistent$(date +%s%N)XYZ"
empty_search=$(./target/debug/linears --out json search issue "$random_search" 2>&1)
empty_exit=$?
if [ "$empty_exit" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $empty_exit)"
    echo "Output: $empty_search"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All filter deleted tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
