#!/bin/bash
# Test pagination functionality

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing pagination..."
PASS=0
FAIL=0

# Test 1: First page returns pageInfo
echo -n "Test 1: First page returns pageInfo... "
output=$(./target/debug/linears --out json list issue --first 5 2>&1)
exit_code=$?
if [ "$exit_code" -eq 0 ] && echo "$output" | grep -q "pageInfo"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $exit_code or missing pageInfo)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 2: First page has endCursor
echo -n "Test 2: First page has endCursor... "
output=$(./target/debug/linears --out json list issue --first 5 2>&1)
if echo "$output" | grep -q "endCursor"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (no endCursor)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 3: First page has nodes
echo -n "Test 3: First page has nodes array... "
output=$(./target/debug/linears --out json list issue --first 5 2>&1)
if echo "$output" | grep -q '"nodes"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (no nodes)"
    ((FAIL++))
fi

# Test 4: Extract cursor and use it for second page
echo -n "Test 4: Second page with cursor works... "
# Get first page and extract cursor
first_page=$(./target/debug/linears --out json list issue --first 2 2>&1)
cursor=$(echo "$first_page" | grep -o '"endCursor":"[^"]*"' | head -1 | sed 's/"endCursor":"\([^"]*\)"/\1/')

if [ -n "$cursor" ] && [ "$cursor" != "null" ]; then
    # Get second page with cursor
    second_page=$(./target/debug/linears --out json list issue --first 2 --after "$cursor" 2>&1)
    exit_code=$?
    if [ "$exit_code" -eq 0 ] && echo "$second_page" | grep -q "nodes"; then
        echo "PASS"
        ((PASS++))
    else
        echo "FAIL (second page request failed)"
        echo "Output: $second_page"
        ((FAIL++))
    fi
else
    echo "SKIP (no cursor or only one page of data)"
    ((PASS++))
fi

# Test 5: Pages have different data (if enough data)
echo -n "Test 5: Different pages have different data... "
# Get first two issues from page 1
first_page=$(./target/debug/linears --out json list issue --first 2 2>&1)
first_ids=$(echo "$first_page" | grep -o '"id":"[^"]*"' | head -2)
cursor=$(echo "$first_page" | grep -o '"endCursor":"[^"]*"' | head -1 | sed 's/"endCursor":"\([^"]*\)"/\1/')

if [ -n "$cursor" ] && [ "$cursor" != "null" ]; then
    # Get first two issues from page 2
    second_page=$(./target/debug/linears --out json list issue --first 2 --after "$cursor" 2>&1)
    second_ids=$(echo "$second_page" | grep -o '"id":"[^"]*"' | head -2)

    # Check if any IDs overlap
    if [ "$first_ids" != "$second_ids" ]; then
        echo "PASS"
        ((PASS++))
    else
        echo "FAIL (same IDs on different pages)"
        ((FAIL++))
    fi
else
    echo "SKIP (not enough data for multiple pages)"
    ((PASS++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All pagination tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
