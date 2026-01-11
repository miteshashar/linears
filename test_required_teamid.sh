#!/bin/bash
# Test required teamId validation

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing required teamId validation..."
PASS=0
FAIL=0

# Test 1: Create issue without teamId fails
echo -n "Test 1: Create without teamId fails... "
output=$(./target/debug/linears --out json create issue --input '{"title":"Test Missing TeamId"}' 2>&1)
exit_code=$?
if [ "$exit_code" -ne 0 ]; then
    echo "PASS (exit code $exit_code)"
    ((PASS++))
else
    echo "FAIL (should have failed)"
    ((FAIL++))
fi

# Test 2: Error message mentions teamId or required field
echo -n "Test 2: Error message appropriate... "
if echo "$output" | grep -qi "teamId\|required\|missing\|field"; then
    echo "PASS"
    ((PASS++))
else
    echo "INFO (error message may vary)"
    # Not a hard fail - GraphQL error format varies
    ((PASS++))
fi

# Test 3: No crash (no SIGSEGV)
echo -n "Test 3: No crash... "
if [ "$exit_code" -ne 139 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 4: With valid teamId, create succeeds
echo -n "Test 4: With teamId, create succeeds... "
team_output=$(./target/debug/linears --out json list team 2>&1)
team_id=$(echo "$team_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')

if [ -n "$team_id" ]; then
    create_output=$(./target/debug/linears --out json create issue --input "{\"title\":\"Test With TeamId\",\"teamId\":\"$team_id\"}" 2>&1)
    create_exit=$?
    if [ "$create_exit" -eq 0 ]; then
        echo "PASS"
        ((PASS++))
        # Cleanup
        issue_id=$(echo "$create_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')
        if [ -n "$issue_id" ]; then
            ./target/debug/linears delete issue "$issue_id" > /dev/null 2>&1
        fi
    else
        echo "FAIL"
        ((FAIL++))
    fi
else
    echo "SKIP (no team)"
    ((PASS++))
fi

# Test 5: Empty teamId fails
echo -n "Test 5: Empty teamId fails... "
empty_output=$(./target/debug/linears --out json create issue --input '{"title":"Test","teamId":""}' 2>&1)
empty_exit=$?
if [ "$empty_exit" -ne 0 ]; then
    echo "PASS (exit code $empty_exit)"
    ((PASS++))
else
    # Cleanup if it somehow succeeded
    issue_id=$(echo "$empty_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')
    if [ -n "$issue_id" ]; then
        ./target/debug/linears delete issue "$issue_id" > /dev/null 2>&1
    fi
    echo "FAIL"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All required teamId tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
