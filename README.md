# rust basic

## Mirrors
```
export RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup
export RUSTUP_UPDATE_ROOT=https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup

```

## Install From TsingHua Mirror
```
$ wget 'https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup/archive/1.22.1/x86_64-unknown-linux-gnu/rustup-init' -O 'rustup-init'
$ chmod a+x rustup-init
$ ./rustup-init
```

## Install From Rust Org 
```
$ wget 'https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init'
$ chmod a+x rustup-init
$ ./rustup-init
```

## Mirror for crates in ~/.cargo/config:
```
[source.crates-io]
replace-with = 'tuna'

[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"
``` 