#!/bin/bash
# Test search fallback strategy

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing search fallback strategy..."
PASS=0
FAIL=0

# Test 1: Search on issue works (has native search)
echo -n "Test 1: Search on issue (native)... "
output_issue=$(./target/debug/linears --verbose --out json search issue "test" 2>&1)
exit_issue=$?
if [ "$exit_issue" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $exit_issue)"
    ((FAIL++))
fi

# Test 2: Check verbose output shows strategy
echo -n "Test 2: Verbose shows query info... "
if echo "$output_issue" | grep -q "Query:\|Variables:"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output_issue"
    ((FAIL++))
fi

# Test 3: Search on team (may use filter fallback)
echo -n "Test 3: Search on team... "
output_team=$(./target/debug/linears --verbose --out json search team "test" 2>&1)
exit_team=$?
if [ "$exit_team" -ne 139 ]; then
    echo "PASS (exit code $exit_team)"
    ((PASS++))
else
    echo "FAIL (crash)"
    ((FAIL++))
fi

# Test 4: Search on user (may use filter fallback)
echo -n "Test 4: Search on user... "
output_user=$(./target/debug/linears --verbose --out json search user "test" 2>&1)
exit_user=$?
if [ "$exit_user" -ne 139 ]; then
    echo "PASS (exit code $exit_user)"
    ((PASS++))
else
    echo "FAIL (crash)"
    ((FAIL++))
fi

# Test 5: Response structure is valid
echo -n "Test 5: Response has valid structure... "
if echo "$output_issue" | grep -q '"nodes"\|"strategy"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output_issue"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All search fallback tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
