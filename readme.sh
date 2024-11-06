#!/bin/bash

# Check if SUMMARY.md exists
if [[ ! -f ./src/SUMMARY.md ]]; then
  echo "File SUMMARY.md not found!"
  exit 1
fi

# Copy SUMMARY.md to README.md with modified paths
sed 's|\./chapter_|./src/chapter_|g' ./src/SUMMARY.md > README.md

echo "README.md created with modified paths."
