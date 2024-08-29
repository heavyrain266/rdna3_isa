pub mod dataformats;
pub mod encodings;
pub mod instructions;
pub mod operand_types;

use serde::Deserialize;


#[derive(Debug, PartialEq, Deserialize)]
pub struct Spec {
	#[serde(rename = "Document")]
	pub document: Document,
	#[serde(rename = "ISA")]
	pub isa: ISA,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Document {
	pub copyright: String,
	pub sensitivity: String,
	// FIXME: Error: Custom("a character literal was not valid")
	//
	// #[serde(with = "time::serde::rfc3339")]
	// pub release_date: time::OffsetDateTime,
	pub schema_version: String,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FunctionalGroups {
	pub functional_group: Vec<FunctionalGroup>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FunctionalGroup {
	pub name: String,
	pub subgroup: Option<String>,
	pub description: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Architecture {
	pub architecture_name: String,
	pub architecture_id: String,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ISA {
	pub architecture: Architecture,
	pub encodings: encodings::Encodings,
	pub instructions: instructions::Instructions,
	pub data_formats: dataformats::DataFormats,
	pub operand_types: operand_types::OperandTypes,
	pub functional_groups: FunctionalGroups,
}
