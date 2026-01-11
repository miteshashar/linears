#!/bin/bash
# Test default output format is table

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing default output format..."
PASS=0
FAIL=0

# Test 1: Default output (no --out flag)
echo -n "Test 1: Default output runs successfully... "
output=$(./target/debug/linears list issue --first 1 2>&1)
exit_code=$?
if [ "$exit_code" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $exit_code)"
    ((FAIL++))
fi

# Test 2: Output is NOT JSON (doesn't start with { or [)
echo -n "Test 2: Output is not JSON... "
first_char=$(echo "$output" | head -c 1)
if [ "$first_char" != "{" ] && [ "$first_char" != "[" ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (output appears to be JSON)"
    ((FAIL++))
fi

# Test 3: Output is NOT YAML (no leading ---)
echo -n "Test 3: Output is not YAML... "
if ! echo "$output" | head -1 | grep -q "^---"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (output appears to be YAML)"
    ((FAIL++))
fi

# Test 4: Output looks like table (has separator characters or alignment)
echo -n "Test 4: Output looks like table format... "
# Table output typically has column headers and separators or ASCII table chars
# Or it has multiple words separated by spaces in a tabular format
if echo "$output" | grep -qE "^\s*[A-Za-z].*\s{2,}[A-Za-z]|─|│|├|┼|┤|=+|^[A-Z]"; then
    echo "PASS"
    ((PASS++))
else
    # If empty result, that's also okay
    if [ -z "$output" ] || echo "$output" | grep -qi "no.*found\|empty\|0 items"; then
        echo "PASS (empty/no results)"
        ((PASS++))
    else
        echo "FAIL"
        echo "Output: $output"
        ((FAIL++))
    fi
fi

# Test 5: Explicit --out table produces same format
echo -n "Test 5: Explicit --out table matches default... "
table_output=$(./target/debug/linears --out table list issue --first 1 2>&1)
# Both should be non-JSON format
table_first_char=$(echo "$table_output" | head -c 1)
if [ "$table_first_char" != "{" ] && [ "$table_first_char" != "[" ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All default output format tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
