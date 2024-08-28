mod decode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let path: String = std::fs::read_to_string(
		format!("{}/spec/amdgpu_isa_rdna3.xml", env!("CARGO_MANIFEST_DIR")
	))?;

	let spec: decode::Spec = quick_xml::de::from_str(path.as_str())?;

	println!("{:#?}", spec);

	return Ok(());
}
