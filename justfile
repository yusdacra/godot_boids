set shell := ['nu', 'justfile.nu']
set export


profile := 'dev'
host-target := `rustc -vV | lines | skip 1 | to text | from csv -s : --noheaders | reduce -f {} {|el, acc| $acc | upsert $el.column0 $el.column1 } | get host | str trim`
artifact-dir := 'addons/boids/lib'

[private]
default: (just-cmd '-l')

[private]
setup-env:
  mkdir -v {{artifact-dir}}

[private]
just-cmd *FLAGS="":
  @just -f {{justfile()}} {{FLAGS}}


build $target=(host-target) *FLAGS="": setup-env
  cd rust; cross build {{FLAGS}} --profile {{profile}} --target {{target}}

install $target=(host-target): setup-env
  mv -f rust/target/{{target}}/{{`$env.profiledir`}}/{{`$env.libprefix`}}boids.{{`$env.ext`}} {{artifact-dir}}/boids.{{`$env.arch`}}.{{`$env.ext`}}

build-install target *FLAGS="": (build target FLAGS) (install target)

wasm: (build-install 'wasm32-unknown-emscripten' '+nightly' '-Zbuild-std')
windows: (build-install 'x86_64-pc-windows-msvc')
linux: (build-install 'x86_64-unknown-linux-gnu')

all: (just-cmd '--timestamp' 'profile=release' 'linux' 'windows' 'wasm')


package:
  run-external 'zip' '-r' 'boids-release.zip' 'addons' 'examples' 'README.md' 'LICENSE.txt'