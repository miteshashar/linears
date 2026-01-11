#!/bin/bash
# Test default preset behavior

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing default preset behavior..."
PASS=0
FAIL=0

# Test 1: List without --preset
echo -n "Test 1: List without --preset runs... "
output_default=$(./target/debug/linears --out json list issue --first 1 2>&1)
exit_code1=$?
if [ "$exit_code1" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $exit_code1)"
    ((FAIL++))
fi

# Test 2: List with --preset default
echo -n "Test 2: List with --preset default runs... "
output_preset=$(./target/debug/linears --out json list issue --first 1 --preset default 2>&1)
exit_code2=$?
if [ "$exit_code2" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL (exit code $exit_code2)"
    ((FAIL++))
fi

# Test 3: Both have same fields
echo -n "Test 3: Same fields selected... "
# Extract keys from first node in each response
keys_default=$(echo "$output_default" | python3 -c "
import sys, json
try:
    d = json.load(sys.stdin)
    nodes = d.get('nodes', [])
    if nodes:
        print(','.join(sorted(nodes[0].keys())))
    else:
        print('')
except:
    print('')
" 2>/dev/null)

keys_preset=$(echo "$output_preset" | python3 -c "
import sys, json
try:
    d = json.load(sys.stdin)
    nodes = d.get('nodes', [])
    if nodes:
        print(','.join(sorted(nodes[0].keys())))
    else:
        print('')
except:
    print('')
" 2>/dev/null)

if [ "$keys_default" = "$keys_preset" ] && [ -n "$keys_default" ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Default keys: $keys_default"
    echo "Preset keys: $keys_preset"
    ((FAIL++))
fi

# Test 4: Default includes common fields (id, title, state)
echo -n "Test 4: Default includes common fields... "
if echo "$keys_default" | grep -q "id" && echo "$keys_default" | grep -q "title"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    echo "Keys: $keys_default"
    ((FAIL++))
fi

# Test 5: Minimal preset has fewer fields
echo -n "Test 5: Minimal preset has fewer fields... "
output_minimal=$(./target/debug/linears --out json list issue --first 1 --preset minimal 2>&1)
keys_minimal=$(echo "$output_minimal" | python3 -c "
import sys, json
try:
    d = json.load(sys.stdin)
    nodes = d.get('nodes', [])
    if nodes:
        print(','.join(sorted(nodes[0].keys())))
    else:
        print('')
except:
    print('')
" 2>/dev/null)

# Count fields
count_default=$(echo "$keys_default" | tr ',' '\n' | wc -l | tr -d ' ')
count_minimal=$(echo "$keys_minimal" | tr ',' '\n' | wc -l | tr -d ' ')

if [ "$count_minimal" -le "$count_default" ]; then
    echo "PASS (minimal: $count_minimal, default: $count_default)"
    ((PASS++))
else
    echo "FAIL (minimal: $count_minimal > default: $count_default)"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All default preset tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
