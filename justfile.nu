let exts = {
    wasm32-unknown-emscripten: "wasm"
    x86_64-pc-windows-msvc: "dll"
    x86_64-unknown-linux-gnu: "so"
}

def getext [] { $exts | get $env.target }
def getlibprefix [] { if ($env.target | str contains "linux") { "lib" } else { "" } }
def getarch [] {
    let arch = $env.target | split row - | first
    if ($arch == "wasm32") {
        ""
    } else {
        "." + $arch
    }
}
def getprofiledir [] { if ($env.profile == 'dev') { "debug" } else { $env.profile }}

def main [command: string] {
    if ('target' in $env) {
        $env.ext = getext
        $env.libprefix = getlibprefix
        $env.arch = getarch
    }
    if ('profile' in $env) {
        $env.profiledir = getprofiledir
    }
    nu -c $command
}