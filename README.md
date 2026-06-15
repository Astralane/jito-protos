# Schemas

This repository contains schemas for [grpc](#grpc) and [json rpc](#json-rpc) endpoints for Jito Lab's MEV system.

## grpc

The below explains how to use the public protobuf definitions for Jito Lab's MEV system.

### Usage

Add this repo as a git submodule to your repo. Here's an example file tree in a Rust codebase:

```
your-rust-repo/
├─ src/
│  ├─ gm/
│  │  ├─ lib.rs
│  ├─ jito-protos/
│  │  ├─ protos/
│  │  │  ├─ *.proto
|  |  |─ src/
|  |  |  |─ lib.rs
|  |  |─ build.rs
```

```rust
/// lib.rs

pub mod proto_package {
    tonic::include_proto!("proto_package.proto");
}
```

```rust
/// build.rs

use tonic_build::configure;

fn main() {
    configure()
        .compile(
            &[
                "protos/proto_package.proto",
            ],
            &["protos"],
        )
        .unwrap();
}

```

If you are looking for inspiration, a sample client can be found at [searcher examples](https://github.com/jito-labs/searcher-examples)

## json rpc

[json rpc schema](json_rpc/http.md) explains how to use json rpc for Jito Lab's MEV system.

## Attribution

The protobuf definitions in this repository are derived from [Jito Labs'](https://github.com/jito-labs)
MEV system schemas. This is a fork maintained by [Astralane](https://github.com/Astralane)
that packages those definitions as a Rust crate and tracks the current
tonic / prost toolchain.

All credit for the original schemas belongs to Jito Labs.

## License

Licensed under the [Apache License, Version 2.0](LICENSE).

See [NOTICE](NOTICE) for attribution of the original work and modifications.
