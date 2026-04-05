use serde::{Deserialize, Serialize};

use crate::expression::model::{ConditionSpec, Expr, IdentitySpec, SignalValue};
use forge_signal::facade::{ChangedRegion, PartitionMatchMode, PartitionSubscription};

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
    #[serde(rename = "familyId")]
    pub family_id: String,
    #[serde(default)]
    pub initial: SignalValue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeSpec {
    pub id: String,
    #[serde(default)]
    pub reads: Vec<RecipeReadSpec>,
    pub expr: Expr,
    #[serde(default)]
    pub when: Option<ConditionSpec>,
    #[serde(default)]
    pub identity: Option<IdentitySpec>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeReadSignalSpec {
    pub id: String,
    #[serde(default)]
    pub scope: Option<PartitionSubscription>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RecipeReadSpec {
    LegacyId(String),
    Signal(RecipeReadSignalSpec),
}

impl RecipeReadSpec {
    pub fn id(&self) -> &str {
        match self {
            Self::LegacyId(id) => id,
            Self::Signal(spec) => &spec.id,
        }
    }

    pub fn scope(&self) -> Option<&PartitionSubscription> {
        match self {
            Self::LegacyId(_) => None,
            Self::Signal(spec) => spec.scope.as_ref(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeFamilyReadScopeSpec {
    #[serde(default)]
    pub partition: Option<String>,
    #[serde(default)]
    pub partition_from: Option<String>,
    #[serde(default)]
    pub detail: Option<String>,
    #[serde(default)]
    pub match_mode: Option<PartitionMatchMode>,
}

impl RecipeFamilyReadScopeSpec {
    pub fn resolve(&self, key: &str) -> Option<PartitionSubscription> {
        let partition = self
            .partition
            .as_ref()
            .cloned()
            .or_else(|| match self.partition_from.as_deref() {
                Some("key") => Some(key.to_owned()),
                _ => None,
            })?;

        let detail = self.detail.clone();
        let match_mode = self.match_mode.unwrap_or({
            if detail.is_some() {
                PartitionMatchMode::PartitionAndDetail
            } else {
                PartitionMatchMode::WholePartition
            }
        });

        Some(PartitionSubscription {
            partition: partition.into(),
            detail,
            match_mode,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum RecipeFamilyReadSpec {
    Signal {
        id: String,
        #[serde(default)]
        scope: Option<RecipeFamilyReadScopeSpec>,
    },
    Keyed {
        #[serde(rename = "familyId")]
        family_id: String,
        #[serde(default)]
        scope: Option<RecipeFamilyReadScopeSpec>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyedRecipeFamilySpec {
    #[serde(rename = "familyId")]
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
    Set {
        id: String,
        value: SignalValue,
    },
    SetWithRegions {
        id: String,
        value: SignalValue,
        #[serde(rename = "changedRegions", default)]
        changed_regions: Vec<ChangedRegion>,
    },
    SetMany {
        values: Vec<SetValue>,
    },
    SetManyWithRegions {
        values: Vec<SetValueWithRegions>,
    },
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetValueWithRegions {
    pub id: String,
    pub value: SignalValue,
    #[serde(default)]
    pub changed_regions: Vec<ChangedRegion>,
}
