#!/bin/sh
cargo update
yarn install
rm -rf dist
rm -rf pkg
#yarn build
wasm-pack build --release --out-name index
#--target nodejs
# https://github.com/rustwasm/wasm-pack/issues/199 not fixed
#wasm-pack pack
#wasm-pack publish
#yarn serve
