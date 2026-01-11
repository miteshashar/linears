#!/bin/bash
# Test success message on create

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing success message on create..."
PASS=0
FAIL=0

# Get team ID
team_output=$(./target/debug/linears --out json list team 2>&1)
team_id=$(echo "$team_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')

if [ -z "$team_id" ]; then
    echo "No team found, cannot run success message tests"
    exit 1
fi

# Test 1: JSON output shows success structure
echo -n "Test 1: JSON output shows success structure... "
json_output=$(./target/debug/linears --out json create issue --input "{\"title\":\"Success Msg Test $(date +%s)\",\"teamId\":\"$team_id\"}" 2>&1)
json_exit=$?
json_id=$(echo "$json_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')
if [ "$json_exit" -eq 0 ] && echo "$json_output" | grep -q '"success":true\|"id":'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $json_output"
    ((FAIL++))
fi

# Test 2: Created ID is displayed
echo -n "Test 2: Created ID is displayed... "
if [ -n "$json_id" ]; then
    echo "PASS ($json_id)"
    ((PASS++))
else
    echo "FAIL (no ID found)"
    ((FAIL++))
fi

# Test 3: Exit code is 0 for success
echo -n "Test 3: Exit code is 0... "
if [ "$json_exit" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $json_exit)"
    ((FAIL++))
fi

# Test 4: Table output also works
echo -n "Test 4: Table output works... "
table_output=$(./target/debug/linears --out table create issue --input "{\"title\":\"Table Success Test $(date +%s)\",\"teamId\":\"$team_id\"}" 2>&1)
table_exit=$?
table_id=$(echo "$table_output" | grep -o '[0-9a-f-]\{36\}' | head -1)
if [ "$table_exit" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $table_exit)"
    ((FAIL++))
fi

# Test 5: Response contains useful info
echo -n "Test 5: Response contains useful info... "
if echo "$json_output" | grep -q 'issue\|id\|success\|identifier'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Cleanup
echo -n "Cleaning up... "
for id in "$json_id" "$table_id"; do
    if [ -n "$id" ]; then
        ./target/debug/linears delete issue "$id" > /dev/null 2>&1
    fi
done
echo "Done"

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All success message tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
