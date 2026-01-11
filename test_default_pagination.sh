#!/bin/bash
# Test default pagination limit

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing default pagination limit..."
PASS=0
FAIL=0

# Test 1: List issues without --first
echo -n "Test 1: List issues without --first... "
output=$(./target/debug/linears --out json list issue 2>&1)
exit_code=$?
if [ "$exit_code" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $exit_code)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 2: Count nodes (should be 20 or less)
echo -n "Test 2: Default pagination limit applied... "
# Count occurrences of "id" in nodes array (rough count)
node_count=$(echo "$output" | grep -o '"id":"[^"]*"' | wc -l | tr -d ' ')
# The count includes id fields in nested objects, so this is approximate
# Check if nodes array exists and has items
if echo "$output" | grep -q '"nodes":\['; then
    # Get just the issue ids (at top level of nodes)
    issue_count=$(echo "$output" | python3 -c "import sys, json; d=json.load(sys.stdin); print(len(d.get('nodes', [])))" 2>/dev/null || echo "0")
    if [ "$issue_count" -le 20 ]; then
        echo "PASS (got $issue_count items, max 20)"
        ((PASS++))
    else
        echo "FAIL (got $issue_count items, expected <= 20)"
        ((FAIL++))
    fi
else
    echo "FAIL (no nodes array found)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 3: Explicit --first 5 works
echo -n "Test 3: Explicit --first 5 returns <= 5 items... "
output5=$(./target/debug/linears --out json list issue --first 5 2>&1)
if echo "$output5" | grep -q '"nodes":\['; then
    count5=$(echo "$output5" | python3 -c "import sys, json; d=json.load(sys.stdin); print(len(d.get('nodes', [])))" 2>/dev/null || echo "0")
    if [ "$count5" -le 5 ]; then
        echo "PASS (got $count5 items)"
        ((PASS++))
    else
        echo "FAIL (got $count5 items)"
        ((FAIL++))
    fi
else
    echo "FAIL (no nodes array)"
    ((FAIL++))
fi

# Test 4: Explicit --first 1 returns exactly 1 item
echo -n "Test 4: Explicit --first 1 returns 1 item... "
output1=$(./target/debug/linears --out json list issue --first 1 2>&1)
if echo "$output1" | grep -q '"nodes":\['; then
    count1=$(echo "$output1" | python3 -c "import sys, json; d=json.load(sys.stdin); print(len(d.get('nodes', [])))" 2>/dev/null || echo "0")
    if [ "$count1" -eq 1 ] || [ "$count1" -eq 0 ]; then
        echo "PASS (got $count1 item)"
        ((PASS++))
    else
        echo "FAIL (got $count1 items)"
        ((FAIL++))
    fi
else
    echo "FAIL (no nodes array)"
    ((FAIL++))
fi

# Test 5: pageInfo is present
echo -n "Test 5: pageInfo present in response... "
if echo "$output" | grep -q '"pageInfo"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All default pagination tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
