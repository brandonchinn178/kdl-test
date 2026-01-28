#!/usr/bin/env bash

set -eu -o pipefail

main() {
    echo "===> Building kdl-rs-decoder"
    cargo build -p kdl-rs-decoder --bin kdl-rs-decoder
    local decoder=target/debug/kdl-rs-decoder

    echo ""
    echo "===> Regenerating files"
    local dir=test_cases/valid
    local failures=()
    for input in "$dir"/*.kdl; do
        local expected; expected="$dir/$(basename "$input" .kdl).json"
        echo "- $input => $expected"
        if ! color $decoder < "$input" > "$expected"; then
            rm "$expected"
            failures+=("$input")
        fi
    done

    if [[ "${#failures[@]}" -gt 0 ]]; then
        echo ""
        echo "===> Failures:"
        for failure in "${failures[@]}"; do
            echo "$failure"
        done
    fi
}

# https://stackoverflow.com/a/16178979
color()(set -o pipefail;"$@" 2>&1>&3|sed $'s,.*,\e[31m&\e[m,'>&2)3>&1

main
