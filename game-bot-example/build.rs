fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure().compile_protos(&["../proto/common.proto"], &["../proto"])?;
    tonic_prost_build::configure().compile_protos(&["../proto/game.proto"], &["../proto"])?;
    Ok(())
}
