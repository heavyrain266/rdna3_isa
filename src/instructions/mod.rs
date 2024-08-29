mod encodings;

use serde::Deserialize;


#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct FunctionalGroup {
	name: String,
	subgroup: String,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct InstructionFlags {
	is_branch: bool,
	is_conditional_branch: bool,
	is_indirect_branch: bool,
	is_program_terminator: bool,
	is_immediately_executed: bool,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Instruction {
	instruction_name: String,
	description: String,
	instruction_flags: Vec<InstructionFlags>,
	instruction_encodings: encodings::InstructionEncodings,
	functional_group: FunctionalGroup
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Instructions {
	instruction: Vec<Instruction>,
}
