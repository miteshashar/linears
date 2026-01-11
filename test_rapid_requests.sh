#!/bin/bash
# Test rapid sequential requests

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing rapid sequential requests..."
PASS=0
FAIL=0

# Test 1: 5 rapid list commands
echo -n "Test 1: 5 rapid list commands... "
success_count=0
for i in 1 2 3 4 5; do
    ./target/debug/linears --out json list issue --first 1 > /dev/null 2>&1
    if [ $? -eq 0 ]; then
        ((success_count++))
    fi
done
if [ "$success_count" -ge 4 ]; then
    echo "PASS ($success_count/5 succeeded)"
    ((PASS++))
else
    echo "FAIL (only $success_count/5 succeeded)"
    ((FAIL++))
fi

# Test 2: 5 rapid search commands
echo -n "Test 2: 5 rapid search commands... "
success_count=0
for i in 1 2 3 4 5; do
    ./target/debug/linears --out json search issue "test" > /dev/null 2>&1
    if [ $? -eq 0 ]; then
        ((success_count++))
    fi
done
if [ "$success_count" -ge 4 ]; then
    echo "PASS ($success_count/5 succeeded)"
    ((PASS++))
else
    echo "FAIL (only $success_count/5 succeeded)"
    ((FAIL++))
fi

# Test 3: No crashes in rapid requests
echo -n "Test 3: No crashes (SIGSEGV)... "
crash_count=0
for i in 1 2 3; do
    ./target/debug/linears --out json list team > /dev/null 2>&1
    if [ $? -eq 139 ]; then
        ((crash_count++))
    fi
done
if [ "$crash_count" -eq 0 ]; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL ($crash_count crashes)"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All rapid request tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
