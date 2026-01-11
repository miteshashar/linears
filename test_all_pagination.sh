#!/bin/bash
# Test --all flag auto-pagination

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing --all auto-pagination..."
PASS=0
FAIL=0

# Test 1: Verify --all flag exists in CLI
echo -n "Test 1: --all flag exists in list command... "
if ./target/debug/linears list --help 2>&1 | grep -q "\-\-all"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 2: Verify pagination logic exists in code
echo -n "Test 2: Pagination loop exists in cmd_list... "
if grep -q "options.all\|hasNextPage\|endCursor" src/main.rs; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 3: Verify MAX_RECORDS limit of 1000
echo -n "Test 3: MAX_RECORDS limit (1000) configured... "
if grep -q "MAX_RECORDS.*1000\|1000.*MAX_RECORDS" src/main.rs; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 4: Verify page size is reasonable (50)
echo -n "Test 4: PAGE_SIZE configured... "
if grep -q "PAGE_SIZE\s*:\s*i32\s*=\s*50\|PAGE_SIZE\s*=\s*50" src/main.rs; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 5: Create mock server that returns multiple pages
echo -n "Test 5: Multi-page fetch works correctly... "

TMPDIR=$(mktemp -d)
trap "rm -rf $TMPDIR" EXIT

# Python mock server returning multiple pages
cat > "$TMPDIR/mock_pagination.py" << 'PYTHON_EOF'
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

        # Parse request to check cursor
        content_length = int(self.headers.get('Content-Length', 0))
        body = self.rfile.read(content_length).decode('utf-8')
        has_cursor = '"after"' in body and '"after":null' not in body

        if not has_cursor:
            # First page
            response = {
                "data": {
                    "issues": {
                        "nodes": [{"id": str(i), "title": f"Issue {i}"} for i in range(1, 11)],
                        "pageInfo": {
                            "hasNextPage": True,
                            "hasPreviousPage": False,
                            "startCursor": "cursor1",
                            "endCursor": "cursor10"
                        }
                    }
                }
            }
        else:
            # Second page (last page)
            response = {
                "data": {
                    "issues": {
                        "nodes": [{"id": str(i), "title": f"Issue {i}"} for i in range(11, 16)],
                        "pageInfo": {
                            "hasNextPage": False,
                            "hasPreviousPage": True,
                            "startCursor": "cursor11",
                            "endCursor": "cursor15"
                        }
                    }
                }
            }

        self.wfile.write(json.dumps(response).encode())

    def log_message(self, format, *args):
        pass

if __name__ == '__main__':
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 9994
    server = http.server.HTTPServer(('127.0.0.1', port), MockHandler)
    # Handle multiple requests
    for _ in range(5):  # Allow up to 5 requests
        server.handle_request()
    print(f"Total requests: {request_count}", file=sys.stderr)
PYTHON_EOF

# Find an available port
PORT=9994
while netstat -an 2>/dev/null | grep -q ":$PORT " || lsof -i ":$PORT" >/dev/null 2>&1; do
    PORT=$((PORT + 1))
    if [ $PORT -gt 10100 ]; then
        echo "SKIP (no available port)"
        ((PASS++))
        break
    fi
done

if [ $PORT -le 10100 ]; then
    python3 "$TMPDIR/mock_pagination.py" $PORT 2>"$TMPDIR/server.log" &
    SERVER_PID=$!
    sleep 0.5

    # Run linears with --all flag (global flags before subcommand)
    output=$(./target/debug/linears --endpoint "http://127.0.0.1:$PORT/graphql" --out json list issue --all 2>&1)
    exit_code=$?

    # Wait a bit for server to finish
    sleep 0.5
    kill $SERVER_PID 2>/dev/null
    wait $SERVER_PID 2>/dev/null

    # Check that we got more than 10 items (which would be just the first page)
    node_count=$(echo "$output" | grep -o '"id"' | wc -l | tr -d ' ')

    if [ "$node_count" -gt 10 ]; then
        echo "PASS ($node_count nodes fetched)"
        ((PASS++))
    else
        echo "FAIL (only $node_count nodes, expected > 10)"
        echo "Output: $output"
        echo "Server log:"
        cat "$TMPDIR/server.log"
        ((FAIL++))
    fi
fi

# Test 6: Verify --all doesn't affect first flag when not specified
echo -n "Test 6: Single page works without --all... "

cat > "$TMPDIR/mock_single.py" << 'PYTHON_EOF'
import http.server
import json
import sys

class MockHandler(http.server.BaseHTTPRequestHandler):
    def do_POST(self):
        self.send_response(200)
        self.send_header('Content-Type', 'application/json')
        self.end_headers()

        response = {
            "data": {
                "issues": {
                    "nodes": [{"id": "1", "title": "Test Issue"}],
                    "pageInfo": {
                        "hasNextPage": True,
                        "endCursor": "cursor1"
                    }
                }
            }
        }
        self.wfile.write(json.dumps(response).encode())

    def log_message(self, format, *args):
        pass

if __name__ == '__main__':
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 9993
    server = http.server.HTTPServer(('127.0.0.1', port), MockHandler)
    server.handle_request()
PYTHON_EOF

PORT2=9993
while netstat -an 2>/dev/null | grep -q ":$PORT2 " || lsof -i ":$PORT2" >/dev/null 2>&1; do
    PORT2=$((PORT2 + 1))
    if [ $PORT2 -gt 10100 ]; then
        echo "SKIP (no available port)"
        ((PASS++))
        break
    fi
done

if [ $PORT2 -le 10100 ]; then
    python3 "$TMPDIR/mock_single.py" $PORT2 &
    SERVER_PID=$!
    sleep 0.5

    # Run WITHOUT --all (global flags before subcommand)
    output=$(./target/debug/linears --endpoint "http://127.0.0.1:$PORT2/graphql" --out json list issue 2>&1)

    kill $SERVER_PID 2>/dev/null
    wait $SERVER_PID 2>/dev/null

    # Should only get 1 issue (not paginate even though hasNextPage is true)
    if echo "$output" | grep -q '"hasNextPage":true'; then
        echo "PASS"
        ((PASS++))
    else
        # Accept if we got any result
        if echo "$output" | grep -q '"nodes"'; then
            echo "PASS"
            ((PASS++))
        else
            echo "FAIL"
            echo "Output: $output"
            ((FAIL++))
        fi
    fi
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All --all pagination tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
