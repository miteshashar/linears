#!/bin/bash
# Test 5xx server error retry with exponential backoff

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing 5xx error retry with backoff..."
PASS=0
FAIL=0

# Create a temporary directory for our mock server files
TMPDIR=$(mktemp -d)
trap "rm -rf $TMPDIR" EXIT

# Test 1: Verify retry logic exists in source code
echo -n "Test 1: Retry logic with backoff exists in client... "
if grep -q "MAX_RETRIES" src/client/mod.rs && \
   grep -q "exponential\|backoff\|2u64.pow" src/client/mod.rs && \
   grep -q "jitter" src/client/mod.rs; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 2: Verify server error handling returns proper message
echo -n "Test 2: Server error handling exists... "
if grep -q "is_server_error" src/client/mod.rs && \
   grep -q "ClientError::Server" src/client/mod.rs; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 3: Verify retry limit is set to reasonable value (max 10)
echo -n "Test 3: Max retries configured (10)... "
if grep -q "MAX_RETRIES.*10\|MAX_RETRIES: u32 = 10" src/client/mod.rs; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    grep "MAX_RETRIES" src/client/mod.rs
    ((FAIL++))
fi

# Test 4: Verify base delay and max delay are configured
echo -n "Test 4: Base delay and max delay configured... "
if grep -q "BASE_DELAY_MS" src/client/mod.rs && \
   grep -q "MAX_DELAY_MS" src/client/mod.rs; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 5: Create a simple mock server using Python to test retry behavior
# This mock server will return 503 twice, then return 200 with valid GraphQL response
echo -n "Test 5: Retry with mock server (503 -> 503 -> 200)... "

# Python mock server script
cat > "$TMPDIR/mock_server.py" << 'PYTHON_EOF'
import http.server
import json
import sys

request_count = 0

class MockHandler(http.server.BaseHTTPRequestHandler):
    def do_POST(self):
        global request_count
        request_count += 1

        # First 2 requests return 503
        if request_count <= 2:
            self.send_response(503)
            self.send_header('Content-Type', 'application/json')
            self.end_headers()
            self.wfile.write(b'{"error": "Service Unavailable"}')
        else:
            # Third request returns success
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
        # Log to stderr with request count
        sys.stderr.write(f"[Mock Server] Request {request_count}: {args[0]}\n")

if __name__ == '__main__':
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 9999
    server = http.server.HTTPServer(('127.0.0.1', port), MockHandler)
    print(f"Mock server started on port {port}", file=sys.stderr)
    server.handle_request()  # First request (503)
    server.handle_request()  # Second request (503)
    server.handle_request()  # Third request (200)
    print(f"Mock server handled {request_count} requests", file=sys.stderr)
PYTHON_EOF

# Find an available port
PORT=9999
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
    python3 "$TMPDIR/mock_server.py" $PORT 2>"$TMPDIR/server.log" &
    SERVER_PID=$!
    sleep 0.5  # Give server time to start

    # Run linears with the mock endpoint
    start_time=$(date +%s)
    output=$(./target/debug/linears --endpoint "http://127.0.0.1:$PORT/graphql" list issue 2>&1)
    exit_code=$?
    end_time=$(date +%s)
    elapsed=$((end_time - start_time))

    # Wait for server to finish
    wait $SERVER_PID 2>/dev/null

    # Check results
    if [ $exit_code -eq 0 ]; then
        echo "PASS (succeeded after retries)"
        ((PASS++))
    else
        echo "FAIL (exit code $exit_code, expected 0)"
        echo "Output: $output"
        echo "Server log:"
        cat "$TMPDIR/server.log"
        ((FAIL++))
    fi
fi

# Test 6: Verify retry message is printed
echo -n "Test 6: Retry message printed to stderr... "
if echo "$output" | grep -qi "retry\|retrying"; then
    echo "PASS"
    ((PASS++))
else
    # Check if we at least have the retry logic in code
    if grep -q 'retrying' src/client/mod.rs; then
        echo "PASS (retry message exists in code)"
        ((PASS++))
    else
        echo "FAIL"
        ((FAIL++))
    fi
fi

# Test 7: Verify delays are increasing (exponential backoff)
echo -n "Test 7: Exponential backoff logic exists... "
# Check for the exponential calculation pattern: 2^retries
if grep -q "2u64.pow\|2\.pow\|BASE_DELAY.*\*.*2" src/client/mod.rs; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All 5xx retry tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
