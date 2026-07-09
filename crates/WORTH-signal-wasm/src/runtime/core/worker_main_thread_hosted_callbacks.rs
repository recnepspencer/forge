use crate::boundary::errors::WORTHSignalJsError;
use crate::expression::model::SignalValue;
use crate::recipe::model::SetValue;
use crate::runtime::compute_callbacks::CapturedHostCapabilityRead;

use super::aspects::{bump_aspects, defaulted_produced_aspects};
use super::evaluation::canonicalize_callback_reads;
use super::state::StoredRecipeDefinition;
use super::RuntimeCore;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct MainThreadHostedCallbackClosedInput {
    pub id: String,
    pub value: SignalValue,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct MainThreadHostedCallbackClosedRequest {
    pub callback_id: String,
    pub closed_inputs: Vec<MainThreadHostedCallbackClosedInput>,
    pub host_capability_read_count: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct MainThreadHostedCallbackAdmission {
    pub callback_id: String,
    pub value: SignalValue,
    pub captured_read_ids: Vec<String>,
    pub captured_host_capability_reads: Vec<CapturedHostCapabilityRead>,
    pub runtime_read_breadth: u64,
}

impl RuntimeCore {
    pub(crate) fn main_thread_hosted_callback_closed_request(
        &self,
        callback_id: &str,
    ) -> Result<MainThreadHostedCallbackClosedRequest, WORTHSignalJsError> {
        let store = self.lock_store()?;
        let recipe = store.recipes.get(callback_id).ok_or_else(|| {
            WORTHSignalJsError::invalid_input(format!(
                "unknown main-thread-hosted callback `{callback_id}`"
            ))
        })?;
        let StoredRecipeDefinition::Callback(callback) = &recipe.definition else {
            return Err(WORTHSignalJsError::invalid_input(format!(
                "signal `{callback_id}` is not a main-thread-hosted callback"
            )));
        };

        let mut closed_inputs = Vec::with_capacity(callback.reads.len());
        for read in &callback.reads {
            let id = read.id().to_owned();
            let value = store.read_value(&id).ok_or_else(|| {
                WORTHSignalJsError::invalid_input(format!(
                    "main-thread-hosted callback `{callback_id}` cannot close over uninitialized read `{id}`"
                ))
            })?;
            closed_inputs.push(MainThreadHostedCallbackClosedInput { id, value });
        }

        Ok(MainThreadHostedCallbackClosedRequest {
            callback_id: callback_id.to_owned(),
            closed_inputs,
            host_capability_read_count: callback.host_capability_reads.len() as u64,
        })
    }

    pub(crate) fn admit_main_thread_hosted_callback_result(
        &mut self,
        admission: MainThreadHostedCallbackAdmission,
    ) -> Result<u32, WORTHSignalJsError> {
        let canonical_reads = canonicalize_callback_reads(admission.captured_read_ids.clone());
        let mut store = self.lock_store()?;
        let recipe = store
            .recipes
            .get_mut(&admission.callback_id)
            .ok_or_else(|| {
                WORTHSignalJsError::invalid_input(format!(
                    "unknown main-thread-hosted callback `{}`",
                    admission.callback_id
                ))
            })?;
        let StoredRecipeDefinition::Callback(callback) = &mut recipe.definition else {
            return Err(WORTHSignalJsError::invalid_input(format!(
                "signal `{}` is not a main-thread-hosted callback",
                admission.callback_id
            )));
        };

        if canonical_reads != callback.reads {
            return Err(WORTHSignalJsError::invalid_input(
                "main-thread-hosted callback result must use exactly the closed worker-issued read frontier",
            ));
        }

        let produced_aspects = defaulted_produced_aspects(callback.produces_aspects.as_deref());
        callback.host_capability_reads = admission.captured_host_capability_reads.clone();
        recipe.value = admission.value;
        recipe.initialized = true;
        recipe.version = bump_aspects(recipe.version, &produced_aspects);
        drop(store);

        {
            let mut diagnostics = self.lock_callback_diagnostics()?;
            let state = diagnostics
                .entry(admission.callback_id.clone())
                .or_default();
            state.current_reads = admission.captured_read_ids;
            state.host_capability_reads = admission.captured_host_capability_reads;
            state.last_runtime_read_breadth = admission.runtime_read_breadth;
            state.last_failure = None;
        }

        let active_branch_id = self.runtime.current_branch().id.0;
        self.branch_states
            .insert(active_branch_id, self.snapshot_branch_state());
        Ok(1)
    }
}

impl From<MainThreadHostedCallbackClosedInput> for SetValue {
    fn from(input: MainThreadHostedCallbackClosedInput) -> Self {
        Self {
            id: input.id,
            value: input.value,
            aspect: None,
            aspects: None,
        }
    }
}
