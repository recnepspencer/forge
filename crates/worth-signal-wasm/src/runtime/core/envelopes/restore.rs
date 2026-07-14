use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};

use crate::boundary::errors::WorthSignalJsError;
use crate::recipe::model::{RecipeSpec, SetValue, TransactionOp};
use crate::runtime::adapters::{RuntimeDefinitionEnvelope, RuntimeEnvelope};
use crate::runtime::summaries::RuntimeStoreSnapshot;

use super::{
    ExactRuntimeRestoreArtifact, RuntimeCore, CALLBACK_UNAVAILABLE_FOR_RUNTIME_ENVELOPE_IMPORT,
};

impl RuntimeCore {
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn replace_runtime_envelope(
        &mut self,
        envelope: RuntimeEnvelope,
    ) -> Result<(), WorthSignalJsError> {
        reject_unavailable_callbacks(self, &envelope.definitions.unavailable_callbacks)?;

        let mut rebuilt = RuntimeCore::new(envelope.definitions.policy.clone())?;
        for family in envelope.definitions.source_families {
            rebuilt.define_source_family(family)?;
        }
        for family in envelope.definitions.recipe_families {
            rebuilt.define_keyed_recipe_family(family)?;
        }
        let source_ids = envelope
            .definitions
            .sources
            .iter()
            .map(|source| source.id.clone())
            .collect::<Vec<_>>();
        for source in envelope.definitions.sources {
            rebuilt.define_source(source)?;
        }
        let worker_public_output_ids = envelope.definitions.worker_public_output_ids;
        define_recipes_in_dependency_order(&mut rebuilt, envelope.definitions.recipes, source_ids)?;
        rebuilt.restore_snapshot(envelope.snapshot)?;
        rebuilt.mark_worker_public_outputs(worker_public_output_ids)?;
        *self = rebuilt;
        Ok(())
    }

    pub(crate) fn replace_runtime_envelope_portable_artifact(
        &mut self,
        definitions: RuntimeDefinitionEnvelope,
        state: RuntimeStoreSnapshot,
    ) -> Result<(), WorthSignalJsError> {
        reject_unavailable_callbacks(self, &definitions.unavailable_callbacks)?;

        let RuntimeDefinitionEnvelope {
            policy,
            sources,
            recipes,
            source_families,
            recipe_families,
            worker_public_output_ids,
            unavailable_callbacks: _,
        } = definitions;

        let mut rebuilt = RuntimeCore::new(policy)?;
        for family in source_families {
            rebuilt.define_source_family(family)?;
        }
        for family in recipe_families {
            rebuilt.define_keyed_recipe_family(family)?;
        }
        let source_ids = sources
            .iter()
            .map(|source| source.id.clone())
            .collect::<Vec<_>>();
        for source in sources {
            rebuilt.define_source(source)?;
        }
        define_recipes_in_dependency_order(&mut rebuilt, recipes, source_ids)?;

        if !state.sources.is_empty() {
            rebuilt.apply_transaction(vec![TransactionOp::SetMany {
                values: state
                    .sources
                    .iter()
                    .map(|source| SetValue {
                        id: source.id.clone(),
                        value: source.value.clone(),
                        aspect: None,
                        aspects: source.produces_aspects.clone(),
                    })
                    .collect(),
            }])?;
        }

        rebuilt.mark_worker_public_outputs(worker_public_output_ids)?;
        *self = rebuilt;
        Ok(())
    }

    pub(crate) fn replace_runtime_envelope_exact(
        &mut self,
        artifact: ExactRuntimeRestoreArtifact,
    ) -> Result<(), WorthSignalJsError> {
        let mut rebuilt = RuntimeCore::new(artifact.policy.clone())?;
        rebuilt.catalog = artifact.catalog;
        rebuilt.web_signals = artifact.web_signals;
        rebuilt.nodes_by_id = artifact.nodes_by_id;
        rebuilt.dense_grids = artifact.dense_grids;
        rebuilt.branch_states = artifact.branch_states;
        rebuilt.snapshot_states = artifact.snapshot_states;
        rebuilt.runtime_snapshots = artifact.runtime_snapshots;
        rebuilt.web_metrics = artifact.web_metrics;
        rebuilt.store = Arc::new(Mutex::new(artifact.store));
        rebuilt.callback_diagnostics = Arc::new(Mutex::new(artifact.callback_diagnostics));
        rebuilt.restore_snapshot(artifact.snapshot)?;
        *self = rebuilt;
        Ok(())
    }
}

fn reject_unavailable_callbacks(
    runtime: &mut RuntimeCore,
    unavailable_callbacks: &[crate::runtime::adapters::UnavailableCallbackArtifact],
) -> Result<(), WorthSignalJsError> {
    if unavailable_callbacks.is_empty() {
        return Ok(());
    }

    runtime
        .web_metrics
        .compute_callback_missing_unavailability_count = runtime
        .web_metrics
        .compute_callback_missing_unavailability_count
        .saturating_add(unavailable_callbacks.len() as u64);
    let ids = unavailable_callbacks
        .iter()
        .map(|artifact| artifact.id.clone())
        .collect::<Vec<_>>()
        .join(", ");
    Err(WorthSignalJsError::callback_failure(
        CALLBACK_UNAVAILABLE_FOR_RUNTIME_ENVELOPE_IMPORT,
        format!(
            "runtime envelope import cannot restore callback-backed nodes without live callback registrations: {ids}"
        ),
        Some(ids),
    ))
}

fn define_recipes_in_dependency_order(
    rebuilt: &mut RuntimeCore,
    recipes: Vec<RecipeSpec>,
    source_ids: Vec<String>,
) -> Result<(), WorthSignalJsError> {
    let mut known_ids = source_ids.into_iter().collect::<BTreeSet<_>>();
    let mut pending = recipes;
    while !pending.is_empty() {
        let mut next_pending = Vec::new();
        let mut progressed = false;
        for recipe in pending {
            if recipe
                .reads
                .iter()
                .all(|read| known_ids.contains(read.id()))
            {
                known_ids.insert(recipe.id.clone());
                rebuilt.define_recipe(recipe)?;
                progressed = true;
            } else {
                next_pending.push(recipe);
            }
        }
        if !progressed {
            let unresolved = next_pending
                .iter()
                .map(|recipe| {
                    let missing = recipe
                        .reads
                        .iter()
                        .filter(|read| !known_ids.contains(read.id()))
                        .map(|read| read.id().to_owned())
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("{} -> [{missing}]", recipe.id)
                })
                .collect::<Vec<_>>()
                .join("; ");
            return Err(WorthSignalJsError::invalid_input(format!(
                "runtime envelope definitions contained unresolved recipe reads: {unresolved}"
            )));
        }
        pending = next_pending;
    }
    Ok(())
}
