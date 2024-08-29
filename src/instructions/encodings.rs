use serde::Deserialize;


#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Operand {
	#[serde(rename = "@Input")]
	pub input: bool,
	#[serde(rename = "@Output")]
	pub output: bool,
	#[serde(rename = "@IsImplicit")]
	pub is_implicit: bool,
	#[serde(rename = "@IsBinaryMicrocodeRequired")]
	pub is_binary_microcode_required: bool,
	#[serde(rename = "@Order")]
	pub order: i8,

	pub field_name: Option<String>,
	pub data_format_name: String,
	pub operand_type: String,
	pub operand_size: i16,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Operands {
	pub operand: Option<Vec<Operand>>,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct Opcode {
	#[serde(rename = "@Radix")]
	pub bradix: i8,
	#[serde(rename = "$text")]
	pub btext: i16,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InstructionEncoding {
	pub encoding_name: String,
	pub encoding_condition: String,
	pub opcode: Opcode,
	pub operands: Operands,
}


#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InstructionEncodings {
	pub instruction_encoding: Vec<InstructionEncoding>,
}
