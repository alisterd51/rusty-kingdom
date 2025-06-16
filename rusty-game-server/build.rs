fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../proto/common.proto")?;
    tonic_build::compile_protos("../proto/crud.proto")?;
    tonic_build::compile_protos("../proto/game.proto")?;
    Ok(())
}
