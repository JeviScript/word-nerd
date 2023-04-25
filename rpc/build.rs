fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("./src")
        .compile(&["account.proto", "dictionary.proto"], &["proto/"])?;
    Ok(())
}
