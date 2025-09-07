#!/bin/bash

LINTER_CMD="./target/release/lintymclintface"
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

passed_files=0
failed_files=0
total_files=0

echo "Running linter on javapoet project..."
REPO_DIR="tmp_javapoet_repo"

# Clone the repository if it doesn't exist
if [ ! -d "${REPO_DIR}" ]; then
  echo "Cloning JavaPoet into ${REPO_DIR}..."
  git clone https://github.com/square/javapoet.git ${REPO_DIR}
  if [ $? -ne 0 ]; then
    echo "Error: Failed to clone repository."
    exit 1
  fi
else
  echo "Repository ${REPO_DIR} already exists. Skipping clone."
fi

for file in $(find ${REPO_DIR} -name "*.java"); do
    ((total_files++))
    output=$($LINTER_CMD -l java -f "$file")
    if [ -z "$output" ]; then
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
echo "Javapoet test summary:"
echo "Total files: $total_files"
echo -e "${GREEN}Passed: $passed_files${NC}"
echo -e "${RED}Failed: $failed_files${NC}"
echo "-----------------"

if [ "$failed_files" -ne 0 ]; then
    exit 1
fi
