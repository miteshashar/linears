#!/bin/bash
# Test JSON output envelope format for get command

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing get JSON output envelope format..."
PASS=0
FAIL=0

# Get a valid issue ID first
list_output=$(./target/debug/linears --out json list issue --first 1 2>&1)
issue_id=$(echo "$list_output" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')

if [ -z "$issue_id" ]; then
    echo "No issue found, cannot run get tests"
    exit 1
fi

# Test 1: Get has resource field
echo -n "Test 1: Get has 'resource' field... "
output=$(./target/debug/linears --out json get issue "$issue_id" 2>&1)
if echo "$output" | grep -q '"resource"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 2: Get resource equals 'issue'
echo -n "Test 2: Resource equals 'issue'... "
if echo "$output" | grep -q '"resource":"issue"' || echo "$output" | grep -q '"resource": "issue"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 3: Get has operation field
echo -n "Test 3: Get has 'operation' field... "
if echo "$output" | grep -q '"operation"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 4: Get operation equals 'get'
echo -n "Test 4: Operation equals 'get'... "
if echo "$output" | grep -q '"operation":"get"' || echo "$output" | grep -q '"operation": "get"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 5: Get has entity field
echo -n "Test 5: Get has 'entity' field... "
if echo "$output" | grep -q '"entity"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 6: Entity has id
echo -n "Test 6: Entity has 'id' field... "
if echo "$output" | grep -q '"id"'; then
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
    echo "All get envelope tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
