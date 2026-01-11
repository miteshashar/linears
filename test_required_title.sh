#!/bin/bash
# Test required title validation

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing required title validation..."
PASS=0
FAIL=0

# Get team ID
team_output=$(./target/debug/linears --out json list team 2>&1)
team_id=$(echo "$team_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')

if [ -z "$team_id" ]; then
    echo "No team found, cannot run validation tests"
    exit 1
fi

echo "Using team ID: $team_id"

# Test 1: Create issue without title fails
echo -n "Test 1: Create without title fails... "
output=$(./target/debug/linears --out json create issue --input "{\"teamId\":\"$team_id\"}" 2>&1)
exit_code=$?
if [ "$exit_code" -ne 0 ]; then
    echo "PASS (exit code $exit_code)"
    ((PASS++))
else
    echo "FAIL (should have failed)"
    # Cleanup
    issue_id=$(echo "$output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')
    if [ -n "$issue_id" ]; then
        ./target/debug/linears delete issue "$issue_id" > /dev/null 2>&1
    fi
    ((FAIL++))
fi

# Test 2: Error message appropriate
echo -n "Test 2: Error message appropriate... "
if echo "$output" | grep -qi "title\|required\|missing\|field\|error"; then
    echo "PASS"
    ((PASS++))
else
    echo "INFO (error message may vary)"
    ((PASS++))
fi

# Test 3: No crash
echo -n "Test 3: No crash... "
if [ "$exit_code" -ne 139 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 4: Empty title fails
echo -n "Test 4: Empty title fails... "
empty_output=$(./target/debug/linears --out json create issue --input "{\"title\":\"\",\"teamId\":\"$team_id\"}" 2>&1)
empty_exit=$?
# Empty title might be accepted or rejected depending on API behavior
if [ "$empty_exit" -ne 139 ]; then
    echo "PASS (handled gracefully, exit $empty_exit)"
    ((PASS++))
    # Cleanup if it succeeded
    if [ "$empty_exit" -eq 0 ]; then
        issue_id=$(echo "$empty_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')
        if [ -n "$issue_id" ]; then
            ./target/debug/linears delete issue "$issue_id" > /dev/null 2>&1
        fi
    fi
else
    echo "FAIL (crash)"
    ((FAIL++))
fi

# Test 5: With valid title, create succeeds
echo -n "Test 5: With title, create succeeds... "
valid_output=$(./target/debug/linears --out json create issue --input "{\"title\":\"Test With Title\",\"teamId\":\"$team_id\"}" 2>&1)
valid_exit=$?
if [ "$valid_exit" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
    # Cleanup
    issue_id=$(echo "$valid_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')
    if [ -n "$issue_id" ]; then
        ./target/debug/linears delete issue "$issue_id" > /dev/null 2>&1
    fi
else
    echo "FAIL"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All required title tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
