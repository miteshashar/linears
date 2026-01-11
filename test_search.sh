#!/bin/bash
# Test search command functionality

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing search functionality..."
PASS=0
FAIL=0

# Test 1: Search requires API key
echo -n "Test 1: Search without API key returns exit code 2... "
LINEARS_API_KEY= ./target/debug/linears search issue "test" > /dev/null 2>&1
exit_code=$?
if [ "$exit_code" -eq 2 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (expected 2, got $exit_code)"
    ((FAIL++))
fi

# Test 2: Search with valid API key returns data (exit code 0)
echo -n "Test 2: Search with valid API key returns exit code 0... "
output=$(./target/debug/linears --out json search issue "test" 2>&1)
exit_code=$?
if [ "$exit_code" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (expected 0, got $exit_code)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 3: Search returns JSON with nodes array (envelope uses "nodes" for list data)
echo -n "Test 3: Search returns JSON with nodes array... "
output=$(./target/debug/linears --out json search issue "test" 2>&1)
if echo "$output" | grep -q '"nodes"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (no nodes in output)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 4: Search for a specific term returns results
echo -n "Test 4: Search returns nodes array... "
output=$(./target/debug/linears --out json search issue "test" 2>&1)
if echo "$output" | grep -q "nodes"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (no nodes in output)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 5: Verbose mode shows search strategy
echo -n "Test 5: Verbose mode shows search strategy... "
output=$(./target/debug/linears -v --out json search issue "test" 2>&1)
if echo "$output" | grep -q "Search strategy"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (no Search strategy in verbose output)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 6: Search for team uses filter strategy
echo -n "Test 6: Team search uses filter strategy... "
output=$(./target/debug/linears -v --out json search team "eng" 2>&1)
if echo "$output" | grep -q "FilterHeuristic"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (team search should use FilterHeuristic)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 7: Issue search now uses filter strategy (issueSearch was deprecated)
echo -n "Test 7: Issue search uses filter strategy... "
output=$(./target/debug/linears -v --out json search issue "test" 2>&1)
if echo "$output" | grep -q "FilterHeuristic"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (issue search should use FilterHeuristic)"
    echo "Output: $output"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All search tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
