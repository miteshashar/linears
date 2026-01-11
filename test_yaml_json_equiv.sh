#!/bin/bash
# Test YAML output matches JSON structure

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing YAML/JSON structure equivalence..."
PASS=0
FAIL=0

# Test 1: List - YAML has same fields as JSON
echo -n "Test 1: List - YAML has resource field like JSON... "
json_output=$(./target/debug/linears --out json list issue --first 1 2>&1)
yaml_output=$(./target/debug/linears --out yaml list issue --first 1 2>&1)

if echo "$yaml_output" | grep -q "resource:" && echo "$json_output" | grep -q '"resource"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 2: YAML has operation field
echo -n "Test 2: YAML has operation field... "
if echo "$yaml_output" | grep -q "operation:"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "YAML Output: $yaml_output"
    ((FAIL++))
fi

# Test 3: YAML has nodes field
echo -n "Test 3: YAML has nodes field... "
if echo "$yaml_output" | grep -q "nodes:"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "YAML Output: $yaml_output"
    ((FAIL++))
fi

# Test 4: YAML has pageInfo field
echo -n "Test 4: YAML has pageInfo field... "
if echo "$yaml_output" | grep -q "pageInfo:"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "YAML Output: $yaml_output"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All YAML/JSON equivalence tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
