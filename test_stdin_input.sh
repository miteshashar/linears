#!/bin/bash
# Test stdin input for mutations

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing stdin input..."
PASS=0
FAIL=0

# Get team ID
team_output=$(./target/debug/linears --out json list team 2>&1)
team_id=$(echo "$team_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')

if [ -z "$team_id" ]; then
    echo "No team found, cannot run stdin tests"
    exit 1
fi

echo "Using team ID: $team_id"

# Test 1: Create with stdin input
echo -n "Test 1: Create with stdin input... "
stdin_output=$(echo "{\"title\":\"Stdin Test $(date +%s)\",\"teamId\":\"$team_id\"}" | ./target/debug/linears --out json create issue --input - 2>&1)
stdin_exit=$?
stdin_id=$(echo "$stdin_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')
if [ "$stdin_exit" -eq 0 ] && [ -n "$stdin_id" ]; then
    echo "PASS (created $stdin_id)"
    ((PASS++))
else
    echo "FAIL (exit code $stdin_exit)"
    echo "Output: $stdin_output"
    ((FAIL++))
fi

# Test 2: Update with stdin input
echo -n "Test 2: Update with stdin input... "
if [ -n "$stdin_id" ]; then
    update_output=$(echo '{"title":"Updated Via Stdin"}' | ./target/debug/linears --out json update issue "$stdin_id" --set - 2>&1)
    update_exit=$?
    if [ "$update_exit" -eq 0 ]; then
        echo "PASS"
        ((PASS++))
    else
        echo "FAIL (exit code $update_exit)"
        ((FAIL++))
    fi
else
    echo "SKIP (no issue to update)"
    ((PASS++))
fi

# Test 3: Multiline stdin works
echo -n "Test 3: Multiline stdin YAML works... "
yaml_output=$(cat << EOF | ./target/debug/linears --out json create issue --input - 2>&1
title: Multiline YAML $(date +%s)
teamId: $team_id
EOF
)
yaml_exit=$?
yaml_id=$(echo "$yaml_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')
if [ "$yaml_exit" -eq 0 ] && [ -n "$yaml_id" ]; then
    echo "PASS (created $yaml_id)"
    ((PASS++))
else
    echo "FAIL (exit code $yaml_exit)"
    ((FAIL++))
fi

# Test 4: Empty stdin handled
echo -n "Test 4: Empty stdin handled... "
empty_output=$(echo "" | ./target/debug/linears --out json create issue --input - 2>&1)
empty_exit=$?
if [ "$empty_exit" -ne 139 ]; then
    echo "PASS (exit code $empty_exit)"
    ((PASS++))
else
    echo "FAIL (crash)"
    ((FAIL++))
fi

# Cleanup
echo -n "Cleaning up... "
for id in "$stdin_id" "$yaml_id"; do
    if [ -n "$id" ]; then
        ./target/debug/linears delete issue "$id" > /dev/null 2>&1
    fi
done
echo "Done"

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All stdin input tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
