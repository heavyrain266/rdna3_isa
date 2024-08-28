mod cond;
mod ident;
mod microcode;

use serde::Deserialize;


#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct Encoding {
	#[serde(rename = "@Order")]
	order: i8,

	#[serde(rename = "EncodingName")]
	name: String,
	#[serde(rename = "BitCount")]
	bit_count: i8,
	#[serde(rename = "EncodingIdentifierMask")]
	identifier_mask: ident::EncodingIdentifierMask,
	#[serde(rename = "EncodingIdentifiers")]
	identifiers: Vec<ident::EncodingIdentifiers>,
	#[serde(rename = "EncodingConditions")]
	conditions: Vec<cond::EncodingConditions>,

	#[serde(rename = "Description")]
	description: String,
	#[serde(rename = "MicrocodeFormat")]
	microcode_format: microcode::MicrocodeFormat,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Encodings {
	encoding: Vec<Encoding>,
}
