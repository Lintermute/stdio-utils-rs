#!/bin/sh

set -ex

cargo build --release
time cargo run --release
