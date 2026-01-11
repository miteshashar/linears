#!/bin/bash
# Test --select option for specific fields

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing --select option..."
PASS=0
FAIL=0

# Test 1: --select works and returns data
echo -n "Test 1: --select id,title,identifier works... "
output=$(./target/debug/linears --out json list issue --select id,title,identifier 2>&1)
exit_code=$?
if [ "$exit_code" -eq 0 ] && echo "$output" | grep -q "nodes"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $exit_code)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 2: Verify selected fields are present
echo -n "Test 2: Selected fields (id, title, identifier) present... "
if echo "$output" | grep -q '"id"' && echo "$output" | grep -q '"title"' && echo "$output" | grep -q '"identifier"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 3: Verify other fields are NOT present (e.g., state, assignee, priority)
echo -n "Test 3: Other fields (state, assignee) NOT present... "
if ! echo "$output" | grep -q '"state"' && ! echo "$output" | grep -q '"assignee"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (found extra fields)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 4: Verbose shows select fields in query
echo -n "Test 4: Verbose shows selected fields in query... "
verbose_output=$(./target/debug/linears -v --out json list issue --select id,title,identifier 2>&1)
# Should see "id title identifier" in the query
if echo "$verbose_output" | grep -q "Query:" && echo "$verbose_output" | grep -E "id.*title.*identifier"; then
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
    echo "All --select tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
