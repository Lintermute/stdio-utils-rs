#!/bin/sh

set -ex

# Source: https://stackoverflow.com/questions/450799/shell-command-to-sum-integers-one-per-line#comment273681_450853
time python -c"import sys; print(sum(map(int, sys.stdin)))"
