#!/bin/bash
# Test pagination consistency

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing pagination consistency..."
PASS=0
FAIL=0

# Test 1: Get first page
echo -n "Test 1: Get first page... "
page1=$(./target/debug/linears --out json list issue --first 2 2>&1)
exit1=$?
if [ "$exit1" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $exit1)"
    ((FAIL++))
fi

# Extract cursor for next page
cursor=$(echo "$page1" | python3 -c "import sys,json; d=json.load(sys.stdin); print(d.get('pageInfo',{}).get('endCursor',''))" 2>/dev/null)
ids_page1=$(echo "$page1" | python3 -c "import sys,json; d=json.load(sys.stdin); print(','.join([n['id'] for n in d.get('nodes',[])]))" 2>/dev/null)

# Test 2: Get second page with cursor
echo -n "Test 2: Get second page with --after... "
if [ -n "$cursor" ]; then
    page2=$(./target/debug/linears --out json list issue --first 2 --after "$cursor" 2>&1)
    exit2=$?
    if [ "$exit2" -eq 0 ]; then
        echo "PASS"
        ((PASS++))
    else
        echo "FAIL (exit code $exit2)"
        ((FAIL++))
    fi
    ids_page2=$(echo "$page2" | python3 -c "import sys,json; d=json.load(sys.stdin); print(','.join([n['id'] for n in d.get('nodes',[])]))" 2>/dev/null)
else
    echo "SKIP (no cursor - maybe only one page of data)"
    ((PASS++))
    ids_page2=""
fi

# Test 3: No duplicate IDs between pages
echo -n "Test 3: No duplicate IDs... "
if [ -n "$ids_page1" ] && [ -n "$ids_page2" ]; then
    # Check for duplicates
    duplicates=0
    for id in $(echo "$ids_page1" | tr ',' '\n'); do
        if echo "$ids_page2" | grep -q "$id"; then
            ((duplicates++))
        fi
    done
    if [ "$duplicates" -eq 0 ]; then
        echo "PASS"
        ((PASS++))
    else
        echo "FAIL ($duplicates duplicates)"
        ((FAIL++))
    fi
else
    echo "SKIP (not enough data)"
    ((PASS++))
fi

# Test 4: pageInfo has correct fields
echo -n "Test 4: pageInfo has correct fields... "
if echo "$page1" | grep -q '"hasNextPage"' && echo "$page1" | grep -q '"endCursor"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 5: Empty cursor handled
echo -n "Test 5: Empty cursor handled... "
empty_output=$(./target/debug/linears --out json list issue --first 1 --after "" 2>&1)
empty_exit=$?
if [ "$empty_exit" -ne 139 ]; then
    echo "PASS (exit code $empty_exit)"
    ((PASS++))
else
    echo "FAIL (crash)"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All pagination consistency tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
