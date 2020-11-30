#! /bin/sh

set -eu

EVENT=2020

. ~/usr/lib/log.sh

if [ $# -lt 1 ]; then
    fatal "Usage: $0 day"
fi

day="$(printf "%d" "${1}")"
name="$(printf "d%02d" "${1}")"

[ "${day}" -lt 1 ] && fatal "Invalid day: ${1}"

[ -d "input" ] || mkdir input

curl --fail --cookie "session=${SESSION}" "https://adventofcode.com/${EVENT}/day/${day}/input" > "input/${name}.txt"
git add "input/${name}.txt"
