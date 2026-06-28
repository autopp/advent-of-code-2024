#!/usr/bin/env bash
set -euo pipefail

# Usage: ./add_problem.sh <DD_PP>
#   e.g. ./add_problem.sh 02_01
#
# Creates src/aocDD_PP.rs with a stub `fnDD_PP` and wires it into src/main.rs
# (the `mod` declaration and the match arm).

if [[ $# -ne 1 ]]; then
    echo "Usage: $0 <DD_PP>  (e.g. $0 02_01)" >&2
    exit 1
fi

id="$1"
if [[ ! "$id" =~ ^[0-9]{2}_[0-9]{2}$ ]]; then
    echo "Error: argument must look like 'DD_PP' (two digits, underscore, two digits), got: $id" >&2
    exit 1
fi

root="$(cd "$(dirname "$0")" && pwd)"
src_file="$root/src/aoc${id}.rs"
main_file="$root/src/main.rs"
func="fn${id}"
module="aoc${id}"

if [[ -e "$src_file" ]]; then
    echo "Error: $src_file already exists" >&2
    exit 1
fi

if grep -q "^mod ${module};" "$main_file"; then
    echo "Error: module ${module} is already declared in $main_file" >&2
    exit 1
fi

# 1. Create the problem source file with a stub.
cat > "$src_file" <<EOF
use std::io::stdin;

pub fn ${func}() {
    stdin().lines().for_each(|line| {
        let _line = line.unwrap();
        // TODO: implement
    });
}
EOF

# 2. Patch src/main.rs:
#    - insert `mod aocDD_PP;` after the last existing `mod aoc...;` line
#    - insert the match arm just before the `_ =>` catch-all
tmp="$(mktemp)"
awk -v modname="$module" -v target="$id" -v fname="$func" '
    /^mod aoc[0-9_]+;/ { mod_lines[++mod_count] = NR }
    { lines[NR] = $0 }
    END {
        last_mod = mod_lines[mod_count]
        for (i = 1; i <= NR; i++) {
            if (i == last_mod) {
                print lines[i]
                print "mod " modname ";"
                continue
            }
            if (lines[i] ~ /^[[:space:]]*_ => \{/) {
                print "        \"" target "\" => {"
                print "            " modname "::" fname "();"
                print "        }"
            }
            print lines[i]
        }
    }
' "$main_file" > "$tmp"
mv "$tmp" "$main_file"

echo "Created $src_file"
echo "Updated $main_file (added mod ${module}; and target \"${id}\")"
