#!/bin/bash
# Test archive idempotency

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing archive idempotency..."
PASS=0
FAIL=0

# Get team ID
team_output=$(./target/debug/linears --out json list team 2>&1)
team_id=$(echo "$team_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')

if [ -z "$team_id" ]; then
    echo "No team found, cannot run archive tests"
    exit 1
fi

# Create a test issue
unique_title="Archive Idempotent Test $(date +%s)"
echo "Creating test issue with title: $unique_title"

create_output=$(./target/debug/linears --out json create issue --input "{\"title\":\"$unique_title\",\"teamId\":\"$team_id\"}" 2>&1)
issue_id=$(echo "$create_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')

if [ -z "$issue_id" ]; then
    echo "Failed to create test issue"
    echo "Output: $create_output"
    exit 1
fi

echo "Created issue: $issue_id"

# Test 1: First archive succeeds
echo -n "Test 1: First archive succeeds... "
archive_output1=$(./target/debug/linears --out json archive issue "$issue_id" 2>&1)
exit_code1=$?
if [ "$exit_code1" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $exit_code1)"
    echo "Output: $archive_output1"
    ((FAIL++))
fi

# Test 2: Second archive is idempotent (no crash)
echo -n "Test 2: Second archive handled gracefully... "
archive_output2=$(./target/debug/linears --out json archive issue "$issue_id" 2>&1)
exit_code2=$?
# Should not crash (exit code 139 = SIGSEGV)
if [ "$exit_code2" -ne 139 ]; then
    echo "PASS (exit code $exit_code2)"
    ((PASS++))
else
    echo "FAIL (crash)"
    ((FAIL++))
fi

# Test 3: Third archive also works (verify consistent behavior)
echo -n "Test 3: Third archive consistent... "
archive_output3=$(./target/debug/linears --out json archive issue "$issue_id" 2>&1)
exit_code3=$?
if [ "$exit_code3" -eq "$exit_code2" ]; then
    echo "PASS (same exit code $exit_code3)"
    ((PASS++))
else
    echo "FAIL (inconsistent exit codes: $exit_code2 vs $exit_code3)"
    ((FAIL++))
fi

# Test 4: Archive operation returns success
echo -n "Test 4: Archive returns success... "
# Check archive result contains success:true
if echo "$archive_output2" | grep -q '"success":true'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (no success in response)"
    echo "Output: $archive_output2"
    ((FAIL++))
fi

# Cleanup - delete the issue
echo -n "Cleaning up... "
./target/debug/linears delete issue "$issue_id" > /dev/null 2>&1
echo "Done"

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All archive idempotency tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
