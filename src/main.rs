fn main() -> Result<(), Box<dyn std::error::Error>> {
    let spec: String = std::fs::read_to_string("./spec/amdgpu_isa_rdna3.xml")?;

    println!("{:?}", spec);

    return Ok(());
}
