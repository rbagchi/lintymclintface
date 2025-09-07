#!/bin/bash

REPO_URL="https://github.com/brettwooldridge/HikariCP.git"
REPO_DIR="tmp_hikari_repo"
LANGUAGE="java"
LINTER_BIN="../../target/release/lintymclintface"

echo "--- Linting Java Repository: ${REPO_URL} ---"

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

# Find all Java files and lint them
find "${REPO_DIR}" -name "*.java" | while read -r file; do
  echo "Linting ${file}...";
  ${LINTER_BIN} -l ${LANGUAGE} -f "${file}"
  if [ $? -ne 0 ]; then
    echo "Warning: Linter returned an error for ${file}"
  fi
done

echo "--- Finished Linting Java Repository ---"