#!/usr/bin/env bash
# Print the body of the `## [<version>]` section of a Keep a Changelog file, with leading and trailing
# blank lines removed. Exits non-zero when the section is missing or empty.
# Usage: changelog_section.sh <file> <version>
set -euo pipefail

if [ "$#" -ne 2 ]; then
  echo "usage: $0 <file> <version>" >&2
  exit 2
fi

awk -v heading="## [$2]" '
  /^## \[/ {
    if (found) { exit }
    if (index($0, heading) == 1) { found = 1; next }
  }
  found && /^\[[^]]+\]: / { exit }
  found { lines[++n] = $0 }
  END {
    first = 1
    while (first <= n && lines[first] ~ /^[[:space:]]*$/) first++
    last = n
    while (last >= first && lines[last] ~ /^[[:space:]]*$/) last--
    if (!found || first > last) { exit 1 }
    for (i = first; i <= last; i++) print lines[i]
  }
' "$1"
