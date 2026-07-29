use worth_runtime_bridge::facade::BridgeMixedCauseOrdering;

use super::async_source_transition_plan::plan_async_result_transitions;
use super::{
    WorthQueryAsyncResultTransitionBatch, WorthQueryAsyncSourceBindingError,
    WorthQueryAsyncSourceBindingErrorKind, WorthQueryLiveArtifactTarget, WorthQueryLiveView,
    WorthQueryRuntime, WorthQueryRuntimeAsyncResultState,
};

impl WorthQueryRuntime {
    pub fn admit_bridge_async_result_transitions<T>(
        &mut self,
        view: &WorthQueryLiveView<T>,
        ordering: &BridgeMixedCauseOrdering,
    ) -> Result<WorthQueryAsyncResultTransitionBatch, WorthQueryAsyncSourceBindingError> {
        let target = WorthQueryLiveArtifactTarget::from_view_name(view.name());
        let state = self.live_subscriptions.get(&target).ok_or_else(|| {
            WorthQueryAsyncSourceBindingError::new(
                WorthQueryAsyncSourceBindingErrorKind::MissingLiveSubscription,
                format!("live view `{}` has no active subscription", view.name()),
            )
        })?;
        let binding = state.async_source_binding.clone().ok_or_else(|| {
            WorthQueryAsyncSourceBindingError::new(
                WorthQueryAsyncSourceBindingErrorKind::MissingBinding,
                format!("live view `{}` has no async source binding", view.name()),
            )
        })?;
        let prior_kind = state
            .async_result_state
            .as_ref()
            .map(WorthQueryRuntimeAsyncResultState::kind);
        let plan = plan_async_result_transitions(view.name(), binding, prior_kind, ordering)?;
        let (binding, projections, suppressed_duplicate_count) = plan.into_parts();

        let basis = state.installation.basis_binding_identity().clone();
        let checkpoint = state.active_lane_handle.checkpoint_identity().clone();
        let mut states = Vec::with_capacity(projections.len());
        for projection in &projections {
            states.push(
                self.project_async_result_state(view.name(), projection, &basis, &checkpoint)
                    .map_err(|error| {
                        WorthQueryAsyncSourceBindingError::new(
                            WorthQueryAsyncSourceBindingErrorKind::ProjectionFailed,
                            format!("{error:?}"),
                        )
                    })?,
            );
        }
        let binding_identity = binding.binding_identity().clone();
        self.live_subscriptions
            .get_mut(&target)
            .expect("validated live target remains installed")
            .async_source_binding = Some(binding);
        Ok(WorthQueryAsyncResultTransitionBatch::admitted(
            binding_identity,
            states,
            suppressed_duplicate_count,
        ))
    }
}
