pub(crate) fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .out_dir("src/domain")
        .compile(&["proto/domain.proto"], &["proto"])?;
    Ok(())
}
