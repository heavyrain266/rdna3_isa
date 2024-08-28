pub mod encodings;

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
pub struct ISA {
	pub architecture: Architecture,
	pub encodings: encodings::Encodings,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct Architecture {
	#[serde(rename = "ArchitectureName")]
	pub name: String,
	#[serde(rename = "ArchitectureId")]
	pub id: String,
}
