#!/bin/bash

# Print information about the execution
echo "=== Test Script 1 ==="
echo "Working Directory: $(pwd)"
echo "Environment Variables:"
echo "  TEST_MODE: $TEST_MODE"
echo "  TEST_VALUE: $TEST_VALUE"

# Simulate some work
for i in {1..3}; do
    echo "Test 1 - Step $i"
    sleep 1
done

echo "Test 1 Complete!"
