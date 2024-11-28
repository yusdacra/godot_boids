set shell := ['nu', '-c']


profile := 'dev'
host-target := `rustc -vV | lines | skip 1 | to text | from csv -s : --noheaders | reduce -f {} {|el, acc| $acc | upsert $el.column0 $el.column1 } | get host`
artifact-dir := 'addons/boids/lib'

[private]
default: (just-cmd '-l')

[private]
setup-env:
  mkdir -v {{artifact-dir}}

[private]
just-cmd *FLAGS="":
  @just -f {{justfile()}} {{FLAGS}}


build ext $target=(host-target) *FLAGS="": setup-env
  cd rust; cross build {{FLAGS}} --profile {{profile}} --target {{target}}
  mv -f rust/target/{{target}}/{{ if profile == 'dev' { 'debug' } else { profile } }}/{{ if target =~ 'linux' { 'lib' } else { '' } }}boids.{{ext}} {{artifact-dir}}/boids.{{`$env.target | split row - | first`}}.{{ext}}

build-wasm: (build 'wasm' 'wasm32-unknown-emscripten' '+nightly' '-Zbuild-std')
build-windows: (build 'dll' 'x86_64-pc-windows-msvc')
build-linux: (build 'so' 'x86_64-unknown-linux-gnu')

build-all: (just-cmd '--timestamp' 'profile=release' 'build-linux' 'build-windows' 'build-wasm')


package:
  run-external 'zip' '-r' 'boids-release.zip' 'addons' 'examples' 'README.md' 'LICENSE.txt'