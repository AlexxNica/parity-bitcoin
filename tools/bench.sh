#!/bin/bash
echo -en 'travis_fold:start:bench'
echo 'Running bench from /benhcer'
cargo run --manifest-path ./bencher/Cargo.toml --release
echo -en 'travis_fold:end:bench'
