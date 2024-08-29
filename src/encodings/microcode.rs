use serde::Deserialize;


#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Range {
	#[serde(rename = "@Order")]
	pub order: i8,
	pub bit_count: i8,
	pub bit_offset: i8,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BitLayout {
	#[serde(rename = "@RangeCount")]
	pub range_count: i8,
	pub range: Range,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Field {
	#[serde(rename = "@IsConditional")]
	pub is_conditional: bool,

	pub field_name: String,
	pub description: String,
	pub bit_layout: BitLayout,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BitMap {
	pub field: Vec<Field>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MicrocodeFormat {
	pub bit_map: BitMap,
}
