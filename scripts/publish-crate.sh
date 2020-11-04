#!/bin/bash

set -e
cd "$(dirname "$(readlink -f "$0")")/../yagui"

if [[ "$TOKEN" == "" ]]
then
    echo "ERROR: Set \$TOKEN environment variable first"
    exit 1
fi

cargo login $TOKEN
cargo publish --dry-run --allow-dirty
cargo package --list --allow-dirty

echo
echo "Continue?"
read -n 1 KEY

if [[ "$KEY" != "y" && "$KEY" != "Y" ]]
then
    exit 1
fi

echo
echo "Publishing:"
cargo publish --allow-dirty
