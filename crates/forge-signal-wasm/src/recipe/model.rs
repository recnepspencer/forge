use serde::{Deserialize, Serialize};

use crate::expression::model::{ConditionSpec, Expr, IdentitySpec, SignalValue};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceSpec {
    pub id: String,
    #[serde(default)]
    pub initial: SignalValue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyedSourceFamilySpec {
    pub family_id: String,
    #[serde(default)]
    pub initial: SignalValue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeSpec {
    pub id: String,
    #[serde(default)]
    pub reads: Vec<String>,
    pub expr: Expr,
    #[serde(default)]
    pub when: Option<ConditionSpec>,
    #[serde(default)]
    pub identity: Option<IdentitySpec>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum RecipeFamilyReadSpec {
    Signal { id: String },
    Keyed { family_id: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyedRecipeFamilySpec {
    pub family_id: String,
    #[serde(default)]
    pub reads: Vec<RecipeFamilyReadSpec>,
    pub expr: Expr,
    #[serde(default)]
    pub when: Option<ConditionSpec>,
    #[serde(default)]
    pub identity: Option<IdentitySpec>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum TransactionOp {
    Set { id: String, value: SignalValue },
    SetMany { values: Vec<SetValue> },
    SetManyKeyed {
        #[serde(rename = "familyId")]
        family_id: String,
        values: Vec<KeyedSetValue>,
    },
    SetPackedGridRgba {
        #[serde(rename = "familyId")]
        family_id: String,
        width: u32,
        height: u32,
        rgba: Vec<u8>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetValue {
    pub id: String,
    pub value: SignalValue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyedSetValue {
    pub key: String,
    pub value: SignalValue,
}
