pub mod dataformats;
pub mod encodings;
pub mod instructions;

use serde::Deserialize;


#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct Spec {
	#[serde(rename = "Document")]
	pub document: Document,
	#[serde(rename = "ISA")]
	pub isa: ISA,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Document {
	pub copyright: String,
	pub sensitivity: String,
	// FIXME: figure out how to make it parse date... (returns None)
	// pub release_data: Option<time::Date>,
	pub schema_version: String,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct FunctionalGroups {
	pub functional_group: Vec<FunctionalGroup>,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct FunctionalGroup {
	pub name: String,
	pub subgroup: Option<String>,
	pub description: Option<String>,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Architecture {
	pub architecture_name: String,
	pub architecture_id: String,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct ISA {
	pub architecture: Architecture,
	pub encodings: encodings::Encodings,
	pub instructions: instructions::Instructions,
	pub data_formats: dataformats::DataFormats,
	pub functional_groups: FunctionalGroups,
}
