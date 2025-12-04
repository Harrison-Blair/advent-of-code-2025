#!/bin/bash

# Find the highest day number
latest=$(ls -d day-* 2>/dev/null | grep -oP 'day-\K\d+' | sort -n | tail -1)

# If no directories found, start at 1, otherwise increment
if [[ -z "$latest" ]]; then
    next=1
else
    next=$((latest + 1))
fi

# Create the new directory
new_dir="day-$next"
mkdir "$new_dir"

# Create empty text files
touch "$new_dir/example.txt" "$new_dir/input.txt"

# Create empty markdown files
touch "$new_dir/PART-1.md" "$new_dir/PART-2.md" "$new_dir/NOTES.md"

echo "Created $new_dir with template files"