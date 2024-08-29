pub mod cond;
pub mod ident;
pub mod microcode;

use serde::Deserialize;


#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Encoding {
	#[serde(rename = "@Order")]
	pub order: i8,


	pub encoding_name: String,
	pub bit_count: i8,
	pub encoding_identifier_mask: ident::EncodingIdentifierMask,
	pub encoding_identifiers: Vec<ident::EncodingIdentifiers>,
	pub encoding_conditions: Vec<cond::EncodingConditions>,

	pub description: String,
	pub microcode_format: microcode::MicrocodeFormat,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Encodings {
	pub encoding: Vec<Encoding>,
}
