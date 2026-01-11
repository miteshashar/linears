#!/bin/bash
# Test duplicate create handling

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing duplicate create handling..."
PASS=0
FAIL=0

# Get team ID
team_output=$(./target/debug/linears --out json list team 2>&1)
team_id=$(echo "$team_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')

if [ -z "$team_id" ]; then
    echo "No team found, cannot run duplicate tests"
    exit 1
fi

# Create first issue with unique title
unique_title="Duplicate Test $(date +%s)"
echo "Creating issue with title: $unique_title"

# Test 1: First create succeeds
echo -n "Test 1: First create succeeds... "
output1=$(./target/debug/linears --out json create issue --input "{\"title\":\"$unique_title\",\"teamId\":\"$team_id\"}" 2>&1)
exit_code1=$?
issue_id=$(echo "$output1" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')
if [ "$exit_code1" -eq 0 ] && [ -n "$issue_id" ]; then
    echo "PASS (created issue $issue_id)"
    ((PASS++))
else
    echo "FAIL (exit code $exit_code1)"
    echo "Output: $output1"
    ((FAIL++))
fi

# Test 2: Second create with same title also succeeds (Linear allows duplicates by title)
echo -n "Test 2: Second create with same title succeeds... "
output2=$(./target/debug/linears --out json create issue --input "{\"title\":\"$unique_title\",\"teamId\":\"$team_id\"}" 2>&1)
exit_code2=$?
issue_id2=$(echo "$output2" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')
if [ "$exit_code2" -eq 0 ] && [ -n "$issue_id2" ]; then
    echo "PASS (created second issue $issue_id2)"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output2"
    ((FAIL++))
fi

# Test 3: IDs are different (truly duplicate issues, not the same one)
echo -n "Test 3: IDs are different (two distinct issues created)... "
if [ "$issue_id" != "$issue_id2" ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (same ID returned)"
    ((FAIL++))
fi

# Cleanup
echo -n "Cleaning up... "
if [ -n "$issue_id" ]; then
    ./target/debug/linears delete issue "$issue_id" > /dev/null 2>&1
fi
if [ -n "$issue_id2" ]; then
    ./target/debug/linears delete issue "$issue_id2" > /dev/null 2>&1
fi
echo "Done"

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All duplicate create tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
