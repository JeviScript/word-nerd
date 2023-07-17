fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("./src")
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile(&["account.proto", "dictionary.proto"], &["proto/"])?;
    Ok(())
}
