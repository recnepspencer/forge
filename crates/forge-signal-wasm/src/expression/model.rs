use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SignalValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<SignalValue>),
    Object(Vec<(String, SignalValue)>),
}

impl Default for SignalValue {
    fn default() -> Self {
        Self::Null
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Expr {
    Value { value: SignalValue },
    Read { id: String },
    Get { target: Box<Expr>, field: String },
    At { target: Box<Expr>, index: Box<Expr> },
    First { target: Box<Expr> },
    Last { target: Box<Expr> },
    Slice {
        target: Box<Expr>,
        start: Box<Expr>,
        #[serde(default)]
        end: Option<Box<Expr>>,
    },
    Join { target: Box<Expr>, separator: Box<Expr> },
    Flatten { target: Box<Expr> },
    Object { fields: Vec<(String, Expr)> },
    Array { items: Vec<Expr> },
    Sum { args: Vec<Expr> },
    Multiply { args: Vec<Expr> },
    Concat { args: Vec<Expr> },
    Coalesce { args: Vec<Expr> },
    Length { target: Box<Expr> },
    Contains { target: Box<Expr>, value: Box<Expr> },
    MergeObjects { args: Vec<Expr> },
    Keys { target: Box<Expr> },
    Values { target: Box<Expr> },
    HasField { target: Box<Expr>, field: String },
    Pick { target: Box<Expr>, fields: Vec<String> },
    Omit { target: Box<Expr>, fields: Vec<String> },
    Append { target: Box<Expr>, value: Box<Expr> },
    Abs { target: Box<Expr> },
    Min { args: Vec<Expr> },
    Max { args: Vec<Expr> },
    Sqrt { target: Box<Expr> },
    Sin { target: Box<Expr> },
    Cos { target: Box<Expr> },
    Floor { target: Box<Expr> },
    Mod { left: Box<Expr>, right: Box<Expr> },
    Clamp {
        value: Box<Expr>,
        min: Box<Expr>,
        max: Box<Expr>,
    },
    Atan2 { y: Box<Expr>, x: Box<Expr> },
    Subtract { left: Box<Expr>, right: Box<Expr> },
    Divide { left: Box<Expr>, right: Box<Expr> },
    Eq { left: Box<Expr>, right: Box<Expr> },
    Neq { left: Box<Expr>, right: Box<Expr> },
    Gt { left: Box<Expr>, right: Box<Expr> },
    Gte { left: Box<Expr>, right: Box<Expr> },
    Lt { left: Box<Expr>, right: Box<Expr> },
    Lte { left: Box<Expr>, right: Box<Expr> },
    And { args: Vec<Expr> },
    Or { args: Vec<Expr> },
    Not { arg: Box<Expr> },
    If {
        condition: Box<Expr>,
        then_expr: Box<Expr>,
        else_expr: Box<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConditionSpec {
    pub expr: Expr,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum IdentitySpec {
    Exact,
    Expr { expr: Expr },
}
