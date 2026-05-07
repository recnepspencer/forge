use serde::{Deserialize, Serialize};

use crate::recipe::model::{RecipeSpec, SourceSpec};
use crate::runtime::adapters::RuntimeDefinitionEnvelope;
use crate::runtime::policy::RuntimePolicySpec;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerPortableGraphPublication {
    pub policy: RuntimePolicySpec,
    pub sources: Vec<SourceSpec>,
    pub recipes: Vec<RecipeSpec>,
}

impl WorkerPortableGraphPublication {
    pub fn into_definition_envelope(self) -> RuntimeDefinitionEnvelope {
        RuntimeDefinitionEnvelope {
            policy: self.policy,
            sources: self.sources,
            recipes: self.recipes,
            source_families: Vec::new(),
            recipe_families: Vec::new(),
            unavailable_callbacks: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerGraphPublicationSummary {
    pub published_source_count: u64,
    pub published_recipe_count: u64,
    pub admitted_callback_count: u64,
    pub denied_callback_count: u64,
}
