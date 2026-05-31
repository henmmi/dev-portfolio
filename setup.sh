#!/bin/bash
curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh -s -- -y
source $HOME/.cargo/env


rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
cargo install --locked trunk
trunk build --release
