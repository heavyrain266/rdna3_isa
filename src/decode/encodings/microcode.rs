use serde::Deserialize;


#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct Range {
	#[serde(rename = "@Order")]
	pub order: i8,
	#[serde(rename = "BitCount")]
	pub bit_count: i8,
	#[serde(rename = "BitOffset")]
	pub bit_offset: i8,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct BitLayout {
	#[serde(rename = "@RangeCount")]
	pub range_count: i8,
	#[serde(rename = "Range")]
	pub range: Range,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct Field {
	#[serde(rename = "@IsConditional")]
	pub is_conditional: bool,

	#[serde(rename = "FieldName")]
	pub field_name: String,
	#[serde(rename = "Description")]
	pub description: String,
	#[serde(rename = "BitLayout")]
	pub bit_layout: BitLayout,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BitMap {
	pub field: Vec<Field>,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MicrocodeFormat {
	pub bit_map: BitMap,
}
