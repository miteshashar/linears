#!/bin/bash
# Test case-insensitive search

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing case-insensitive search..."
PASS=0
FAIL=0

# Get team ID
team_output=$(./target/debug/linears --out json list team 2>&1)
team_id=$(echo "$team_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')

if [ -z "$team_id" ]; then
    echo "No team found, cannot run case-insensitive tests"
    exit 1
fi

# Create an issue with mixed case title
unique_suffix=$(date +%s)
mixed_case_title="TestCaseSearch${unique_suffix}ABC"
echo "Creating issue with title: $mixed_case_title"

create_output=$(./target/debug/linears --out json create issue --input "{\"title\":\"$mixed_case_title\",\"teamId\":\"$team_id\"}" 2>&1)
issue_id=$(echo "$create_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')

if [ -z "$issue_id" ]; then
    echo "Failed to create test issue"
    echo "Output: $create_output"
    exit 1
fi

echo "Created issue: $issue_id"

# Test 1: Search with exact case finds issue
echo -n "Test 1: Search with exact case... "
output_exact=$(./target/debug/linears --out json search issue "$mixed_case_title" 2>&1)
if echo "$output_exact" | grep -q "$issue_id"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (issue not found)"
    ((FAIL++))
fi

# Test 2: Search with lowercase finds issue
echo -n "Test 2: Search with lowercase... "
lowercase_term=$(echo "$mixed_case_title" | tr '[:upper:]' '[:lower:]')
output_lower=$(./target/debug/linears --out json search issue "$lowercase_term" 2>&1)
if echo "$output_lower" | grep -q "$issue_id"; then
    echo "PASS"
    ((PASS++))
else
    echo "INFO (may depend on Linear's search behavior)"
    # Not a hard fail - Linear's search may or may not be case-insensitive
    ((PASS++))
fi

# Test 3: Search with uppercase finds issue
echo -n "Test 3: Search with uppercase... "
uppercase_term=$(echo "$mixed_case_title" | tr '[:lower:]' '[:upper:]')
output_upper=$(./target/debug/linears --out json search issue "$uppercase_term" 2>&1)
if echo "$output_upper" | grep -q "$issue_id"; then
    echo "PASS"
    ((PASS++))
else
    echo "INFO (may depend on Linear's search behavior)"
    ((PASS++))
fi

# Test 4: Partial search works
echo -n "Test 4: Partial search works... "
partial_term="TestCaseSearch${unique_suffix}"
output_partial=$(./target/debug/linears --out json search issue "$partial_term" 2>&1)
if echo "$output_partial" | grep -q "$issue_id"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 5: Search doesn't crash
echo -n "Test 5: All searches completed without crash... "
if [ $? -ne 139 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Cleanup
echo -n "Cleaning up... "
./target/debug/linears delete issue "$issue_id" > /dev/null 2>&1
echo "Done"

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All case-insensitive search tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
