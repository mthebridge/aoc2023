#! /bin/bash
set -euo pipefail

if [[ -z "$AOC_SESSION_COOKIE" ]]; then
    echo "Please set variable AOC_SESSION_COOKIE"
    return
fi

declare -i day=$1

if (( day < 10 )); then
    outfile=day0${day}.txt
else
    outfile=day${day}.txt
fi

curl --fail --output "inputs/$outfile" --cookie "session=$AOC_SESSION_COOKIE" "https://adventofcode.com/2023/day/$1/input"
