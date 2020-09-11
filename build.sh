#!/bin/sh
cargo update
yarn install
rm -rf dist
rm -rf pkg
#yarn build
wasm-pack build
#yarn serve
