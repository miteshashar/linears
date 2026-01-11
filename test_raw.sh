#!/bin/bash
# Test raw GraphQL query command

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing raw GraphQL command..."
PASS=0
FAIL=0

# Test 1: Raw inline query works
echo -n "Test 1: Raw inline query works... "
output=$(./target/debug/linears --out json raw --query 'query { viewer { id name } }' 2>&1)
exit_code=$?
if [ "$exit_code" -eq 0 ] && echo "$output" | grep -q "viewer"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $exit_code)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 2: Raw query returns viewer data
echo -n "Test 2: Raw query returns viewer data... "
output=$(./target/debug/linears --out json raw --query 'query { viewer { id name email } }' 2>&1)
if echo "$output" | grep -q '"id"' && echo "$output" | grep -q '"name"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (no id/name in output)"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 3: Raw query from file works
echo -n "Test 3: Raw query from file works... "
# Create a temp query file
query_file=$(mktemp)
echo 'query { organization { id name urlKey } }' > "$query_file"
output=$(./target/debug/linears --out json raw --query "$query_file" 2>&1)
rm -f "$query_file"
if [ "$exit_code" -eq 0 ] && echo "$output" | grep -q "organization"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 4: Complex query works
echo -n "Test 4: Complex query with nested fields works... "
output=$(./target/debug/linears --out json raw --query 'query { teams { nodes { id name key } } }' 2>&1)
if echo "$output" | grep -q "teams" && echo "$output" | grep -q "nodes"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 5: Verbose shows query
echo -n "Test 5: Verbose shows the query... "
output=$(./target/debug/linears -v --out json raw --query 'query { viewer { id } }' 2>&1)
if echo "$output" | grep -q "Query:"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (no Query: in verbose output)"
    echo "Output: $output"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All raw query tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
