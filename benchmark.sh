#!/bin/sh

set -ex


DATA="random_numbers.txt"

shuf -i 1-100 -n $(( 25 * 1000 * 1000 )) -r > "$DATA"

./benchmark-awk.sh < "$DATA"
./benchmark-paste-bc.sh < "$DATA"
./benchmark-python.sh < "$DATA"
./benchmark-stdio-utils.sh < "$DATA"

rm "$DATA"
