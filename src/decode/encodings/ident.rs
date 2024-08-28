use serde::Deserialize;

#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct EncodingIdentifier {
	#[serde(rename = "@Radix")]
	radix: i8,
	#[serde(rename = "$text")]
	text: String,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct EncodingIdentifierMask {
	#[serde(rename = "@Radix")]
	radix: i8,
	#[serde(rename = "$text")]
	text: String,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct EncodingIdentifiers {
	#[serde(rename = "EncodingIdentifier")]
	identifier: Vec<EncodingIdentifier>,
}
