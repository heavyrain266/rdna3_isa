use serde::Deserialize;


#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct EncodingIdentifier {
	#[serde(rename = "@Radix")]
	pub radix: i8,
	#[serde(rename = "$text")]
	pub text: String,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct EncodingIdentifierMask {
	#[serde(rename = "@Radix")]
	pub radix: i8,
	#[serde(rename = "$text")]
	pub text: String,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct EncodingIdentifiers {
	pub encoding_identifier: Vec<EncodingIdentifier>,
}
