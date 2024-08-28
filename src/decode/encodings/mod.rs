pub mod cond;
pub mod ident;
pub mod microcode;

use serde::Deserialize;


#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct Encoding {
	#[serde(rename = "@Order")]
	pub order: i8,

	#[serde(rename = "EncodingName")]
	pub name: String,
	#[serde(rename = "BitCount")]
	pub bit_count: i8,
	#[serde(rename = "EncodingIdentifierMask")]
	pub identifier_mask: ident::EncodingIdentifierMask,
	#[serde(rename = "EncodingIdentifiers")]
	pub identifiers: Vec<ident::EncodingIdentifiers>,
	#[serde(rename = "EncodingConditions")]
	pub conditions: Vec<cond::EncodingConditions>,

	#[serde(rename = "Description")]
	pub description: String,
	#[serde(rename = "MicrocodeFormat")]
	pub microcode_format: microcode::MicrocodeFormat,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Encodings {
	pub encoding: Vec<Encoding>,
}
