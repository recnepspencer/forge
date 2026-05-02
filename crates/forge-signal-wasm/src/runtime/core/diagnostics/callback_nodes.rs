use crate::boundary::errors::ForgeSignalJsError;
use crate::runtime::compute_callbacks;
use crate::runtime::summaries::{
    public_callback_dependency_patch_summary, public_callback_read_ids, CallbackRuntimeNodeSummary,
};

use super::super::state::{StoredRecipeDefinition, StoredRecipeOrigin, WebSignalKind};
use super::super::RuntimeCore;

pub(super) const CALLBACK_UNAVAILABLE_FOR_REPLAY: &str = "computeCallbackUnavailableForReplay";

impl RuntimeCore {
    pub(super) fn callback_node_for_node(
        &self,
        node: forge_signal::facade::NodeId,
    ) -> Result<Option<CallbackRuntimeNodeSummary>, ForgeSignalJsError> {
        let Some(id) = self.nodes_by_id.get(&node).cloned() else {
            return Ok(None);
        };
        let store = self.lock_store()?;
        let Some(recipe) = store.recipes.get(&id) else {
            return Ok(None);
        };
        let diagnostics = self.lock_callback_diagnostics()?;
        let state = diagnostics.get(&id).cloned().unwrap_or_default();
        let summary = match (&recipe.origin, &recipe.definition) {
            (
                StoredRecipeOrigin::CallbackSignalTracked,
                StoredRecipeDefinition::Callback(callback_recipe),
            ) => {
                let registered = compute_callbacks::is_compute_registered(callback_recipe.token);
                CallbackRuntimeNodeSummary {
                    id: id.clone(),
                    node: node.to_string(),
                    api_family: self.web_signals.get(&id).map(|kind| match kind {
                        WebSignalKind::Input => "input".to_owned(),
                        WebSignalKind::Computed => "computed".to_owned(),
                        WebSignalKind::Output => "output".to_owned(),
                    }),
                    recipe_family: Some("callback".to_owned()),
                    purity_posture: state
                        .purity_posture
                        .clone()
                        .unwrap_or_else(|| "signalTracked".to_owned()),
                    current_reads: public_callback_read_ids(&state.current_reads),
                    host_capability_reads: state.host_capability_reads,
                    registered,
                    unavailable_reason: (!registered)
                        .then_some(CALLBACK_UNAVAILABLE_FOR_REPLAY.to_owned()),
                    token_slot: Some(callback_recipe.token.slot),
                    token_generation: Some(callback_recipe.token.generation),
                    last_runtime_read_breadth: state.last_runtime_read_breadth,
                    last_dependency_patch: state.last_dependency_patch.map(|patch| {
                        public_callback_dependency_patch_summary(
                            &patch.previous_reads,
                            &patch.current_reads,
                            patch.runtime_read_breadth,
                        )
                    }),
                    last_failure: state.last_failure,
                }
            }
            (StoredRecipeOrigin::CallbackConstantizedNoSignalReads, _) => {
                CallbackRuntimeNodeSummary {
                    id: id.clone(),
                    node: node.to_string(),
                    api_family: self.web_signals.get(&id).map(|kind| match kind {
                        WebSignalKind::Input => "input".to_owned(),
                        WebSignalKind::Computed => "computed".to_owned(),
                        WebSignalKind::Output => "output".to_owned(),
                    }),
                    recipe_family: Some("callbackConstantized".to_owned()),
                    purity_posture: state
                        .purity_posture
                        .clone()
                        .unwrap_or_else(|| "constantizedNoSignalReads".to_owned()),
                    current_reads: public_callback_read_ids(&state.current_reads),
                    host_capability_reads: state.host_capability_reads,
                    registered: false,
                    unavailable_reason: None,
                    token_slot: None,
                    token_generation: None,
                    last_runtime_read_breadth: state.last_runtime_read_breadth,
                    last_dependency_patch: state.last_dependency_patch.map(|patch| {
                        public_callback_dependency_patch_summary(
                            &patch.previous_reads,
                            &patch.current_reads,
                            patch.runtime_read_breadth,
                        )
                    }),
                    last_failure: state.last_failure,
                }
            }
            _ => return Ok(None),
        };
        Ok(Some(summary))
    }

    pub(super) fn callback_nodes_for_node_ids(
        &self,
        nodes: impl IntoIterator<Item = forge_signal::facade::NodeId>,
    ) -> Result<Vec<CallbackRuntimeNodeSummary>, ForgeSignalJsError> {
        let mut deduped = std::collections::BTreeSet::new();
        let mut summaries = Vec::new();
        for node in nodes {
            if !deduped.insert(node) {
                continue;
            }
            if let Some(summary) = self.callback_node_for_node(node)? {
                summaries.push(summary);
            }
        }
        Ok(summaries)
    }
}
