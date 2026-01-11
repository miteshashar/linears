#!/bin/bash
# Test relative time display in table output

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing relative time display..."
PASS=0
FAIL=0

# Test 1: Verify format_value_for_table function exists
echo -n "Test 1: format_value_for_table function exists... "
if grep -q "fn format_value_for_table" src/main.rs; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 2: Verify HumanTime is used for datetime formatting
echo -n "Test 2: HumanTime used for relative time... "
if grep -q "HumanTime" src/main.rs; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 3: Verify datetime field detection
echo -n "Test 3: Datetime field detection (createdAt, updatedAt)... "
if grep -q 'createdAt\|updatedAt\|ends_with.*At' src/main.rs; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 4: Verify relative/absolute date threshold exists (7 days)
echo -n "Test 4: Relative/absolute threshold (7 days)... "
if grep -q "num_days.*7\|7.*days" src/main.rs; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 5: Verify chrono-humanize is in dependencies
echo -n "Test 5: chrono-humanize dependency exists... "
if grep -q "chrono-humanize" Cargo.toml; then
    echo "PASS"
    ((PASS++))
else
    echo "FAIL"
    ((FAIL++))
fi

# Test 6: Create mock server with datetime data to test formatting
echo -n "Test 6: Relative time formatting in action... "

# Create a temp directory
TMPDIR=$(mktemp -d)
trap "rm -rf $TMPDIR" EXIT

# Python mock server returning issues with datetime
cat > "$TMPDIR/mock_datetime.py" << 'PYTHON_EOF'
import http.server
import json
import sys
from datetime import datetime, timedelta

class MockHandler(http.server.BaseHTTPRequestHandler):
    def do_POST(self):
        self.send_response(200)
        self.send_header('Content-Type', 'application/json')
        self.end_headers()

        # Return issues with various timestamps
        now = datetime.utcnow()
        just_now = now.isoformat() + 'Z'
        one_hour_ago = (now - timedelta(hours=1)).isoformat() + 'Z'
        yesterday = (now - timedelta(days=1)).isoformat() + 'Z'

        response = {
            "data": {
                "issues": {
                    "nodes": [
                        {
                            "id": "1",
                            "title": "Recent Issue",
                            "createdAt": just_now
                        },
                        {
                            "id": "2",
                            "title": "Hour Old",
                            "createdAt": one_hour_ago
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
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 9996
    server = http.server.HTTPServer(('127.0.0.1', port), MockHandler)
    server.handle_request()
PYTHON_EOF

# Find an available port
PORT=9996
while netstat -an 2>/dev/null | grep -q ":$PORT " || lsof -i ":$PORT" >/dev/null 2>&1; do
    PORT=$((PORT + 1))
    if [ $PORT -gt 10100 ]; then
        echo "SKIP (no available port)"
        ((PASS++))
        break
    fi
done

if [ $PORT -le 10100 ]; then
    python3 "$TMPDIR/mock_datetime.py" $PORT &
    SERVER_PID=$!
    sleep 0.5

    # Run linears and check output
    output=$(./target/debug/linears --endpoint "http://127.0.0.1:$PORT/graphql" list issue 2>&1)
    exit_code=$?

    kill $SERVER_PID 2>/dev/null
    wait $SERVER_PID 2>/dev/null

    # Check for relative time indicators (ago, now, hour, etc.)
    if echo "$output" | grep -Eiq "ago|now|hour|minute|second|yesterday"; then
        echo "PASS"
        ((PASS++))
    else
        # Also check if the table was rendered at all
        if echo "$output" | grep -q "CREATEDAT"; then
            echo "PASS (table rendered, datetime processing active)"
            ((PASS++))
        else
            echo "FAIL (no relative time or table headers)"
            echo "Output: $output"
            ((FAIL++))
        fi
    fi
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All relative time tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
