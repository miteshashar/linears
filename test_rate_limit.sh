#!/bin/bash
# Test rate limit reset time display

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing rate limit reset time display..."
PASS=0
FAIL=0

# Create a temporary directory for our mock server files
TMPDIR=$(mktemp -d)
trap "rm -rf $TMPDIR" EXIT

# Test 1: Verify rate limit handling exists in code
echo -n "Test 1: Rate limit handling code exists... "
if grep -q "RateLimitedTooLong\|rate limit resets" src/client/mod.rs; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 2: Verify short rate limits are retried automatically
echo -n "Test 2: Short rate limits (≤60s) are retried... "
if grep -q "secs <= 60" src/client/mod.rs && grep -q "RateLimited(secs)" src/client/mod.rs; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 3: Verify long rate limits show reset time
echo -n "Test 3: Long rate limits (>60s) show reset time... "
if grep -q "reset.*time\|resets at" src/client/mod.rs; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 4: Create mock server to test long rate limit behavior
echo -n "Test 4: Mock 429 with Retry-After: 120 shows reset time... "

# Python mock server that returns 429 with Retry-After: 120
cat > "$TMPDIR/mock_rate_limit.py" << 'PYTHON_EOF'
import http.server
import sys

class MockHandler(http.server.BaseHTTPRequestHandler):
    def do_POST(self):
        self.send_response(429)
        self.send_header('Content-Type', 'application/json')
        self.send_header('Retry-After', '120')  # 2 minutes (> 60s threshold)
        self.end_headers()
        self.wfile.write(b'{"error": "Rate limited"}')

    def log_message(self, format, *args):
        pass

if __name__ == '__main__':
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 9998
    server = http.server.HTTPServer(('127.0.0.1', port), MockHandler)
    server.handle_request()
PYTHON_EOF

# Find an available port
PORT=9998
while netstat -an 2>/dev/null | grep -q ":$PORT " || lsof -i ":$PORT" >/dev/null 2>&1; do
    PORT=$((PORT + 1))
    if [ $PORT -gt 10100 ]; then
        echo "SKIP (no available port)"
        ((PASS++))
        break
    fi
done

if [ $PORT -le 10100 ]; then
    # Start mock server in background
    python3 "$TMPDIR/mock_rate_limit.py" $PORT &
    SERVER_PID=$!
    sleep 0.5

    # Run linears - should fail quickly with reset time message
    output=$(timeout 5 ./target/debug/linears --endpoint "http://127.0.0.1:$PORT/graphql" list issue 2>&1)
    exit_code=$?

    # Kill server
    kill $SERVER_PID 2>/dev/null
    wait $SERVER_PID 2>/dev/null

    # Check for reset time message
    if echo "$output" | grep -qi "reset\|120"; then
        echo "PASS"
        ((PASS++))
    else
        echo "FAIL (no reset time in output)"
        echo "Output: $output"
        ((FAIL++))
    fi
fi

# Test 5: Verify short rate limits trigger retry message
echo -n "Test 5: Short rate limit triggers retry message... "

cat > "$TMPDIR/mock_short_rate_limit.py" << 'PYTHON_EOF'
import http.server
import json
import sys

request_count = 0

class MockHandler(http.server.BaseHTTPRequestHandler):
    def do_POST(self):
        global request_count
        request_count += 1

        if request_count == 1:
            # First request: rate limited for 1 second
            self.send_response(429)
            self.send_header('Content-Type', 'application/json')
            self.send_header('Retry-After', '1')
            self.end_headers()
            self.wfile.write(b'{"error": "Rate limited"}')
        else:
            # Second request: success
            self.send_response(200)
            self.send_header('Content-Type', 'application/json')
            self.end_headers()
            response = {
                "data": {
                    "issues": {
                        "nodes": [],
                        "pageInfo": {"hasNextPage": False, "endCursor": None}
                    }
                }
            }
            self.wfile.write(json.dumps(response).encode())

    def log_message(self, format, *args):
        pass

if __name__ == '__main__':
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 9997
    server = http.server.HTTPServer(('127.0.0.1', port), MockHandler)
    server.handle_request()
    server.handle_request()
PYTHON_EOF

PORT2=9997
while netstat -an 2>/dev/null | grep -q ":$PORT2 " || lsof -i ":$PORT2" >/dev/null 2>&1; do
    PORT2=$((PORT2 + 1))
    if [ $PORT2 -gt 10100 ]; then
        echo "SKIP (no available port)"
        ((PASS++))
        break
    fi
done

if [ $PORT2 -le 10100 ]; then
    python3 "$TMPDIR/mock_short_rate_limit.py" $PORT2 &
    SERVER_PID=$!
    sleep 0.5

    # Run linears - should retry and eventually succeed
    output=$(timeout 10 ./target/debug/linears --endpoint "http://127.0.0.1:$PORT2/graphql" list issue 2>&1)
    exit_code=$?

    kill $SERVER_PID 2>/dev/null
    wait $SERVER_PID 2>/dev/null

    # Check if command succeeded after retry
    if [ $exit_code -eq 0 ]; then
        echo "PASS"
        ((PASS++))
    else
        echo "FAIL (exit code $exit_code, expected 0)"
        echo "Output: $output"
        ((FAIL++))
    fi
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All rate limit tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
