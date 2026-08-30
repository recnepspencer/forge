use crate::boundary::errors::WorthSignalJsError;

use super::state::BranchRuntimeState;
use super::RuntimeCore;

pub(super) struct MergeCallerState {
    active_branch_id: u64,
    active_state: BranchRuntimeState,
    source_branch_id: u64,
    source_state: BranchRuntimeState,
    target_branch_id: u64,
    target_state: BranchRuntimeState,
}

impl MergeCallerState {
    pub(super) fn capture(
        runtime: &mut RuntimeCore,
        source_branch_id: u64,
        target_branch_id: u64,
    ) -> Result<Self, WorthSignalJsError> {
        let active_branch_id = runtime.runtime.current_branch().id.0;
        let active_state = runtime.snapshot_branch_state();
        runtime
            .branch_states
            .insert(active_branch_id, active_state.clone());
        let source_state = state_for_branch(runtime, source_branch_id, &active_state)?;
        let target_state = state_for_branch(runtime, target_branch_id, &active_state)?;
        Ok(Self {
            active_branch_id,
            active_state,
            source_branch_id,
            source_state,
            target_branch_id,
            target_state,
        })
    }

    pub(super) fn source_state(&self) -> &BranchRuntimeState {
        &self.source_state
    }

    pub(super) fn target_state(&self) -> &BranchRuntimeState {
        &self.target_state
    }

    pub(super) fn stage_source(&self, runtime: &mut RuntimeCore) -> Result<(), WorthSignalJsError> {
        if self.active_branch_id == self.source_branch_id {
            Ok(())
        } else {
            runtime.switch_branch(self.source_branch_id)
        }
    }

    pub(super) fn restore_after_success(
        self,
        runtime: &mut RuntimeCore,
        merged_state: BranchRuntimeState,
    ) -> Result<(), WorthSignalJsError> {
        runtime
            .branch_states
            .insert(self.target_branch_id, merged_state);
        if self.active_branch_id == self.source_branch_id {
            return runtime.restore_branch_state(self.source_state);
        }
        runtime.switch_branch(self.active_branch_id)
    }

    pub(super) fn restore_after_failure(
        self,
        runtime: &mut RuntimeCore,
        merge_error: WorthSignalJsError,
    ) -> WorthSignalJsError {
        runtime
            .branch_states
            .insert(self.active_branch_id, self.active_state.clone());
        let restoration = if runtime.runtime.current_branch().id.0 == self.active_branch_id {
            runtime.restore_branch_state(self.active_state)
        } else {
            runtime.switch_branch(self.active_branch_id)
        };
        runtime
            .branch_states
            .insert(self.source_branch_id, self.source_state);
        runtime
            .branch_states
            .insert(self.target_branch_id, self.target_state);
        match restoration {
            Ok(()) => merge_error,
            Err(restoration_error) => WorthSignalJsError::internal(format!(
                "branch merge failed with `{}` and caller restoration failed with `{}`",
                merge_error.message, restoration_error.message
            )),
        }
    }
}

fn state_for_branch(
    runtime: &RuntimeCore,
    branch_id: u64,
    active_state: &BranchRuntimeState,
) -> Result<BranchRuntimeState, WorthSignalJsError> {
    if branch_id == runtime.runtime.current_branch().id.0 {
        return Ok(active_state.clone());
    }
    runtime
        .branch_states
        .get(&branch_id)
        .cloned()
        .ok_or_else(|| {
            WorthSignalJsError::internal(format!(
                "branch `{branch_id}` has no staged Wasm runtime state"
            ))
        })
}

#[cfg(test)]
mod tests {
    use crate::expression::model::SignalValue;
    use crate::recipe::model::SourceSpec;
    use crate::runtime::compute_callbacks::ComputeCallbackInvocationResult;
    use crate::runtime::policy::RuntimePolicySpec;

    use super::RuntimeCore;

    #[test]
    fn source_staging_failure_is_returned_without_moving_the_caller() {
        let mut runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
        runtime
            .define_source(SourceSpec {
                id: "counter".to_owned(),
                initial: SignalValue::Number(1.0),
                produces_aspects: None,
            })
            .unwrap();
        runtime
            .define_web_computed_native_callback(
                "derived".to_owned(),
                Box::new(|| {
                    Ok(ComputeCallbackInvocationResult {
                        value: SignalValue::Number(2.0),
                        captured_read_ids: vec!["counter".to_owned()],
                        captured_host_capability_reads: Vec::new(),
                        runtime_read_breadth: 1,
                        return_serialization_breadth: 1,
                    })
                }),
            )
            .unwrap();
        let main_branch = runtime.current_branch().id.0;
        let feature_branch = runtime.create_branch("feature".to_owned()).unwrap().id.0;
        let callback = runtime
            .branch_states
            .get_mut(&feature_branch)
            .unwrap()
            .store
            .recipes
            .iter_mut()
            .find(|recipe| recipe.id == "derived")
            .unwrap()
            .callback
            .as_mut()
            .unwrap();
        callback.token_generation = callback.token_generation.saturating_add(1);

        let error = runtime
            .merge_branches(feature_branch, main_branch)
            .unwrap_err();

        assert_eq!(error.code, "computeCallbackUnavailableForRestore");
        assert_eq!(runtime.current_branch().id.0, main_branch);
        assert_eq!(
            runtime.read_value("counter").unwrap(),
            SignalValue::Number(1.0)
        );
    }
}
