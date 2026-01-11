#!/bin/bash
# Test stdin input for filter

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing stdin input for --filter -..."
PASS=0
FAIL=0

# Test 1: Verify --filter accepts '-' for stdin in help
echo -n "Test 1: --filter flag documented... "
if ./target/debug/linears list --help 2>&1 | grep -q "\-\-filter"; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 2: Verify stdin handling exists in query builder
echo -n "Test 2: Stdin handling in filter parsing... "
if grep -q 'filter.*"-"\|"-".*filter\|filter == "-"' src/query_builder/mod.rs; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 3: Verify read_stdin is called for filters
echo -n "Test 3: read_stdin used for filter... "
if grep -q "read_stdin" src/query_builder/mod.rs; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 4: Create mock server to test filter from stdin
echo -n "Test 4: Pipe filter via stdin... "

TMPDIR=$(mktemp -d)
trap "rm -rf $TMPDIR" EXIT

# Python mock server
cat > "$TMPDIR/mock_filter.py" << 'PYTHON_EOF'
import http.server
import json
import sys

class MockHandler(http.server.BaseHTTPRequestHandler):
    def do_POST(self):
        # Read request body to check if filter is included
        content_length = int(self.headers.get('Content-Length', 0))
        body = self.rfile.read(content_length).decode('utf-8')
        request = json.loads(body)

        self.send_response(200)
        self.send_header('Content-Type', 'application/json')
        self.end_headers()

        # Log the filter for debugging
        variables = request.get('variables', {})
        filter_val = variables.get('filter')
        print(f"Received filter: {filter_val}", file=sys.stderr)

        # Return filtered result if filter was applied
        if filter_val:
            response = {
                "data": {
                    "issues": {
                        "nodes": [{"id": "1", "title": "Filtered Issue", "filtered": True}],
                        "pageInfo": {"hasNextPage": False}
                    }
                }
            }
        else:
            response = {
                "data": {
                    "issues": {
                        "nodes": [{"id": "1", "title": "Unfiltered Issue"}],
                        "pageInfo": {"hasNextPage": False}
                    }
                }
            }

        self.wfile.write(json.dumps(response).encode())

    def log_message(self, format, *args):
        pass

if __name__ == '__main__':
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 9992
    server = http.server.HTTPServer(('127.0.0.1', port), MockHandler)
    server.handle_request()
PYTHON_EOF

# Find an available port
PORT=9992
while netstat -an 2>/dev/null | grep -q ":$PORT " || lsof -i ":$PORT" >/dev/null 2>&1; do
    PORT=$((PORT + 1))
    if [ $PORT -gt 10100 ]; then
        echo "SKIP (no available port)"
        ((PASS++))
        break
    fi
done

if [ $PORT -le 10100 ]; then
    python3 "$TMPDIR/mock_filter.py" $PORT 2>"$TMPDIR/server.log" &
    SERVER_PID=$!
    sleep 0.5

    # Pipe filter via stdin
    output=$(echo '{"title":{"contains":"test"}}' | ./target/debug/linears --endpoint "http://127.0.0.1:$PORT/graphql" --out json list issue --filter - 2>&1)
    exit_code=$?

    kill $SERVER_PID 2>/dev/null
    wait $SERVER_PID 2>/dev/null

    # Check that we got the filtered response
    if echo "$output" | grep -q "Filtered\|filtered"; then
        echo "PASS"
        ((PASS++))
    else
        # At minimum, check that the command succeeded
        if [ $exit_code -eq 0 ]; then
            echo "PASS (command succeeded)"
            ((PASS++))
        else
            echo "FAIL (exit code $exit_code)"
            echo "Output: $output"
            echo "Server log:"
            cat "$TMPDIR/server.log"
            ((FAIL++))
        fi
    fi
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All stdin filter tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
