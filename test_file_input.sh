#!/bin/bash
# Test file input for mutations

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing file input..."
PASS=0
FAIL=0

# Get team ID
team_output=$(./target/debug/linears --out json list team 2>&1)
team_id=$(echo "$team_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')

if [ -z "$team_id" ]; then
    echo "No team found, cannot run file input tests"
    exit 1
fi

echo "Using team ID: $team_id"

# Create temp files
input_json=$(mktemp)
input_yaml=$(mktemp)

# Test 1: Create with --input-file (JSON)
echo -n "Test 1: Create with --input-file JSON... "
echo "{\"title\":\"File Input JSON Test $(date +%s)\",\"teamId\":\"$team_id\"}" > "$input_json"
json_output=$(./target/debug/linears --out json create issue --input-file "$input_json" 2>&1)
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

# Test 2: Create with --input-file (YAML)
echo -n "Test 2: Create with --input-file YAML... "
cat > "$input_yaml" << EOF
title: File Input YAML Test $(date +%s)
teamId: $team_id
EOF
yaml_output=$(./target/debug/linears --out json create issue --input-file "$input_yaml" 2>&1)
yaml_exit=$?
yaml_id=$(echo "$yaml_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')
if [ "$yaml_exit" -eq 0 ] && [ -n "$yaml_id" ]; then
    echo "PASS (created $yaml_id)"
    ((PASS++))
else
    echo "FAIL (exit code $yaml_exit)"
    echo "Output: $yaml_output"
    ((FAIL++))
fi

# Test 3: Non-existent file handled
echo -n "Test 3: Non-existent file handled... "
nofile_output=$(./target/debug/linears --out json create issue --input-file "/nonexistent/file.json" 2>&1)
nofile_exit=$?
if [ "$nofile_exit" -ne 0 ] && [ "$nofile_exit" -ne 139 ]; then
    echo "PASS (exit code $nofile_exit)"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Cleanup
echo -n "Cleaning up... "
rm -f "$input_json" "$input_yaml"
for id in "$json_id" "$yaml_id"; do
    if [ -n "$id" ]; then
        ./target/debug/linears delete issue "$id" > /dev/null 2>&1
    fi
done
echo "Done"

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All file input tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
