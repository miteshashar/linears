#!/bin/bash
# Test table output format

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing table output format..."
PASS=0
FAIL=0

# Test 1: Default output (table) returns data
echo -n "Test 1: Default table output returns data... "
output=$(./target/debug/linears list issue --first 2 2>&1)
exit_code=$?
if [ "$exit_code" -eq 0 ] && [ -n "$output" ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 2: Table output contains nodes data
echo -n "Test 2: Table output contains nodes data... "
if echo "$output" | grep -q "nodes"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 3: Table output contains issue identifiers
echo -n "Test 3: Table output contains issue data (identifiers)... "
if echo "$output" | grep -qE "TES-[0-9]+|identifier"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 4: Table output is formatted/multiline
echo -n "Test 4: Table output is formatted (multiline)... "
line_count=$(echo "$output" | wc -l)
if [ "$line_count" -gt 5 ]; then
    echo "PASS ($line_count lines)"
    ((PASS++))
else
    echo "FAIL (only $line_count lines)"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All table output tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
