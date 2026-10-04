#!/bin/sh
set -eu

cargo fmt --manifest-path engine/Cargo.toml -- --check
cargo test --locked --manifest-path engine/Cargo.toml
cargo clippy --locked --manifest-path engine/Cargo.toml --all-targets -- -D warnings
julia analysis/test_frequencies.jl
"${CUE_BIN:-cue}" vet -c language/schema.cue language/tables.json
scripts/test-cue.sh
cargo build --release --locked --manifest-path engine/Cargo.toml --target wasm32-unknown-unknown
node --test tests/*.test.mjs
printf '%s\n' 'I passed my pinned local checks.'
