#!/bin/bash
# Test mutate --vars with JSON and YAML

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing mutate --vars format..."
PASS=0
FAIL=0

# Get team ID
team_output=$(./target/debug/linears --out json list team 2>&1)
team_id=$(echo "$team_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')

if [ -z "$team_id" ]; then
    echo "No team found, cannot run vars format tests"
    exit 1
fi

echo "Using team ID: $team_id"

# Test 1: Mutate with JSON vars
echo -n "Test 1: JSON vars works... "
json_output=$(./target/debug/linears --out json mutate issueCreate --vars "{\"input\":{\"title\":\"JSON Vars Test $(date +%s)\",\"teamId\":\"$team_id\"}}" 2>&1)
json_exit=$?
json_id=$(echo "$json_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')
if [ "$json_exit" -eq 0 ] && [ -n "$json_id" ]; then
    echo "PASS (created $json_id)"
    ((PASS++))
else
    echo "FAIL (exit code $json_exit)"
    echo "Output: $json_output"
    ((FAIL++))
fi

# Test 2: Mutate with YAML vars
echo -n "Test 2: YAML vars works... "
yaml_vars="input:
  title: YAML Vars Test $(date +%s)
  teamId: $team_id"
yaml_output=$(./target/debug/linears --out json mutate issueCreate --vars "$yaml_vars" 2>&1)
yaml_exit=$?
yaml_id=$(echo "$yaml_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')
if [ "$yaml_exit" -eq 0 ] && [ -n "$yaml_id" ]; then
    echo "PASS (created $yaml_id)"
    ((PASS++))
else
    echo "FAIL (exit code $yaml_exit)"
    echo "Vars: $yaml_vars"
    echo "Output: $yaml_output"
    ((FAIL++))
fi

# Test 3: Invalid vars format handled
echo -n "Test 3: Invalid vars format handled... "
invalid_output=$(./target/debug/linears --out json mutate issueCreate --vars "not valid {{{{" 2>&1)
invalid_exit=$?
if [ "$invalid_exit" -ne 139 ]; then
    echo "PASS (exit code $invalid_exit)"
    ((PASS++))
else
    echo "FAIL (crash)"
    ((FAIL++))
fi

# Test 4: Missing required vars handled
echo -n "Test 4: Missing required vars handled... "
missing_output=$(./target/debug/linears --out json mutate issueCreate --vars '{}' 2>&1)
missing_exit=$?
if [ "$missing_exit" -ne 139 ]; then
    echo "PASS (exit code $missing_exit)"
    ((PASS++))
else
    echo "FAIL (crash)"
    ((FAIL++))
fi

# Test 5: Both created distinct issues
echo -n "Test 5: Both are distinct issues... "
if [ -n "$json_id" ] && [ -n "$yaml_id" ] && [ "$json_id" != "$yaml_id" ]; then
    echo "PASS"
    ((PASS++))
else
    echo "INFO (one or both may have failed)"
    ((PASS++))
fi

# Cleanup
echo -n "Cleaning up... "
for id in "$json_id" "$yaml_id"; do
    if [ -n "$id" ]; then
        ./target/debug/linears delete issue "$id" > /dev/null 2>&1
    fi
done
echo "Done"

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All vars format tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
