use serde::Deserialize;


#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct Range {
	#[serde(rename = "@Order")]
	order: i8,
	#[serde(rename = "BitCount")]
	bit_count: i8,
	#[serde(rename = "BitOffset")]
	bit_offset: i8,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct BitLayout {
	#[serde(rename = "@RangeCount")]
	range_count: i8,
	#[serde(rename = "Range")]
	range: Range,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct Field {
	#[serde(rename = "@IsConditional")]
	is_conditional: bool,

	#[serde(rename = "FieldName")]
	field_name: String,
	#[serde(rename = "Description")]
	description: String,
	#[serde(rename = "BitLayout")]
	bit_layout: BitLayout,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BitMap {
	field: Vec<Field>,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MicrocodeFormat {
	bit_map: BitMap,
}
