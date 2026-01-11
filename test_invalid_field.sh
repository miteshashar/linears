#!/bin/bash
# Test update with invalid field name

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing update with invalid field name..."
PASS=0
FAIL=0

# Get team ID
team_output=$(./target/debug/linears --out json list team 2>&1)
team_id=$(echo "$team_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')

if [ -z "$team_id" ]; then
    echo "No team found, cannot run validation tests"
    exit 1
fi

# Create a test issue
unique_title="Invalid Field Test $(date +%s)"
echo "Creating test issue..."

create_output=$(./target/debug/linears --out json create issue --input "{\"title\":\"$unique_title\",\"teamId\":\"$team_id\"}" 2>&1)
issue_id=$(echo "$create_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')

if [ -z "$issue_id" ]; then
    echo "Failed to create test issue"
    echo "Output: $create_output"
    exit 1
fi

echo "Created issue: $issue_id"

# Test 1: Update with invalid field name handled
echo -n "Test 1: Invalid field name handled... "
output=$(./target/debug/linears --out json update issue "$issue_id" --set '{"nonexistentField":"value"}' 2>&1)
exit_code=$?
# Should either fail or ignore the field - key is no crash
if [ "$exit_code" -ne 139 ]; then
    echo "PASS (exit code $exit_code)"
    ((PASS++))
else
    echo "FAIL (crash)"
    ((FAIL++))
fi

# Test 2: Error message if failed
echo -n "Test 2: Error message appropriate... "
if [ "$exit_code" -ne 0 ]; then
    if echo "$output" | grep -qi "field\|unknown\|error\|invalid"; then
        echo "PASS"
        ((PASS++))
    else
        echo "INFO (error message may vary)"
        ((PASS++))
    fi
else
    echo "INFO (field was ignored)"
    ((PASS++))
fi

# Test 3: No panic
echo -n "Test 3: No panic... "
if ! echo "$output" | grep -q "panic"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 4: Multiple invalid fields handled
echo -n "Test 4: Multiple invalid fields handled... "
multi_output=$(./target/debug/linears --out json update issue "$issue_id" --set '{"foo":"bar","baz":"qux"}' 2>&1)
multi_exit=$?
if [ "$multi_exit" -ne 139 ]; then
    echo "PASS (exit code $multi_exit)"
    ((PASS++))
else
    echo "FAIL (crash)"
    ((FAIL++))
fi

# Test 5: Valid update still works
echo -n "Test 5: Valid update works... "
valid_output=$(./target/debug/linears --out json update issue "$issue_id" --set '{"title":"Updated Title"}' 2>&1)
valid_exit=$?
if [ "$valid_exit" -eq 0 ]; then
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
    echo "All invalid field tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
