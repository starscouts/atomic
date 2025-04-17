#!/bin/zsh
rm -rf release
mkdir release

cargo clean
cargo build --release --all

cp target/x86_64-pc-windows-gnu/release/launcher.exe release/launcher-windows-x64.exe
cp target/x86_64-unknown-linux-gnu/release/launcher release/launcher-linux-x64
cp target/x86_64-apple-darwin/release/launcher release/launcher-darwin-x64
cp target/aarch64-unknown-linux-musl/release/launcher release/launcher-linux-arm64
cp target/aarch64-apple-darwin/release/launcher release/launcher-darwin-arm64
cp -f release/* ../converter
