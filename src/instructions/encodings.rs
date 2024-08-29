use serde::Deserialize;


#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Operand {
	#[serde(rename = "@Input")]
	input: bool,
	#[serde(rename = "@Output")]
	output: bool,
	#[serde(rename = "@IsImplicit")]
	is_implicit: bool,
	#[serde(rename = "@IsBinaryMicrocodeRequired")]
	is_binary_microcode_required: bool,
	#[serde(rename = "@Order")]
	order: i8,

	field_name: Option<String>,
	data_format_name: String,
	operand_type: String,
	operand_size: i16,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Operands {
	operand: Option<Vec<Operand>>,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct Opcode {
	#[serde(rename = "@Radix")]
	radix: i8,
	#[serde(rename = "$text")]
	text: i16,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct InstructionEncoding {
	encoding_name: String,
	encoding_condition: String,
	opcode: Opcode,
	operands: Operands,
}


#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct InstructionEncodings {
	instruction_encoding: Vec<InstructionEncoding>,
}
