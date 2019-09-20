#!/bin/zsh

set -e

./scripts/build.sh

rm Cargo.lock
cp Cargo.lock.good Cargo.lock
cargo build

cargo run -- purge-chain --dev -y
cargo run -- --dev
