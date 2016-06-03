#!/bin/bash
./target/release/zopfli test/data/*
mv test/data/*.gz test/results/