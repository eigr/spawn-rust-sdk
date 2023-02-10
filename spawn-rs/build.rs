fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .out_dir("src/eigr/spawn")
        .compile(
            &[
                "proto/google/protobuf/any.proto",
                "proto/actor.proto",
                "proto/protocol.proto",
            ],
            &["proto"],
        )?;
    Ok(())
}
