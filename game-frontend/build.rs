fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .build_transport(false)
        .compile_protos(&["../proto/common.proto"], &["../proto"])?;
    tonic_prost_build::configure()
        .build_transport(false)
        .compile_protos(&["../proto/game.proto"], &["../proto"])?;
    Ok(())
}
