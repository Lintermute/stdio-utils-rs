#!/bin/sh

set -ex

# Source: https://stackoverflow.com/a/450821
time awk '{s+=$1} END {printf "%.0f", s}'
