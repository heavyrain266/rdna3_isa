use serde::Deserialize;


#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SubExpressions {
	pub expression: Vec<Expression>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ValueType {
	#[serde(rename = "@Name")]
	pub name: String,

	pub base_type: char,
	pub size: i8,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Expression {
	#[serde(rename = "@Type")]
	pub ty: Option<String>,

	pub operator: Option<String>,
	pub subexpressions: Option<SubExpressions>,
	pub label: Option<String>,
	pub value: Option<i32>,
	pub value_type: Option<ValueType>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConditionExpression {
	pub expression: Expression,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EncodingCondition {
	pub condition_name: String,
	pub condtion_expression: ConditionExpression,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EncodingConditions {
	pub encoding_condition: Vec<EncodingCondition>,
}
