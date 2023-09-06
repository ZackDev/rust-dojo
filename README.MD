# How to use cargo

clean, then build the debug and release targets:
```
cargo clean && cargo build && cargo build --release
```
or call the `compile.sh` script which includes the commands from above. 

get version info about a crate from crates.io:
```
cargo info <crate>
```

add or update the crate dependency in a Cargo.toml
- navigate to Cargo.toml root
```
cargo add <crate>[@<version>]
```
