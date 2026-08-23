use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};

use super::{
    WorthQueryAsyncResultTransitionBatch, WorthQueryAsyncSourceBindingError,
    WorthQueryAsyncSourceBindingErrorKind, WorthQueryInstalledOwnedAsyncDeclaration,
    WorthQueryLiveArtifactTarget, WorthQueryLiveView, WorthQueryRuntime,
    WorthQueryRuntimeAsyncResultState, WorthQueryRuntimeAsyncResultStateKind,
};

impl WorthQueryRuntime {
    pub fn supersede_owned_bridge_async_live_view<T>(
        &mut self,
        view: &WorthQueryLiveView<T>,
        prior: &WorthQueryInstalledOwnedAsyncDeclaration,
        displacing: &WorthQueryInstalledOwnedAsyncDeclaration,
    ) -> Result<WorthQueryAsyncResultTransitionBatch, WorthQueryAsyncSourceBindingError> {
        if prior.runtime_provenance() != self.runtime_provenance()
            || displacing.runtime_provenance() != self.runtime_provenance()
            || prior.signal_graph_instance() != displacing.signal_graph_instance()
            || prior.identity() == displacing.identity()
        {
            return Err(WorthQueryAsyncSourceBindingError::new(
                WorthQueryAsyncSourceBindingErrorKind::ForeignRequest,
                "owned async supersession requires distinct declarations from this runtime graph",
            ));
        }
        let runtime_provenance = self.runtime_provenance();
        let target = WorthQueryLiveArtifactTarget::from_view_name(view.name());
        let state = self.live_subscriptions.get_mut(&target).ok_or_else(|| {
            WorthQueryAsyncSourceBindingError::new(
                WorthQueryAsyncSourceBindingErrorKind::MissingLiveSubscription,
                format!("live view `{}` has no active subscription", view.name()),
            )
        })?;
        let current = state
            .async_result_state
            .as_ref()
            .map(WorthQueryRuntimeAsyncResultState::kind);
        if !matches!(
            current,
            Some(
                WorthQueryRuntimeAsyncResultStateKind::Pending
                    | WorthQueryRuntimeAsyncResultStateKind::Current
                    | WorthQueryRuntimeAsyncResultStateKind::Unresolved
                    | WorthQueryRuntimeAsyncResultStateKind::Superseded
            )
        ) {
            return Err(WorthQueryAsyncSourceBindingError::new(
                WorthQueryAsyncSourceBindingErrorKind::IllegalResultTransition,
                format!("live view `{}` is no longer pending", view.name()),
            ));
        }
        let binding = state.async_source_binding.as_ref().ok_or_else(|| {
            WorthQueryAsyncSourceBindingError::new(
                WorthQueryAsyncSourceBindingErrorKind::MissingBinding,
                format!("live view `{}` has no async binding", view.name()),
            )
        })?;
        if binding.declaration_identity_reference() != prior.lowered_declaration_identity() {
            return Err(WorthQueryAsyncSourceBindingError::new(
                WorthQueryAsyncSourceBindingErrorKind::ForeignRequest,
                format!(
                    "live view `{}` is not bound to the declaration being superseded",
                    view.name()
                ),
            ));
        }
        if current == Some(WorthQueryRuntimeAsyncResultStateKind::Superseded) {
            return retained_terminal_batch(runtime_provenance, view, state);
        }
        let basis = binding.current_basis_identity();
        let checkpoint = binding.current_generation_identity();
        let binding_identity = binding.binding_identity().clone();
        let causality =
            worth_query_evidence_identity(WorthQueryEvidenceScope::RuntimeStateSnapshot)
                .field_shape(
                    WorthQueryEvidenceTag::new("identity_family"),
                    "worth_query_owned_async_supersession_v1",
                )
                .field_shape(WorthQueryEvidenceTag::new("live_target"), view.name())
                .field_shape(
                    WorthQueryEvidenceTag::new("prior_request"),
                    prior.identity().canonical_identity(),
                )
                .field_shape(
                    WorthQueryEvidenceTag::new("displacing_request"),
                    displacing.identity().canonical_identity(),
                )
                .seal();
        let superseded = WorthQueryRuntimeAsyncResultState::new(
            WorthQueryRuntimeAsyncResultStateKind::Superseded,
            &causality,
            &basis,
            &checkpoint,
        );
        state.async_result_state = Some(superseded.clone());
        let remask_posture = state.remask_posture.clone();
        Ok(WorthQueryAsyncResultTransitionBatch::admitted(
            runtime_provenance,
            view.name(),
            binding_identity,
            basis,
            checkpoint,
            remask_posture,
            vec![superseded],
            0,
        ))
    }

    pub fn deny_owned_bridge_async_live_view<T>(
        &mut self,
        view: &WorthQueryLiveView<T>,
        declaration: &WorthQueryInstalledOwnedAsyncDeclaration,
    ) -> Result<WorthQueryAsyncResultTransitionBatch, WorthQueryAsyncSourceBindingError> {
        self.transition_owned_bridge_async_live_view(
            view,
            declaration,
            WorthQueryRuntimeAsyncResultStateKind::Denied,
            "worth_query_owned_async_before_effects_denial_v1",
            "denied_request",
        )
    }

    pub fn cancel_owned_bridge_async_live_view<T>(
        &mut self,
        view: &WorthQueryLiveView<T>,
        declaration: &WorthQueryInstalledOwnedAsyncDeclaration,
    ) -> Result<WorthQueryAsyncResultTransitionBatch, WorthQueryAsyncSourceBindingError> {
        self.transition_owned_bridge_async_live_view(
            view,
            declaration,
            WorthQueryRuntimeAsyncResultStateKind::Cancelled,
            "worth_query_owned_async_before_effects_cancellation_v1",
            "cancelled_request",
        )
    }

    fn transition_owned_bridge_async_live_view<T>(
        &mut self,
        view: &WorthQueryLiveView<T>,
        declaration: &WorthQueryInstalledOwnedAsyncDeclaration,
        terminal_kind: WorthQueryRuntimeAsyncResultStateKind,
        evidence_family: &'static str,
        request_field: &'static str,
    ) -> Result<WorthQueryAsyncResultTransitionBatch, WorthQueryAsyncSourceBindingError> {
        if declaration.runtime_provenance() != self.runtime_provenance()
            || self
                .conditional_signal_runtime
                .as_ref()
                .is_none_or(|runtime| {
                    declaration.signal_graph_instance() != runtime.owned_signal_graph_instance_id()
                })
        {
            return Err(WorthQueryAsyncSourceBindingError::new(
                WorthQueryAsyncSourceBindingErrorKind::ForeignRequest,
                "owned async denial requires a declaration from this runtime graph",
            ));
        }
        let runtime_provenance = self.runtime_provenance();
        let target = WorthQueryLiveArtifactTarget::from_view_name(view.name());
        let state = self.live_subscriptions.get_mut(&target).ok_or_else(|| {
            WorthQueryAsyncSourceBindingError::new(
                WorthQueryAsyncSourceBindingErrorKind::MissingLiveSubscription,
                format!("live view `{}` has no active subscription", view.name()),
            )
        })?;
        let current = state
            .async_result_state
            .as_ref()
            .map(WorthQueryRuntimeAsyncResultState::kind);
        if current == Some(terminal_kind) {
            return retained_terminal_batch(runtime_provenance, view, state);
        }
        if current != Some(WorthQueryRuntimeAsyncResultStateKind::Pending) {
            return Err(WorthQueryAsyncSourceBindingError::new(
                WorthQueryAsyncSourceBindingErrorKind::IllegalResultTransition,
                format!("live view `{}` is no longer pending", view.name()),
            ));
        }
        let binding = state.async_source_binding.as_ref().ok_or_else(|| {
            WorthQueryAsyncSourceBindingError::new(
                WorthQueryAsyncSourceBindingErrorKind::MissingBinding,
                format!("live view `{}` has no async binding", view.name()),
            )
        })?;
        if binding.declaration_identity_reference() != declaration.lowered_declaration_identity() {
            return Err(WorthQueryAsyncSourceBindingError::new(
                WorthQueryAsyncSourceBindingErrorKind::ForeignRequest,
                format!(
                    "live view `{}` is not bound to the denied declaration",
                    view.name()
                ),
            ));
        }
        let basis = binding.current_basis_identity();
        let checkpoint = binding.current_generation_identity();
        let binding_identity = binding.binding_identity().clone();
        let causality =
            worth_query_evidence_identity(WorthQueryEvidenceScope::RuntimeStateSnapshot)
                .field_shape(
                    WorthQueryEvidenceTag::new("identity_family"),
                    evidence_family,
                )
                .field_shape(WorthQueryEvidenceTag::new("live_target"), view.name())
                .field_shape(
                    WorthQueryEvidenceTag::new(request_field),
                    declaration.identity().canonical_identity(),
                )
                .seal();
        let terminal =
            WorthQueryRuntimeAsyncResultState::new(terminal_kind, &causality, &basis, &checkpoint);
        state.async_result_state = Some(terminal.clone());
        let remask_posture = state.remask_posture.clone();
        Ok(WorthQueryAsyncResultTransitionBatch::admitted(
            runtime_provenance,
            view.name(),
            binding_identity,
            basis,
            checkpoint,
            remask_posture,
            vec![terminal],
            0,
        ))
    }
}

fn retained_terminal_batch<T>(
    runtime_provenance: super::WorthQueryRuntimeProvenance,
    view: &WorthQueryLiveView<T>,
    state: &super::WorthQueryRuntimeLiveSubscriptionState,
) -> Result<WorthQueryAsyncResultTransitionBatch, WorthQueryAsyncSourceBindingError> {
    let binding = state.async_source_binding.as_ref().ok_or_else(|| {
        WorthQueryAsyncSourceBindingError::new(
            WorthQueryAsyncSourceBindingErrorKind::MissingBinding,
            format!("live view `{}` has no async binding", view.name()),
        )
    })?;
    let retained = state.async_result_state.clone().ok_or_else(|| {
        WorthQueryAsyncSourceBindingError::new(
            WorthQueryAsyncSourceBindingErrorKind::IllegalResultTransition,
            format!("live view `{}` has no retained terminal state", view.name()),
        )
    })?;
    Ok(WorthQueryAsyncResultTransitionBatch::admitted(
        runtime_provenance,
        view.name(),
        binding.binding_identity().clone(),
        binding.current_basis_identity(),
        binding.current_generation_identity(),
        state.remask_posture.clone(),
        vec![retained],
        0,
    ))
}
