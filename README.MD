# How to use cargo

clean, then build the debug and release targets:
```
cargo clean && cargo build && cargo build --release
```
or call the `compile.sh` script which includes the commands from above.

note that `cargo clean` removes the content of the build folder `<target>`. when running `cargo build` afterwards, all projects included in the `Cargo.toml` get rebuilt. running solely `cargo build` only rebuilds those projects whose source code or dependencies including versions have changed.

get version info about a crate from crates.io:
```
cargo search <crate>
```

add or update the crate dependency in a Cargo.toml
- navigate to Cargo.toml root
```
cargo add <crate>[@<version>]
```
