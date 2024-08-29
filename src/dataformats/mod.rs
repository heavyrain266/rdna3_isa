use serde::Deserialize;

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Range {
	#[serde(rename = "@Order")]
	order: i8,

	bit_count: i8,
	bit_offset: i8,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BitLayout {
	#[serde(rename = "@RangeCount")]
	range_count: i8,
	range: Range,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Field {
	#[serde(rename = "@Signedness")]
	signedness: String,

	field_name: String,
	bit_layout: BitLayout,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BitMap {
	#[serde(rename = "@Order")]
	order: Option<i8>,

	field: Option<Field>,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DataFormat {
	pub data_format_name: String,
	pub description: String,
	pub data_type: String,
	pub bit_count: i16,
	pub component_count: i8,
	pub data_attributes: BitMap,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DataFormats {
	pub data_format: Vec<DataFormat>
}
