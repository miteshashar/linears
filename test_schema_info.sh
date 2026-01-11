#!/bin/bash
# Test schema info command

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing schema info command..."
PASS=0
FAIL=0

# Test 1: Schema info command runs
echo -n "Test 1: Schema info command runs... "
output=$(./target/debug/linears schema info 2>&1)
exit_code=$?
if [ "$exit_code" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $exit_code)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 2: Output contains source
echo -n "Test 2: Output contains source... "
if echo "$output" | grep -qi "source"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 3: Output contains commit
echo -n "Test 3: Output contains commit... "
if echo "$output" | grep -qi "commit"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 4: Output contains sync date
echo -n "Test 4: Output contains sync date... "
if echo "$output" | grep -qi "synced"; then
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
    echo "All schema info tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
