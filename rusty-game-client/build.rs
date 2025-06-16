fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(&["../proto/common.proto"], &["../proto"])?;
    // tonic_build::compile_protos("../proto/common.proto")?;
    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(&["../proto/game.proto"], &["../proto"])?;
    // tonic_build::compile_protos("../proto/game.proto")?;
    Ok(())
}
