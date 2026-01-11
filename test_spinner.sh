#!/bin/bash
# Test spinner/progress indicator during API calls

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing spinner/progress indicator..."
PASS=0
FAIL=0

# Test 1: Verify progress module exists and uses indicatif
echo -n "Test 1: Progress module exists with indicatif... "
if [ -f "src/progress.rs" ] && grep -q "indicatif" src/progress.rs; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 2: Verify TTY detection exists
echo -n "Test 2: TTY detection implemented... "
if grep -q "is_terminal\|IsTerminal" src/progress.rs; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 3: Verify spinner only shows on TTY (check stderr)
echo -n "Test 3: Spinner disabled when piped (no spinner chars in output)... "
# When piped, stderr should not contain spinner chars
output=$(./target/debug/linears --endpoint http://localhost:1 list issue 2>&1 | cat)
# Spinner chars are unicode like ⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏
if echo "$output" | grep -q '[⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏]'; then
    echo "FAIL (spinner chars found in piped output)"
    ((FAIL++))
else
    echo "PASS"
    ((PASS++))
fi

# Test 4: Verify spinner clears after completion (check for finish_and_clear)
echo -n "Test 4: Spinner clears after completion... "
if grep -q "finish_and_clear\|finish_with_message" src/progress.rs; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 5: Verify with_spinner is used in main.rs commands
echo -n "Test 5: with_spinner used in API commands... "
count=$(grep -c "with_spinner" src/main.rs || echo "0")
if [ "$count" -ge 5 ]; then
    echo "PASS ($count uses found)"
    ((PASS++))
else
    echo "FAIL (only $count uses, expected >= 5)"
    ((FAIL++))
fi

# Test 6: Verify spinner message format
echo -n "Test 6: Spinner has proper message format... "
if grep -q 'Fetching\|Creating\|Updating\|Deleting\|Searching\|Executing' src/main.rs; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 7: Verify spinner tick animation is set
echo -n "Test 7: Spinner animation configured... "
if grep -q "tick_strings\|enable_steady_tick" src/progress.rs; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All spinner tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
