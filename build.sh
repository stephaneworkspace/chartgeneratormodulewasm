#!/bin/sh
echo "don't forgot to change version in package.json before... or do it later with the same that Cargo.toml"
cp ./pkg/package.json ./tmp.json
cargo update
yarn install
rm -rf dist
rm -rf pkg
#yarn build
wasm-pack build --release --out-name index
mv ./tmp.json ./pkg/package.json
cp ./test.txt ./pkg/test.txt
#--target nodejs
# https://github.com/rustwasm/wasm-pack/issues/199 not fixed
#wasm-pack pack
#wasm-pack publish
#yarn serve
