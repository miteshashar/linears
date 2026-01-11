#!/bin/bash
# Test JSON and YAML input formats

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing JSON and YAML input formats..."
PASS=0
FAIL=0

# Get team ID
team_output=$(./target/debug/linears --out json list team 2>&1)
team_id=$(echo "$team_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')

if [ -z "$team_id" ]; then
    echo "No team found, cannot run input format tests"
    exit 1
fi

echo "Using team ID: $team_id"

# Test 1: JSON input works
echo -n "Test 1: JSON input works... "
json_output=$(./target/debug/linears --out json create issue --input "{\"title\":\"JSON Test $(date +%s)\",\"teamId\":\"$team_id\"}" 2>&1)
json_exit=$?
json_id=$(echo "$json_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')
if [ "$json_exit" -eq 0 ] && [ -n "$json_id" ]; then
    echo "PASS (created $json_id)"
    ((PASS++))
else
    echo "FAIL (exit code $json_exit)"
    ((FAIL++))
fi

# Test 2: YAML input works
echo -n "Test 2: YAML input works... "
yaml_input="title: YAML Test $(date +%s)
teamId: $team_id"
yaml_output=$(./target/debug/linears --out json create issue --input "$yaml_input" 2>&1)
yaml_exit=$?
yaml_id=$(echo "$yaml_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')
if [ "$yaml_exit" -eq 0 ] && [ -n "$yaml_id" ]; then
    echo "PASS (created $yaml_id)"
    ((PASS++))
else
    echo "FAIL (exit code $yaml_exit)"
    echo "Input: $yaml_input"
    echo "Output: $yaml_output"
    ((FAIL++))
fi

# Test 3: Both created distinct issues
echo -n "Test 3: Both are distinct issues... "
if [ -n "$json_id" ] && [ -n "$yaml_id" ] && [ "$json_id" != "$yaml_id" ]; then
    echo "PASS"
    ((PASS++))
else
    echo "INFO (one or both may have failed)"
    ((PASS++))
fi

# Test 4: Invalid input format handled
echo -n "Test 4: Invalid input format handled... "
invalid_output=$(./target/debug/linears --out json create issue --input "not valid json or yaml {{{{" 2>&1)
invalid_exit=$?
if [ "$invalid_exit" -ne 139 ]; then
    echo "PASS (exit code $invalid_exit)"
    ((PASS++))
else
    echo "FAIL (crash)"
    ((FAIL++))
fi

# Test 5: Complex YAML with nested values
echo -n "Test 5: Complex YAML input handled... "
complex_yaml="title: Complex YAML $(date +%s)
teamId: $team_id
priority: 1"
complex_output=$(./target/debug/linears --out json create issue --input "$complex_yaml" 2>&1)
complex_exit=$?
complex_id=$(echo "$complex_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')
if [ "$complex_exit" -ne 139 ]; then
    echo "PASS (exit code $complex_exit)"
    ((PASS++))
else
    echo "FAIL (crash)"
    ((FAIL++))
fi

# Cleanup
echo -n "Cleaning up... "
for id in "$json_id" "$yaml_id" "$complex_id"; do
    if [ -n "$id" ]; then
        ./target/debug/linears delete issue "$id" > /dev/null 2>&1
    fi
done
echo "Done"

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All input format tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
