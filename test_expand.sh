#!/bin/bash
# Test --expand option for relation expansion

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing --expand option..."
PASS=0
FAIL=0

# Test 1: --expand team works
echo -n "Test 1: --expand team works... "
output=$(./target/debug/linears --out json list issue --expand team 2>&1)
exit_code=$?
if [ "$exit_code" -eq 0 ] && echo "$output" | grep -q "nodes"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $exit_code)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 2: Verify team object is present in results
echo -n "Test 2: Team object present in results... "
if echo "$output" | grep -q '"team"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 3: --expand team:name,key with specific fields
echo -n "Test 3: --expand team:name,key works... "
output2=$(./target/debug/linears --out json list issue --select id,identifier --expand team:name,key 2>&1)
exit_code=$?
if [ "$exit_code" -eq 0 ] && echo "$output2" | grep -q "nodes"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $exit_code)"
    echo "Output: $output2"
    ((FAIL++))
fi

# Test 4: Verify team has name and key fields
echo -n "Test 4: Team object has name and key fields... "
if echo "$output2" | grep -q '"team"' && echo "$output2" | grep -q '"name"' && echo "$output2" | grep -q '"key"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output2"
    ((FAIL++))
fi

# Test 5: Verbose shows expanded fields in query
echo -n "Test 5: Verbose shows expanded fields in query... "
verbose_output=$(./target/debug/linears -v --out json list issue --select id,identifier --expand team:name,key 2>&1)
if echo "$verbose_output" | grep -q "team {" && echo "$verbose_output" | grep -q "name key"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Verbose Output: $verbose_output"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All --expand tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
