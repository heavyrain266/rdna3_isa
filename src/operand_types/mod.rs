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
pub struct PredefinedValue {
	pub name: String,
	pub description: String,
	pub value: i16,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FieldPredefinedValues {
	pub predefined_value: Vec<PredefinedValue>,
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

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OperandPredefinedValues {
	predefined_value: Vec<PredefinedValue>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OperandType {
	#[serde(rename = "@IsPartitioned")]
	pub is_partitioned: bool,

	pub operand_type_name: String,
	pub description: String,
	pub operand_predefined_values: Option<OperandPredefinedValues>,
	pub microcode_format: Option<MicrocodeFormat>
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OperandTypes {
	pub operand_type: Vec<OperandType>,
}
