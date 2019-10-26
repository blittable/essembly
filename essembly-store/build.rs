fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().build_server(true).compile(
        &["proto/api.proto"],
        &["proto", "proto/google/protobuf", "proto/google/types"],
    )?;

    Ok(())
}
