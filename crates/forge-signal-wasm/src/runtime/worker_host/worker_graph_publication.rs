use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::boundary::errors::ForgeSignalJsError;
use crate::recipe::model::{RecipeSpec, SourceSpec};
use crate::runtime::adapters::RuntimeDefinitionEnvelope;
use crate::runtime::policy::RuntimePolicySpec;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerPortableGraphPublication {
    pub policy: RuntimePolicySpec,
    pub sources: Vec<SourceSpec>,
    pub recipes: Vec<RecipeSpec>,
    #[serde(default)]
    pub output_ids: Vec<String>,
}

impl WorkerPortableGraphPublication {
    pub(in crate::runtime::worker_host) fn validate_public_output_ids(
        &self,
    ) -> Result<(), ForgeSignalJsError> {
        let recipe_ids = self
            .recipes
            .iter()
            .map(|recipe| recipe.id.as_str())
            .collect::<BTreeSet<_>>();
        let mut output_ids = BTreeSet::new();
        for output_id in &self.output_ids {
            if output_id.trim().is_empty() {
                return Err(ForgeSignalJsError::invalid_input(
                    "worker portable graph publication rejects blank output ids",
                ));
            }
            if !output_ids.insert(output_id.as_str()) {
                return Err(ForgeSignalJsError::invalid_input(format!(
                    "worker portable graph publication rejects duplicate output id `{output_id}`"
                )));
            }
            if !recipe_ids.contains(output_id.as_str()) {
                return Err(ForgeSignalJsError::invalid_input(format!(
                    "worker portable graph publication output id `{output_id}` must name a published recipe"
                )));
            }
        }
        Ok(())
    }

    pub fn into_definition_envelope(self) -> RuntimeDefinitionEnvelope {
        RuntimeDefinitionEnvelope {
            policy: self.policy,
            sources: self.sources,
            recipes: self.recipes,
            source_families: Vec::new(),
            recipe_families: Vec::new(),
            worker_public_output_ids: self.output_ids,
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
