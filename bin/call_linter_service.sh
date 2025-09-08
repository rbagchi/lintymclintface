#!/bin/bash

# Script to call the lintymclintface service via API

SERVICE_URL="http://localhost:8080/lint"

# Check for correct number of arguments
if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <language> <file_path>"
    echo "Example: $0 java src/main/java/com/example/Main.java"
    exit 1
fi

LANGUAGE="$1"
FILE_PATH="$2"

# Check if jq is installed
if ! command -v jq &> /dev/null
then
    echo "Error: jq is not installed. Please install it to use this script."
    echo "On macOS: brew install jq"
    echo "On Debian/Ubuntu: sudo apt-get install jq"
    exit 1
fi

# Check if file exists
if [ ! -f "$FILE_PATH" ]; then
    echo "Error: File not found at '$FILE_PATH'"
    exit 1
fi

# Read file content
CODE=$(cat "$FILE_PATH")

# Construct JSON payload using jq for robust escaping
JSON_PAYLOAD=$(jq -n \
    --arg lang "$LANGUAGE" \
    --arg code "$CODE" \
    '{language: $lang, code: $code}')

# Make the API call using curl
echo "Calling lintymclintface service for $LANGUAGE file: $FILE_PATH"
curl -X POST \
     -H "Content-Type: application/json" \
     -d "$JSON_PAYLOAD" \
     "$SERVICE_URL"

echo "\nAPI call complete."