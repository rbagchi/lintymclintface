#!/bin/bash

# Get the directory where the script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

# Determine the project root (assuming the script is in examples/ relative to root)
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Path to the linter binary relative to the project root
LINTER_BIN_RELATIVE="target/release/lintymclintface"

# Full path to the linter binary
LINTER_BIN="${PROJECT_ROOT}/${LINTER_BIN_RELATIVE}"

REPO_URL="https://github.com/psf/requests.git"
REPO_DIR="${PROJECT_ROOT}/tmp_requests_repo"
LANGUAGE="python"

echo "--- Linting Python Repository: ${REPO_URL} ---"

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

# Find all Python files and lint them
find "${REPO_DIR}" -name "*.py" | while read -r file; do
  echo "Linting ${file}...";
  "${LINTER_BIN}" -l ${LANGUAGE} -f "${file}"
  if [ $? -ne 0 ]; then
    echo "Warning: Linter returned an error for ${file}"
  fi
done

echo "--- Finished Linting Python Repository ---"
