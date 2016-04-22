#!/bin/bash
echo '' > results.csv
START=$(git rev-parse --abbrev-ref HEAD)
for COMMIT in $(git rev-list cb3986d85b5a7d5ab7b2230b4928395e142ca735.. --author=nichols --reverse)
do
    git checkout $COMMIT
    make zopfli
    time=$( TIMEFORMAT="%R"; { time ./test/run.sh; } 2>&1 )
    git checkout test/results
    echo "$COMMIT,$time" >> results.csv

done
git checkout $START
