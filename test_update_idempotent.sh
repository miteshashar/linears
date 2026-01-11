#!/bin/bash
# Test update idempotency

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing update idempotency..."
PASS=0
FAIL=0

# Get team ID
team_output=$(./target/debug/linears --out json list team 2>&1)
team_id=$(echo "$team_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')

if [ -z "$team_id" ]; then
    echo "No team found, cannot run idempotency tests"
    exit 1
fi

# Create a test issue
unique_title="Idempotent Test $(date +%s)"
echo "Creating test issue with title: $unique_title"

create_output=$(./target/debug/linears --out json create issue --input "{\"title\":\"$unique_title\",\"teamId\":\"$team_id\"}" 2>&1)
issue_id=$(echo "$create_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')

if [ -z "$issue_id" ]; then
    echo "Failed to create test issue"
    echo "Output: $create_output"
    exit 1
fi

echo "Created issue: $issue_id"

# Test 1: Update with same title succeeds
echo -n "Test 1: Update with same title succeeds... "
update_output1=$(./target/debug/linears --out json update issue "$issue_id" --set "{\"title\":\"$unique_title\"}" 2>&1)
exit_code1=$?
if [ "$exit_code1" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $exit_code1)"
    echo "Output: $update_output1"
    ((FAIL++))
fi

# Test 2: Update with same title again succeeds (idempotent)
echo -n "Test 2: Second update with same title succeeds... "
update_output2=$(./target/debug/linears --out json update issue "$issue_id" --set "{\"title\":\"$unique_title\"}" 2>&1)
exit_code2=$?
if [ "$exit_code2" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $exit_code2)"
    echo "Output: $update_output2"
    ((FAIL++))
fi

# Test 3: Title is unchanged after updates
echo -n "Test 3: Title unchanged after updates... "
get_output=$(./target/debug/linears --out json get issue "$issue_id" 2>&1)
current_title=$(echo "$get_output" | grep -o '"title":"[^"]*"' | head -1 | sed 's/"title":"//;s/"//')
if [ "$current_title" = "$unique_title" ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (title changed to '$current_title')"
    ((FAIL++))
fi

# Test 4: Multiple rapid updates don't cause issues
echo -n "Test 4: Multiple rapid updates succeed... "
rapid_success=true
for i in 1 2 3; do
    rapid_output=$(./target/debug/linears --out json update issue "$issue_id" --set "{\"title\":\"$unique_title\"}" 2>&1)
    if [ $? -ne 0 ]; then
        rapid_success=false
        break
    fi
done
if [ "$rapid_success" = true ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Cleanup
echo -n "Cleaning up... "
./target/debug/linears delete issue "$issue_id" > /dev/null 2>&1
echo "Done"

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All update idempotency tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
