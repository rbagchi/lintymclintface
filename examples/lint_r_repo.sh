#!/bin/bash

REPO_URL="https://github.com/tidyverse/dplyr.git"
REPO_DIR="tmp_dplyr_repo"
LANGUAGE="r"
LINTER_BIN="../../target/release/lintymclintface"

echo "--- Linting R Repository: ${REPO_URL} ---"

# Clone the repository if it doesn't exist
if [ ! -d "${REPO_DIR}" ]; then
  echo "Cloning ${REPO_URL} into ${REPO_DIR}..."
  git clone ${REPO_URL} ${REPO_DIR}
  if [ $? -ne 0 ]; then
    echo "Error: Failed to clone repository."
    exit 1
  fi
else
  echo "Repository ${REPO_DIR} already exists. Skipping clone."
fi

# Find all R files and lint them
find "${REPO_DIR}" -name "*.R" | while read -r file; do
  echo "Linting ${file}...";
  ${LINTER_BIN} -l ${LANGUAGE} -f "${file}"
  if [ $? -ne 0 ]; then
    echo "Warning: Linter returned an error for ${file}"
  fi
done

echo "--- Finished Linting R Repository ---"