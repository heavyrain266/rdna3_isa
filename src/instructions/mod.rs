mod encodings;

use serde::Deserialize;


#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InstructionFlags {
	pub is_branch: bool,
	pub is_conditional_branch: bool,
	pub is_indirect_branch: bool,
	pub is_program_terminator: bool,
	pub is_immediately_executed: bool,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Instruction {
	pub instruction_name: String,
	pub description: String,
	pub instruction_flags: Vec<InstructionFlags>,
	pub instruction_encodings: encodings::InstructionEncodings,
	pub functional_group: crate::FunctionalGroup,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Instructions {
	pub instruction: Vec<Instruction>,
}
