#!/usr/bin/env bash

# README:
#
#   Each folder under `src-data` represents a single standard, which may
#   generate one or more data sets. Each directory will contain a Python
#   script, `generate.py` which is called by the top-level script to create
#   the JSON in the correct location. Each should also contain a README
#   that includes attribution for any data retrieved to make this possible.

if [[ "$1" = "" ]] ; then
  echo "Error: needs to specify output location as argument"
  exit 1
elif [[ ! -d $1 ]] ; then
    echo "Error: $1 needs to an existing directory"
fi

pushd "$1" >/dev/null
OUTPUT_DIR="$(pwd)/codes/data"
popd >/dev/null

echo "data file output location is $OUTPUT_DIR"

cd src-data
for f in *
do
    if [[ -d "$f" ]] ; then
        pushd "$f" >/dev/null
        if [[ -e "generate.sh" ]] ; then
          echo "generating $(basename $(pwd)) files into $OUTPUT_DIR"
          python ./generate.py $OUTPUT_DIR
        fi
        popd >/dev/null
    fi
done