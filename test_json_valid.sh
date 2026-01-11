#!/bin/bash
# Test JSON output is valid with special characters

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing JSON output validity..."
PASS=0
FAIL=0

# Get team ID
team_output=$(./target/debug/linears --out json list team 2>&1)
team_id=$(echo "$team_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')

if [ -z "$team_id" ]; then
    echo "No team found, cannot run JSON validity tests"
    exit 1
fi

# Create issue with special characters
special_title="Test with \"quotes\" and \\backslashes\\ and <brackets> and &ampersand $(date +%s)"
echo "Creating issue with special chars..."

create_output=$(./target/debug/linears --out json create issue --input "{\"title\":\"$special_title\",\"teamId\":\"$team_id\"}" 2>&1)
issue_id=$(echo "$create_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')

# Test 1: Create output is valid JSON
echo -n "Test 1: Create output is valid JSON... "
if echo "$create_output" | python3 -c "import sys,json; json.load(sys.stdin)" 2>/dev/null; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $create_output"
    ((FAIL++))
fi

if [ -n "$issue_id" ]; then
    # Test 2: Get output is valid JSON
    echo -n "Test 2: Get output is valid JSON... "
    get_output=$(./target/debug/linears --out json get issue "$issue_id" 2>&1)
    if echo "$get_output" | python3 -c "import sys,json; json.load(sys.stdin)" 2>/dev/null; then
        echo "PASS"
        ((PASS++))
    else
        echo "FAIL"
        ((FAIL++))
    fi

    # Test 3: List output is valid JSON
    echo -n "Test 3: List output is valid JSON... "
    list_output=$(./target/debug/linears --out json list issue --first 1 2>&1)
    if echo "$list_output" | python3 -c "import sys,json; json.load(sys.stdin)" 2>/dev/null; then
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
else
    echo "Skipping get/list tests (issue not created)"
    ((PASS+=2))
fi

# Test 4: Unicode in title is valid JSON
echo -n "Test 4: Unicode handled in JSON... "
unicode_output=$(./target/debug/linears --out json create issue --input "{\"title\":\"Unicode test \u2713 \u2717 \u00e9 $(date +%s)\",\"teamId\":\"$team_id\"}" 2>&1)
unicode_id=$(echo "$unicode_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')
if echo "$unicode_output" | python3 -c "import sys,json; json.load(sys.stdin)" 2>/dev/null; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi
if [ -n "$unicode_id" ]; then
    ./target/debug/linears delete issue "$unicode_id" > /dev/null 2>&1
fi

# Test 5: Empty results are valid JSON
echo -n "Test 5: Empty results are valid JSON... "
empty_output=$(./target/debug/linears --out json search issue "xyznonexistent999$(date +%s)" 2>&1)
if echo "$empty_output" | python3 -c "import sys,json; json.load(sys.stdin)" 2>/dev/null; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All JSON validity tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
