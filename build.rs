use tonic_prost_build::configure;

fn main() -> Result<(), std::io::Error> {
    const PROTOC_ENVAR: &str = "PROTOC";
    if std::env::var(PROTOC_ENVAR).is_err() {
        #[cfg(not(windows))]
        // SAFETY: build script is single-threaded at this point.
        unsafe {
            std::env::set_var(PROTOC_ENVAR, protobuf_src::protoc());
        }
    }

    // Build scripts receive the crate's enabled features as `CARGO_FEATURE_<NAME>`
    // env vars (uppercased, `-` -> `_`), not as `cfg!(feature = ...)`.
    let build_client = std::env::var_os("CARGO_FEATURE_CLIENT").is_some();
    let build_server = std::env::var_os("CARGO_FEATURE_SERVER").is_some();
    let use_bytes = std::env::var_os("CARGO_FEATURE_BYTES").is_some();

    let proto_base_path = std::path::PathBuf::from("protos");
    let proto_files = [
        "auth.proto",
        "block.proto",
        "block_engine.proto",
        "bundle.proto",
        "packet.proto",
        "relayer.proto",
        "searcher.proto",
        "shared.proto",
        "shredstream.proto",
    ];

    let protos: Vec<_> = proto_files
        .iter()
        .map(|proto_file| {
            let proto = proto_base_path.join(proto_file);
            println!("cargo:rerun-if-changed={}", proto.display());
            proto
        })
        .collect();

    let mut builder = configure()
        .build_client(build_client)
        .build_server(build_server);

    if use_bytes {
        // Generate proto `bytes` fields as `bytes::Bytes` instead of `Vec<u8>`.
        builder = builder.bytes(".");
    }

    builder.compile_protos(&protos, &[proto_base_path])?;

    Ok(())
}
