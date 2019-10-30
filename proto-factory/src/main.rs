use std::env;
use std::ffi::OsStr;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let out_dir = get_output_dir();

    tonic_build::configure()
        .build_server(true)
        .out_dir(out_dir)
        .compile(
            &["proto/api.proto"],
            &["proto", "proto/google/protobuf", "proto/google/types"],
        )?;
    Ok(())
}

pub fn get_output_dir() -> PathBuf {
    env::var_os("PROTO_OUTPUT_DIR")
        .unwrap_or_else(|| OsStr::new("../generated_protos").to_os_string())
        .into()
}
