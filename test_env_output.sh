#!/bin/bash
# Test LINEARS_OUTPUT env var

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing LINEARS_OUTPUT env var..."
PASS=0
FAIL=0

# Test 1: LINEARS_OUTPUT=json produces JSON output
echo -n "Test 1: LINEARS_OUTPUT=json produces JSON output... "
output=$(LINEARS_OUTPUT=json ./target/debug/linears list issue --first 1 2>&1)
if echo "$output" | grep -q '"resource"' && echo "$output" | grep -q '"nodes"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output"
    ((FAIL++))
fi

# Test 2: LINEARS_OUTPUT=yaml produces YAML output
echo -n "Test 2: LINEARS_OUTPUT=yaml produces YAML output... "
output2=$(LINEARS_OUTPUT=yaml ./target/debug/linears list issue --first 1 2>&1)
if echo "$output2" | grep -q "resource:" && echo "$output2" | grep -q "nodes:"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output2"
    ((FAIL++))
fi

# Test 3: --out flag overrides env var
echo -n "Test 3: --out flag overrides env var... "
output3=$(LINEARS_OUTPUT=yaml ./target/debug/linears --out json list issue --first 1 2>&1)
if echo "$output3" | grep -q '"resource"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Output: $output3"
    ((FAIL++))
fi

# Test 4: Help mentions LINEARS_OUTPUT
echo -n "Test 4: Help mentions LINEARS_OUTPUT env var... "
help_output=$(./target/debug/linears --help 2>&1)
if echo "$help_output" | grep -q "LINEARS_OUTPUT"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All env output tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
