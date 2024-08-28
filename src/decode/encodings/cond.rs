use serde::Deserialize;


#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct SubExpressions {
	expression: Option<Vec<Expression>>,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct ValueType {
	#[serde(rename = "@Name")]
	name: String,

	#[serde(rename = "BaseType")]
	base_type: char,
	#[serde(rename = "Size")]
	size: i8,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Expression {
	#[serde(rename = "@Type")]
	ty: Option<String>,

	operator: Option<String>,
	subexpressions: Option<SubExpressions>,
	label: Option<String>,
	value: Option<i8>,
	value_type: Option<ValueType>,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConditionExpression {
	expression: Expression,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct EncodingCondition {
	#[serde(rename = "ConditionName")]
	pub name: String,

	#[serde(rename = "CondtionExpression")]
	pub expression: ConditionExpression,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct EncodingConditions {
	#[serde(rename = "EncodingCondition")]
	pub condition: Vec<EncodingCondition>,
}
