#!/bin/sh

set -x

cargo build

cd ..

cp -f rust/target/debug/libboids.so addons/boids/lib/boids.x86.so
cp -f rust/target/debug/boids.dll addons/boids/lib/boids.x86.dll
