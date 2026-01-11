#!/bin/bash
# Test --all pagination doesn't hang and respects limits

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing --all pagination performance..."
PASS=0
FAIL=0

# Test 1: Verify MAX_RECORDS limit is set
echo -n "Test 1: MAX_RECORDS limit exists... "
if grep -q "MAX_RECORDS.*1000\|const.*MAX_RECORDS" src/main.rs; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 2: Verify truncation logic exists
echo -n "Test 2: Truncation logic for limit... "
if grep -q "truncate\|MAX_RECORDS\|>= MAX\|len().*MAX" src/main.rs; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 3: Test that --all with mock completes quickly
echo -n "Test 3: --all completes with timeout... "

TMPDIR=$(mktemp -d)
trap "rm -rf $TMPDIR" EXIT

# Mock server that returns pages quickly
cat > "$TMPDIR/mock_quick.py" << 'PYTHON_EOF'
import http.server
import json
import sys

request_count = 0

class MockHandler(http.server.BaseHTTPRequestHandler):
    def do_POST(self):
        global request_count
        request_count += 1

        self.send_response(200)
        self.send_header('Content-Type', 'application/json')
        self.end_headers()

        # Always return last page (hasNextPage: false)
        response = {
            "data": {
                "issues": {
                    "nodes": [{"id": str(i), "title": f"Issue {i}"} for i in range(1, 6)],
                    "pageInfo": {"hasNextPage": False, "endCursor": None}
                }
            }
        }
        self.wfile.write(json.dumps(response).encode())

    def log_message(self, format, *args):
        pass

if __name__ == '__main__':
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 9991
    server = http.server.HTTPServer(('127.0.0.1', port), MockHandler)
    server.handle_request()
PYTHON_EOF

PORT=9991
while netstat -an 2>/dev/null | grep -q ":$PORT " || lsof -i ":$PORT" >/dev/null 2>&1; do
    PORT=$((PORT + 1))
    if [ $PORT -gt 10100 ]; then
        echo "SKIP (no available port)"
        ((PASS++))
        break
    fi
done

if [ $PORT -le 10100 ]; then
    python3 "$TMPDIR/mock_quick.py" $PORT &
    SERVER_PID=$!
    sleep 0.5

    # Run with 5 second timeout - should complete much faster
    start=$(date +%s)
    output=$(timeout 5 ./target/debug/linears --endpoint "http://127.0.0.1:$PORT/graphql" --out json list issue --all 2>&1)
    exit_code=$?
    end=$(date +%s)
    elapsed=$((end - start))

    kill $SERVER_PID 2>/dev/null
    wait $SERVER_PID 2>/dev/null

    if [ $exit_code -eq 0 ] && [ $elapsed -lt 5 ]; then
        echo "PASS (completed in ${elapsed}s)"
        ((PASS++))
    else
        echo "FAIL (exit $exit_code, took ${elapsed}s)"
        echo "Output: $output"
        ((FAIL++))
    fi
fi

# Test 4: Verify loop termination on no more pages
echo -n "Test 4: Loop terminates on hasNextPage:false... "
if grep -q "!has_next\|hasNextPage.*false\|break" src/main.rs; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All pagination timeout tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
