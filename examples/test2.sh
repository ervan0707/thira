#!/bin/bash

# Print information about the execution
echo "=== Test Script 2 ==="
echo "Working Directory: $(pwd)"
echo "Environment Variables:"
echo "  TEST_MODE: $TEST_MODE"
echo "  TEST_VALUE: $TEST_VALUE"

# Simulate some work
for i in {1..5}; do
    echo "Test 2 - Step $i"
    sleep 1
done

echo "Test 2 Complete!"
