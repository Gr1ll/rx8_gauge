#!/bin/bash

while [ ! -e /dev/fb0 ]; do
  echo "Waiting for /dev/fb0 device..."
  sleep 1
done

echo "/dev/fb0 found"

cd /home/grill/rx8_gauge
./target/release/rx8_gauge
