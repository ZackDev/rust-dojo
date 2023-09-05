# How to use cargo

build the debug and release target:
```
cargo build && cargo build --release
```

get version info about a crate from crates.io:
```
cargo info <crate>
```

add or update the crate dependency in a Cargo.toml
- navigate to Cargo.toml root
```
cargo add <crate>[@<version>]
```
