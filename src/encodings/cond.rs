use serde::Deserialize;


#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SubExpressions {
	expression: Vec<Expression>,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct ValueType {
	#[serde(rename = "@Name")]
	name: String,

	base_type: char,
	size: i8,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Expression {
	#[serde(rename = "@Type")]
	ty: Option<String>,

	operator: Option<String>,
	subexpressions: Option<SubExpressions>,
	label: Option<String>,
	value: Option<i32>,
	value_type: Option<ValueType>,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct ConditionExpression {
	expression: Expression,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct EncodingCondition {
	pub condition_name: String,
	pub condtion_expression: ConditionExpression,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct EncodingConditions {
	pub encoding_condition: Vec<EncodingCondition>,
}
