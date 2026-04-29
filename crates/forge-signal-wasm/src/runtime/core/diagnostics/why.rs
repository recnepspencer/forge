use crate::boundary::errors::ForgeSignalJsError;
use crate::runtime::compute_callbacks;
use crate::runtime::summaries::CallbackWhySummary;
use crate::runtime::summaries::WhySummary;

use super::super::state::{StoredRecipeDefinition, StoredRecipeOrigin, WebSignalKind};
use super::super::RuntimeCore;
use super::callback_nodes::CALLBACK_UNAVAILABLE_FOR_REPLAY;

impl RuntimeCore {
    pub fn why(&self, id: &str) -> Result<WhySummary, ForgeSignalJsError> {
        let node = self.node_for_id(id)?;
        let store = self.lock_store()?;
        let api_family = self.web_signals.get(id).map(|kind| match kind {
            WebSignalKind::Input => "input".to_owned(),
            WebSignalKind::Computed => "computed".to_owned(),
            WebSignalKind::Output => "output".to_owned(),
        });
        let recipe_family = store.recipes.get(id).map(|recipe| match recipe.origin {
            StoredRecipeOrigin::ExprSpec => "expr".to_owned(),
            StoredRecipeOrigin::CallbackSignalTracked => "callback".to_owned(),
            StoredRecipeOrigin::CallbackConstantizedNoSignalReads => {
                "callbackConstantized".to_owned()
            }
        });
        let callback = match store.recipes.get(id) {
            Some(recipe) => match (&recipe.origin, &recipe.definition) {
                (
                    StoredRecipeOrigin::CallbackSignalTracked,
                    StoredRecipeDefinition::Callback(callback_recipe),
                ) => {
                    let state = self
                        .lock_callback_diagnostics()?
                        .get(id)
                        .cloned()
                        .unwrap_or_default();
                    let registered =
                        compute_callbacks::is_compute_registered(callback_recipe.token);
                    Some(CallbackWhySummary {
                        purity_posture: state
                            .purity_posture
                            .unwrap_or_else(|| "signalTracked".to_owned()),
                        current_reads: state.current_reads,
                        registered,
                        unavailable_reason: (!registered)
                            .then_some(CALLBACK_UNAVAILABLE_FOR_REPLAY.to_owned()),
                        token_slot: Some(callback_recipe.token.slot),
                        token_generation: Some(callback_recipe.token.generation),
                        last_runtime_read_breadth: state.last_runtime_read_breadth,
                        last_dependency_patch: state.last_dependency_patch,
                        last_failure: state.last_failure,
                    })
                }
                (StoredRecipeOrigin::CallbackConstantizedNoSignalReads, _) => {
                    let state = self
                        .lock_callback_diagnostics()?
                        .get(id)
                        .cloned()
                        .unwrap_or_default();
                    Some(CallbackWhySummary {
                        purity_posture: state
                            .purity_posture
                            .unwrap_or_else(|| "constantizedNoSignalReads".to_owned()),
                        current_reads: state.current_reads,
                        registered: false,
                        unavailable_reason: None,
                        token_slot: None,
                        token_generation: None,
                        last_runtime_read_breadth: state.last_runtime_read_breadth,
                        last_dependency_patch: state.last_dependency_patch,
                        last_failure: state.last_failure,
                    })
                }
                _ => None,
            },
            None => None,
        };
        let explanation = self
            .runtime
            .diagnostics()
            .why(node)
            .map_err(ForgeSignalJsError::from)?;
        Ok(WhySummary {
            id: id.to_owned(),
            node: explanation.node.to_string(),
            api_family,
            recipe_family,
            state: format!("{:?}", explanation.state),
            upstream: explanation
                .upstream
                .iter()
                .map(|cause| format!("{cause:?}"))
                .collect(),
            changed_regions: explanation
                .changed_regions
                .iter()
                .map(|region| match &region.detail {
                    Some(detail) => format!("{}:{}", region.partition.0, detail),
                    None => region.partition.0.clone(),
                })
                .collect(),
            propagation_suppressed: explanation.propagation_suppressed,
            output_change: explanation
                .output_change
                .map(|change| format!("{change:?}")),
            output_identity: explanation.output_identity.map(String::from),
            callback,
        })
    }
}
