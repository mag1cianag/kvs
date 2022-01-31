#!/bin/usr/env bash

for _ in {1..100}
do 
  echo "set key val"
  ./target/debug/kvs set 1 2
done
