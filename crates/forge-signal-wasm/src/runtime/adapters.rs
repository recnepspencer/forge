use serde::{Deserialize, Serialize};

use crate::recipe::model::{KeyedRecipeFamilySpec, KeyedSourceFamilySpec, RecipeSpec, SourceSpec};
use crate::runtime::policy::RuntimePolicySpec;
use crate::runtime::summaries::RuntimeSnapshotEnvelope;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeDefinitionEnvelope {
    pub policy: RuntimePolicySpec,
    pub sources: Vec<SourceSpec>,
    pub recipes: Vec<RecipeSpec>,
    pub source_families: Vec<KeyedSourceFamilySpec>,
    pub recipe_families: Vec<KeyedRecipeFamilySpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeEnvelope {
    pub definitions: RuntimeDefinitionEnvelope,
    pub snapshot: RuntimeSnapshotEnvelope,
}
