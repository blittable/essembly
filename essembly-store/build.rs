fn main() -> Result<(), Box<dyn std::error::Error>> {

    // tonic_build::configure()
    //     .build_client(true)
    //     .compile(&["proto/susu/google/protobuf/timestamp.proto"], &["proto/susu/google/protobuf"])?;

    // tonic_build::configure()
    //     .build_server(false)
    //     .compile(&["proto/google/.proto"], &["proto/susu"])?;

    // tonic_build::configure()
    //     .build_server(false)
    //     .compile(&["proto/google/protobuf/timestamp.proto"], &["proto", "proto/google/protobuf"])?;

    // tonic_build::configure()
    //     .build_server(false)
    //     .compile(&["proto/google/types/latlng.proto"], &["proto", "proto/google/types"])?;

    tonic_build::configure()
        .build_server(true)
        .compile(&["proto/api.proto"], &["proto", "proto/google/protobuf", "proto/google/types"])?;

    Ok(())
}
