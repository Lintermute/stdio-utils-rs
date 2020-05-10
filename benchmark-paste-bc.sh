#!/bin/sh

set -ex

# Source: https://stackoverflow.com/a/451204
time paste -s -d+ - | bc
