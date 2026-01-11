#!/bin/bash
# Test help flag functionality

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing help functionality..."
PASS=0
FAIL=0

# Test 1: Root help returns exit code 0
echo -n "Test 1: 'linears --help' returns exit code 0... "
./target/debug/linears --help > /dev/null 2>&1
exit_code=$?
if [ "$exit_code" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (expected 0, got $exit_code)"
    ((FAIL++))
fi

# Test 2: Root help shows subcommands
echo -n "Test 2: '--help' shows subcommands... "
output=$(./target/debug/linears --help 2>&1)
if echo "$output" | grep -q "Commands:"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (no Commands: section)"
    ((FAIL++))
fi

# Test 3: Root help shows global flags
echo -n "Test 3: '--help' shows global options... "
output=$(./target/debug/linears --help 2>&1)
if echo "$output" | grep -q "Options:" && echo "$output" | grep -q "\-\-verbose"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (no Options: section or --verbose)"
    ((FAIL++))
fi

# Test 4: Help shows list command
echo -n "Test 4: '--help' shows 'list' command... "
output=$(./target/debug/linears --help 2>&1)
if echo "$output" | grep -q "list"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (no list command)"
    ((FAIL++))
fi

# Test 5: Help shows get command
echo -n "Test 5: '--help' shows 'get' command... "
if echo "$output" | grep -q "get"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (no get command)"
    ((FAIL++))
fi

# Test 6: Help shows resources command
echo -n "Test 6: '--help' shows 'resources' command... "
if echo "$output" | grep -q "resources"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (no resources command)"
    ((FAIL++))
fi

# Test 7: Subcommand help works
echo -n "Test 7: 'linears list --help' returns exit code 0... "
./target/debug/linears list --help > /dev/null 2>&1
exit_code=$?
if [ "$exit_code" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (expected 0, got $exit_code)"
    ((FAIL++))
fi

# Test 8: Subcommand help shows options
echo -n "Test 8: 'linears list --help' shows list options... "
output=$(./target/debug/linears list --help 2>&1)
if echo "$output" | grep -q "\-\-first" && echo "$output" | grep -q "\-\-filter"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (missing list-specific options)"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All help tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
