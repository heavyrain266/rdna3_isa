mod encodings;

use serde::Deserialize;

#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct Spec {
	#[serde(rename = "Document")]
	document: Document,
	#[serde(rename = "ISA")]
	isa: ISA,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Document {
	copyright: String,
	sensitivity: String,
	// FIXME: figure out how to make it parse date... (returns None)
	// release_data: Option<time::Date>,
	schema_version: String,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct ISA {
	architecture: Architecture,
	encodings: encodings::Encodings,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct Architecture {
	#[serde(rename = "ArchitectureName")]
	name: String,
	#[serde(rename = "ArchitectureId")]
	id: String,
}
