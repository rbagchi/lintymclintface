#!/bin/bash

LINTER_CMD="./target/release/lintymclintface"
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

passed_files=0
failed_files=0
total_files=0

echo "Running linter on HikariCP project..."
for file in $(find tests/HikariCP -name "*.java"); do
    ((total_files++))
    output=$($LINTER_CMD -l java -f "$file")
    if [ "$(echo -n "$output" | jq 'length')" -eq 0 ]; then
        echo -e "${GREEN}PASS${NC}: $file"
        ((passed_files++))
    else
        echo -e "${RED}FAIL${NC}: $file"
        echo "Expected no errors, but got:"
        echo "$output"
        ((failed_files++))
    fi
done

echo ""
echo "-----------------"
echo "HikariCP test summary:"
echo "Total files: $total_files"
echo -e "${GREEN}Passed: $passed_files${NC}"
echo -e "${RED}Failed: $failed_files${NC}"
echo "-----------------"

if [ "$failed_files" -ne 0 ]; then
    exit 1
fi
