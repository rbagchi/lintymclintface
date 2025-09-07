#!/bin/bash

# Check for jq
if ! command -v jq &> /dev/null
then
    echo "jq could not be found. Please install jq to run the tests."
    exit 1
fi

LINTER_CMD="./target/release/lintymclintface"
if [ ! -f "$LINTER_CMD" ]; then
    echo "Error: Linter binary not found at $LINTER_CMD"
    echo "Please build the project first by running 'cargo build --release'"
    exit 1
fi

GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

passed_tests=0
failed_tests=0

echo "Running tests for working files..."
for file in tests/working/*.java; do
    output=$($LINTER_CMD -l java -f "$file")
    if [ -z "$output" ]; then
        echo -e "${GREEN}PASS${NC}: $file"
        ((passed_tests++))
    else
        echo -e "${RED}FAIL${NC}: $file"
        echo "Expected no errors, but got:"
        echo "$output"
        ((failed_tests++))
    fi
done

echo ""
echo "Running tests for failing files..."
for file in tests/failing/*.java; do
    output=$($LINTER_CMD -l java -f "$file")
    if [ -n "$output" ]; then
        echo -e "${GREEN}PASS${NC}: $file"
        ((passed_tests++))
    else
        echo -e "${RED}FAIL${NC}: $file"
        echo "Expected errors, but got none."
        ((failed_tests++))
    fi
done

echo ""
echo "-----------------"
echo "Test summary:"
echo -e "${GREEN}Passed: $passed_tests${NC}"
echo -e "${RED}Failed: $failed_tests${NC}"
echo "-----------------"

if [ "$failed_tests" -ne 0 ]; then
    exit 1
fi
