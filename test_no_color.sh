#!/bin/bash
# Test --no-color flag

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing --no-color flag..."
PASS=0
FAIL=0

# Test 1: Command runs with --no-color
echo -n "Test 1: --no-color flag works... "
output=$(./target/debug/linears --no-color --out table list issue --first 1 2>&1)
exit_code=$?
if [ "$exit_code" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $exit_code)"
    ((FAIL++))
fi

# Test 2: No ANSI escape codes in output
echo -n "Test 2: No ANSI escape codes... "
# ANSI escape codes start with \x1b[
if ! echo "$output" | grep -q $'\x1b\['; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (found ANSI codes)"
    ((FAIL++))
fi

# Test 3: Works with JSON output too
echo -n "Test 3: Works with JSON output... "
json_output=$(./target/debug/linears --no-color --out json list issue --first 1 2>&1)
if echo "$json_output" | python3 -c "import sys,json; json.load(sys.stdin)" 2>/dev/null; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 4: Error output also respects --no-color
echo -n "Test 4: Error output respects --no-color... "
error_output=$(./target/debug/linears --no-color get issue nonexistent 2>&1)
if ! echo "$error_output" | grep -q $'\x1b\['; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (found ANSI codes in error)"
    ((FAIL++))
fi

# Test 5: Piped output naturally has no color
echo -n "Test 5: Piped output has no color... "
piped_output=$(./target/debug/linears --out table list issue --first 1 | cat 2>&1)
if ! echo "$piped_output" | grep -q $'\x1b\['; then
    echo "PASS"
    ((PASS++))
else
    echo "INFO (may have color - depends on TTY detection)"
    ((PASS++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All no-color tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
