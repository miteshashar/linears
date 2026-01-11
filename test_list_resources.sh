#!/bin/bash
# Test list command for multiple resources

# Build first
./init.sh build > /dev/null 2>&1

echo "Testing list command for various resources..."
PASS=0
FAIL=0

# Generic test for list resource
test_list_resource() {
    resource="$1"
    expected_field="$2"

    echo -n "Test: list $resource returns valid response... "
    output=$(./target/debug/linears --out json list "$resource" 2>&1)
    exit_code=$?

    if [ "$exit_code" -eq 0 ]; then
        # Check if response has nodes (may be empty array)
        if echo "$output" | grep -q "nodes"; then
            echo "PASS"
            ((PASS++))
            return 0
        else
            echo "FAIL (no nodes in response)"
            echo "Output: $output"
            ((FAIL++))
            return 1
        fi
    else
        echo "FAIL (exit code $exit_code)"
        echo "Output: $output"
        ((FAIL++))
        return 1
    fi
}

# Test each resource type
test_list_resource "issue" "issues"
test_list_resource "team" "teams"
test_list_resource "user" "users"
test_list_resource "project" "projects"
test_list_resource "cycle" "cycles"
test_list_resource "issueLabel" "issueLabels"
test_list_resource "workflowState" "workflowStates"

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -eq 0 ]; then
    echo "All list resource tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
