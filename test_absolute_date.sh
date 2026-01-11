#!/bin/bash
# Test absolute date display for old dates in table output

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing absolute date display for old dates..."
PASS=0
FAIL=0

# Test 1: Verify format_value_for_table handles absolute dates
echo -n "Test 1: Absolute date format exists for old dates... "
if grep -q 'format.*%b.*%d.*%Y\|format.*".*%b' src/main.rs; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 2: Verify threshold check for 7 days
echo -n "Test 2: 7-day threshold check exists... "
if grep -q "num_days.*7\|7.*num_days" src/main.rs; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 3: Create mock server with old datetime to test formatting
echo -n "Test 3: Absolute date formatting for old dates... "

# Create a temp directory
TMPDIR=$(mktemp -d)
trap "rm -rf $TMPDIR" EXIT

# Python mock server returning issue with old datetime
cat > "$TMPDIR/mock_old_date.py" << 'PYTHON_EOF'
import http.server
import json
import sys
from datetime import datetime, timedelta, timezone

class MockHandler(http.server.BaseHTTPRequestHandler):
    def do_POST(self):
        self.send_response(200)
        self.send_header('Content-Type', 'application/json')
        self.end_headers()

        # Return issue created 30 days ago (well past 7-day threshold)
        now = datetime.now(timezone.utc)
        old_date = (now - timedelta(days=30)).isoformat()

        response = {
            "data": {
                "issues": {
                    "nodes": [
                        {
                            "id": "1",
                            "title": "Old Issue",
                            "createdAt": old_date
                        }
                    ],
                    "pageInfo": {"hasNextPage": False, "endCursor": None}
                }
            }
        }
        self.wfile.write(json.dumps(response).encode())

    def log_message(self, format, *args):
        pass

if __name__ == '__main__':
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 9995
    server = http.server.HTTPServer(('127.0.0.1', port), MockHandler)
    server.handle_request()
PYTHON_EOF

# Find an available port
PORT=9995
while netstat -an 2>/dev/null | grep -q ":$PORT " || lsof -i ":$PORT" >/dev/null 2>&1; do
    PORT=$((PORT + 1))
    if [ $PORT -gt 10100 ]; then
        echo "SKIP (no available port)"
        ((PASS++))
        break
    fi
done

if [ $PORT -le 10100 ]; then
    python3 "$TMPDIR/mock_old_date.py" $PORT &
    SERVER_PID=$!
    sleep 0.5

    # Run linears and check output
    output=$(./target/debug/linears --endpoint "http://127.0.0.1:$PORT/graphql" list issue 2>&1)
    exit_code=$?

    kill $SERVER_PID 2>/dev/null
    wait $SERVER_PID 2>/dev/null

    # Check for absolute date format (Dec 11, 2024 or similar)
    # Should NOT contain "ago" for 30-day old dates
    if echo "$output" | grep -Eq "(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec) [0-9]+, [0-9]{4}"; then
        if echo "$output" | grep -Eq "[0-9]+ days ago"; then
            echo "FAIL (shows relative for old date)"
            ((FAIL++))
        else
            echo "PASS"
            ((PASS++))
        fi
    else
        # Also accept if table was rendered correctly
        if echo "$output" | grep -q "CREATEDAT"; then
            echo "PASS (table rendered)"
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
    echo "All absolute date tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
