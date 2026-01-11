#!/bin/bash
# Test file not found handling

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing file not found handling..."
PASS=0
FAIL=0

# Test 1: Non-existent file returns non-zero exit code
echo -n "Test 1: Non-existent file returns non-zero exit code... "
output=$(./target/debug/linears create issue --input-file nonexistent.json 2>&1)
exit_code=$?
if [ "$exit_code" -ne 0 ]; then
    echo "PASS (exit code $exit_code)"
    ((PASS++))
else
    echo "FAIL (exit code 0)"
    ((FAIL++))
fi

# Test 2: Error message mentions file not found
echo -n "Test 2: Error message mentions file not found... "
if echo "$output" | grep -qi "not found\|no such file\|does not exist\|cannot\|error"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 3: Filename is shown in error
echo -n "Test 3: Filename shown in error message... "
if echo "$output" | grep -q "nonexistent.json\|nonexistent"; then
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
    echo "All file not found tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
