set shell := ['nu', 'justfile.nu']
set export


profile := 'dev'
host-target := `rustc -vV | lines | skip 1 | to text | from csv -s : --noheaders | reduce -f {} {|el, acc| $acc | upsert $el.column0 $el.column1 } | get host | str trim`
artifact-dir := 'addons/boids/lib'

_default: (_just-cmd '-l')
_setup-env:
  mkdir -v {{artifact-dir}}
_just-cmd *FLAGS="":
  @just -f {{justfile()}} {{FLAGS}}

build $target=(host-target) *FLAGS="": _setup-env
  cd rust; cross build {{FLAGS}} --profile {{profile}} --target {{target}}
install $target=(host-target): _setup-env
  mv -f rust/target/{{target}}/{{`$env.profiledir`}}/{{`$env.libprefix`}}boids.{{`$env.ext`}} {{artifact-dir}}/boids{{`$env.arch`}}.{{`$env.ext`}}
build-install target=(host-target) *FLAGS="": (build target FLAGS) (install target)

wasm: (build-install 'wasm32-unknown-emscripten' '+nightly' '-Zbuild-std')
windows: (build-install 'x86_64-pc-windows-msvc')
linux: (build-install 'x86_64-unknown-linux-gnu')
all: (_just-cmd '--timestamp' 'profile=release' 'linux' 'windows' 'wasm')

package:
  rm -rf boids-release.zip builds
  mkdir builds/addons
  touch builds/.gdignore
  cp -rf addons/boids builds/addons/
  cp -f README.md LICENSE.txt builds/addons/boids/
  cp -rf examples README.md LICENSE.txt builds/
  cd builds; run-external 'zip' '-r' '../boids-release.zip' 'addons' 'examples' 'README.md' 'LICENSE.txt'