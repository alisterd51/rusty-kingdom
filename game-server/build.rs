fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../proto");
    tonic_prost_build::configure().compile_protos(
        &[
            "../proto/common/v1/common.proto",
            "../proto/crud/v1/crud.proto",
            "../proto/game/v1/game.proto",
        ],
        &["../proto"],
    )?;
    Ok(())
}
