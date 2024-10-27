wasm-pack build --target web --out-name index --release --out-dir ./pkg-web
rm -rf ./pkg-web/.gitignore
rm -rf ./pkg-web/package.json
rm -rf ./pkg-web/README.md
wasm-pack build --target nodejs --out-name index --release --out-dir ./pkg-node
rm -rf ./pkg-node/.gitignore
rm -rf ./pkg-node/package.json
rm -rf ./pkg-node/README.md