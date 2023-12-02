#! /bin/bash
set -xeo pipefail

if [[ -z "$AOC_SESSION_COOKIE" ]]; then
    echo "Please set variable AOC_SESSION_COOKIE"
    exit
fi

declare -i day=$1
year="2023"
if [[ -n "$2" ]]; then
    year="$2"
fi

if (( day < 10 )); then
    outfile=day0${day}.txt
else
    outfile=day${day}.txt
fi

curl --fail --output "inputs/$outfile" --cookie "session=$AOC_SESSION_COOKIE" "https://adventofcode.com/$year/day/$1/input"
