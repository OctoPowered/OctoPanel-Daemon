fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/sysstats.proto")?;
    tonic_build::compile_protos("proto/docker.proto")?;
    Ok(())
}
