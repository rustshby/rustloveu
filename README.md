# rust basic

## Config
```
export CARGO_HOME=/home/simon/devenvs/rust/.cargo
export PATH=$PATH:$CARGO_HOME/bin
export RUSTUP_HOME=/home/simon/devenvs/rust/.rustup
export RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup
export RUSTUP_UPDATE_ROOT=https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup
```

## Install From TsingHua Mirror
```
$ wget 'https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup/archive/1.23.1/x86_64-unknown-linux-gnu/rustup-init' -O 'rustup-init'
$ chmod a+x rustup-init
$ ./rustup-init
```

## Install From Rust Org 
```
$ wget 'https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init'
$ chmod a+x rustup-init
$ ./rustup-init
```

## Mirror for crates in $CARGO_HOME/config:
```
[source.crates-io]
replace-with = 'tuna'

[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"
``` 

## Add Components
```
rustup component add rust-analysis --toolchain stable-x86_64-unknown-linux-gnu
rustup component add rust-src --toolchain stable-x86_64-unknown-linux-gnu
rustup component add rls --toolchain stable-x86_64-unknown-linux-gnu
```

# debug
## install extensions for vscode:
+ [codelldb](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)

## launch.json
```
{
    // Use IntelliSense to learn about possible attributes.
    // Hover to view descriptions of existing attributes.
    // For more information, visit: https://go.microsoft.com/fwlink/?linkid=830387
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug executable 'yourbinfilename'",
            "cargo": {
                "args": [
                    "build",
                    "--bin=yourbinfilename",
                    "--package=yourpackagename"
                ],
                "filter": {
                    "name": "yourbinfilename",
                    "kind": "bin"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        }
    ]
}
```
