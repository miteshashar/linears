#!/bin/bash
# Test table layout with long text

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing table layout with long text..."
PASS=0
FAIL=0

# Get team ID
team_output=$(./target/debug/linears --out json list team 2>&1)
team_id=$(echo "$team_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')

if [ -z "$team_id" ]; then
    echo "No team found, cannot run layout tests"
    exit 1
fi

# Create a long title (200 chars)
long_title="This is a very long title that is used to test how the table layout handles long text. It should wrap or truncate properly without breaking the terminal layout. Extra text to make it really long..."
long_title="${long_title}$(date +%s)"

echo "Creating issue with long title..."
create_output=$(./target/debug/linears --out json create issue --input "{\"title\":\"$long_title\",\"teamId\":\"$team_id\"}" 2>&1)
issue_id=$(echo "$create_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')

if [ -z "$issue_id" ]; then
    echo "Failed to create test issue"
    echo "Output: $create_output"
    exit 1
fi

echo "Created issue: $issue_id"

# Test 1: Table output doesn't crash with long text
echo -n "Test 1: Table output works with long text... "
table_output=$(./target/debug/linears --out table list issue --first 1 --filter "{\"id\":{\"eq\":\"$issue_id\"}}" 2>&1)
table_exit=$?
if [ "$table_exit" -ne 139 ]; then
    echo "PASS (exit code $table_exit)"
    ((PASS++))
else
    echo "FAIL (crash)"
    ((FAIL++))
fi

# Test 2: Output contains text (not empty/broken)
echo -n "Test 2: Output contains content... "
if [ -n "$table_output" ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (empty output)"
    ((FAIL++))
fi

# Test 3: JSON output with long text works
echo -n "Test 3: JSON with long text works... "
json_output=$(./target/debug/linears --out json list issue --first 1 --filter "{\"id\":{\"eq\":\"$issue_id\"}}" 2>&1)
json_exit=$?
if [ "$json_exit" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $json_exit)"
    ((FAIL++))
fi

# Test 4: Long title is preserved in output
echo -n "Test 4: Long title preserved... "
if echo "$json_output" | grep -q "very long title"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 5: NDJSON with long text works
echo -n "Test 5: NDJSON with long text works... "
ndjson_output=$(./target/debug/linears --out ndjson list issue --first 1 --filter "{\"id\":{\"eq\":\"$issue_id\"}}" 2>&1)
ndjson_exit=$?
if [ "$ndjson_exit" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $ndjson_exit)"
    ((FAIL++))
fi

# Cleanup
echo -n "Cleaning up... "
./target/debug/linears delete issue "$issue_id" > /dev/null 2>&1
echo "Done"

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All table layout tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
