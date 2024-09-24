#!/bin/sh

cross build --release --target x86_64-unknown-linux-gnu
cross build --release --target x86_64-pc-windows-gnu
cross +nightly build -Zbuild-std --release --target wasm32-unknown-emscripten

cd ..

cp -f rust/target/x86_64-unknown-linux-gnu/release/libboids.so addons/boids/lib/boids.x86.so
cp -f rust/target/x86_64-pc-windows-gnu/release/boids.dll addons/boids/lib/boids.x86.dll
cp -f rust/target/wasm32-unknown-emscripten/release/boids.wasm addons/boids/lib/boids.wasm

zip -r ../boids-release.zip addons examples README.md LICENSE.txt
