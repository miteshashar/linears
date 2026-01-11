#!/bin/bash
# Test schema metadata persistence

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing schema metadata persistence..."
PASS=0
FAIL=0

# Test 1: schema.meta.json file exists
echo -n "Test 1: schema.meta.json file exists... "
if [ -f "schemas/linear/schema.meta.json" ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 2: Meta file contains commit hash
echo -n "Test 2: Meta file contains commit hash... "
meta_content=$(cat schemas/linear/schema.meta.json 2>/dev/null)
if echo "$meta_content" | grep -q '"commit"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 3: Meta file contains sync date
echo -n "Test 3: Meta file contains sync date... "
if echo "$meta_content" | grep -q '"syncedAt"'; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 4: schema info command shows same commit
echo -n "Test 4: Schema info command shows commit from file... "
file_commit=$(echo "$meta_content" | grep -o '"commit":"[^"]*"' | sed 's/"commit":"//;s/"//')
info_output=$(./target/debug/linears schema info 2>&1)
if echo "$info_output" | grep -q "$file_commit"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "File commit: $file_commit"
    echo "Info output: $info_output"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All schema metadata tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
