#!/bin/sh

cross build --release --target x86_64-unknown-linux-gnu
cross build --release --target x86_64-pc-windows-gnu
cross +nightly build -Zbuild-std --release --target wasm32-unknown-emscripten

cp -f target/x86_64-unknown-linux-gnu/release/libboids.so ../addons/boids/lib/boids.x86.so
cp -f target/x86_64-pc-windows-gnu/release/boids.dll ../addons/boids/lib/boids.x86.dll
cp -f target/wasm32-unknown-emscripten/release/boids.wasm ../addons/boids/lib/boids.wasm
