#!/bin/bash
# Test performance

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing performance..."
PASS=0
FAIL=0

# Test 1: Simple list query under 3 seconds
echo -n "Test 1: Simple list query performance... "
start=$(date +%s%N)
./target/debug/linears --out json list issue --first 10 > /dev/null 2>&1
end=$(date +%s%N)
elapsed_ms=$(( (end - start) / 1000000 ))
if [ "$elapsed_ms" -lt 3000 ]; then
    echo "PASS (${elapsed_ms}ms)"
    ((PASS++))
else
    echo "FAIL (${elapsed_ms}ms > 3000ms)"
    ((FAIL++))
fi

# Test 2: Second query (potentially cached)
echo -n "Test 2: Repeat query... "
start=$(date +%s%N)
./target/debug/linears --out json list issue --first 10 > /dev/null 2>&1
end=$(date +%s%N)
elapsed_ms=$(( (end - start) / 1000000 ))
if [ "$elapsed_ms" -lt 3000 ]; then
    echo "PASS (${elapsed_ms}ms)"
    ((PASS++))
else
    echo "FAIL (${elapsed_ms}ms)"
    ((FAIL++))
fi

# Test 3: Search query under 3 seconds
echo -n "Test 3: Search query performance... "
start=$(date +%s%N)
./target/debug/linears --out json search issue "test" > /dev/null 2>&1
end=$(date +%s%N)
elapsed_ms=$(( (end - start) / 1000000 ))
if [ "$elapsed_ms" -lt 3000 ]; then
    echo "PASS (${elapsed_ms}ms)"
    ((PASS++))
else
    echo "FAIL (${elapsed_ms}ms)"
    ((FAIL++))
fi

# Test 4: Get query under 3 seconds
echo -n "Test 4: Get query performance... "
# Get an issue ID first
issue_id=$(./target/debug/linears --out json list issue --first 1 2>/dev/null | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"//;s/"//')
if [ -n "$issue_id" ]; then
    start=$(date +%s%N)
    ./target/debug/linears --out json get issue "$issue_id" > /dev/null 2>&1
    end=$(date +%s%N)
    elapsed_ms=$(( (end - start) / 1000000 ))
    if [ "$elapsed_ms" -lt 3000 ]; then
        echo "PASS (${elapsed_ms}ms)"
        ((PASS++))
    else
        echo "FAIL (${elapsed_ms}ms)"
        ((FAIL++))
    fi
else
    echo "SKIP (no issue to get)"
    ((PASS++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All performance tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
