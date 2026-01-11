#!/bin/bash
# Test text output format for single entity

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing text output format for single entity..."
PASS=0
FAIL=0

# Get a valid issue ID first
list_output=$(./target/debug/linears --out json list issue --first 1 2>&1)
issue_id=$(echo "$list_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')

if [ -z "$issue_id" ]; then
    echo "No issue found, cannot run text output tests"
    exit 1
fi

# Test 1: Text output returns data
echo -n "Test 1: Text output returns data... "
output=$(./target/debug/linears --out text get issue "$issue_id" 2>&1)
exit_code=$?
if [ "$exit_code" -eq 0 ] && [ -n "$output" ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 2: Text output contains key fields
echo -n "Test 2: Text output contains key fields (id, title)... "
if echo "$output" | grep -qi "id\|title"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 3: Text output is readable (YAML-like format)
echo -n "Test 3: Text output is formatted (readable)... "
line_count=$(echo "$output" | wc -l)
if [ "$line_count" -gt 3 ]; then
    echo "PASS ($line_count lines)"
    ((PASS++))
else
    echo "FAIL (only $line_count lines)"
    ((FAIL++))
fi

# Test 4: Text output contains identifier
echo -n "Test 4: Text output contains identifier... "
if echo "$output" | grep -qE "TES-[0-9]+|identifier"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All text output tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
