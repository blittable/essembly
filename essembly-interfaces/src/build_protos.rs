use std::env;
use std::ffi::OsStr;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = get_output_dir();

    // tonic_build::configure()
    //     .build_server(false)
    //     .out_dir(&out_dir)
    //     .compile(
    //         &["proto/registration/registration.proto"],
    //         &["proto/registration"],
    //     )?;

    tonic_build::configure()
        .build_server(true)
        .out_dir(&out_dir)
        .compile(
            &[
                "proto/api/api.proto",
                "proto/registration/registration.proto",
            ],
            &["proto"],
        )?;

    Ok(())
}

pub fn get_output_dir() -> PathBuf {
    OsStr::new("src").to_os_string().into()
}
