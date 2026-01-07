fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../proto");
    tonic_prost_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(
            &[
                "../proto/common/v1/common.proto",
                "../proto/game/v1/game.proto",
            ],
            &["../proto"],
        )?;
    Ok(())
}
