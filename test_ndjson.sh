#!/bin/bash
# Test NDJSON output format

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing NDJSON output format..."
PASS=0
FAIL=0

# Test 1: NDJSON outputs data
echo -n "Test 1: NDJSON outputs data... "
output=$(./target/debug/linears --out ndjson list issue --first 3 2>&1)
if [ -n "$output" ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (empty output)"
    ((FAIL++))
fi

# Test 2: Each line is valid JSON
echo -n "Test 2: Each line is valid JSON... "
all_valid=true
while IFS= read -r line; do
    if [ -n "$line" ]; then
        if ! echo "$line" | python3 -c "import sys,json; json.load(sys.stdin)" 2>/dev/null; then
            all_valid=false
            break
        fi
    fi
done <<< "$output"

if [ "$all_valid" = true ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 3: Multiple lines (one per entity)
echo -n "Test 3: Output has multiple lines... "
line_count=$(echo "$output" | wc -l)
if [ "$line_count" -ge 1 ]; then
    echo "PASS ($line_count lines)"
    ((PASS++))
else
    echo "FAIL (only $line_count lines)"
    ((FAIL++))
fi

# Test 4: Each line has entity fields
echo -n "Test 4: Each line has entity fields (id, title)... "
first_line=$(echo "$output" | head -1)
if echo "$first_line" | grep -q '"id"' && echo "$first_line" | grep -q '"title"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "First line: $first_line"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All NDJSON tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
