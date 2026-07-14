use worth_relational::facade::history::BranchId;
use worth_relational::facade::runtime::RelationalRuntime;
use worth_runtime_bridge::facade::RuntimeBridge;

use crate::declarative_live::DeclarativeLiveQueryRequest;
use crate::memory_workspace::{
    WorthQueryEntity, WorthQueryLivePatch, WorthQueryLiveViewHandle, WorthQueryMutationReceipt,
    WorthQuerySnapshotIdentity, WorthQueryWorkspaceError,
};
use crate::schema_view::QuerySchemaView;
use crate::session_label::WorthQuerySessionLabel;
use crate::subscription::SubscriptionActivationInput;

use crate::runtime::{
    WorthQueryBackendAdmissibleMutation, WorthQueryBackendMergeAuthority, WorthQueryEffectPolicy,
    WorthQueryExistingTruthAssertionDenial, WorthQueryExistingTruthBindingDenial,
    WorthQueryExistingTruthProbe, WorthQueryExistingTruthProbeDenial,
    WorthQueryExistingTruthTargetBinding, WorthQueryIntentDeclaration, WorthQueryIntentExecution,
    WorthQueryLiveArtifactTarget, WorthQueryPreviewBasisAdmission, WorthQueryRuntimeBackend,
    WorthQueryRuntimeError, WorthQueryRuntimeEvidenceAuthority,
    WorthQueryRuntimeInspectionEvidence, WorthQueryRuntimeSupportProfile,
    WorthQueryVerifiedExistingTruthAssertion, WorthQueryWriteReceipt,
};

use super::bootstrap::BridgeBackedRuntimeBootstrap;

pub struct WorthQueryBridgeBackedRuntimeBackend {
    relational_runtime: Option<RelationalRuntime>,
    runtime_bridge: RuntimeBridge,
    schema_adapter: Box<dyn super::WorthQueryRuntimeSchemaAdapter>,
    source_adapter: Box<dyn super::WorthQueryRuntimeSourceAdapter>,
    snapshot_identity: Option<Box<dyn super::WorthQueryRuntimeSnapshotIdentityAdapter>>,
    existing_truth_verification:
        Option<Box<dyn super::WorthQueryRuntimeExistingTruthVerificationAdapter>>,
    write_authority: Box<dyn super::WorthQueryRuntimeWriteAuthorityAdapter>,
    signal_sink: Box<dyn super::WorthQueryRuntimeSignalSinkAdapter>,
    subscription_activation: Box<dyn super::WorthQueryRuntimeSubscriptionActivationAdapter>,
    preview_basis: Box<dyn super::WorthQueryRuntimePreviewBasisAdapter>,
    inspector_evidence: Box<dyn super::WorthQueryRuntimeInspectorEvidenceAdapter>,
    declaration_initialization:
        Option<Box<dyn super::WorthQueryRuntimeDeclarationInitializationAdapter>>,
    intent_authority: Option<Box<dyn super::WorthQueryIntentAuthorityAdapter>>,
    support_profile: WorthQueryRuntimeSupportProfile,
}

impl WorthQueryBridgeBackedRuntimeBackend {
    #[cfg(test)]
    pub(crate) fn from_parts(
        parts: super::WorthQueryRuntimeBackendParts,
    ) -> Result<Self, WorthQueryRuntimeError> {
        Ok(Self::from_validated_bootstrap(
            parts.lower_bridge_backed_bootstrap()?,
        ))
    }

    pub(in crate::runtime) fn from_validated_bootstrap(
        bootstrap: BridgeBackedRuntimeBootstrap,
    ) -> Self {
        Self {
            relational_runtime: bootstrap.relational_runtime,
            runtime_bridge: bootstrap.runtime_bridge,
            schema_adapter: bootstrap.schema_adapter,
            source_adapter: bootstrap.source_adapter,
            snapshot_identity: bootstrap.snapshot_identity,
            existing_truth_verification: bootstrap.existing_truth_verification,
            write_authority: bootstrap.write_authority,
            signal_sink: bootstrap.signal_sink,
            subscription_activation: bootstrap.subscription_activation,
            preview_basis: bootstrap.preview_basis,
            inspector_evidence: bootstrap.inspector_evidence,
            declaration_initialization: bootstrap.declaration_initialization,
            intent_authority: bootstrap.intent_authority,
            support_profile: bootstrap.support_profile,
        }
    }
}

impl WorthQueryRuntimeBackend for WorthQueryBridgeBackedRuntimeBackend {
    fn support_profile(&self) -> WorthQueryRuntimeSupportProfile {
        self.support_profile.clone()
    }

    fn current_snapshot_identity(&self) -> WorthQuerySnapshotIdentity {
        match self.snapshot_identity.as_ref() {
            Some(adapter) => adapter.current_snapshot_identity(),
            None => super::unavailable_snapshot_identity(),
        }
    }

    fn declare_live_view(
        &mut self,
        name: String,
        request: DeclarativeLiveQueryRequest,
        schema_view: QuerySchemaView,
    ) -> Result<WorthQueryLiveViewHandle, WorthQueryWorkspaceError> {
        self.source_adapter
            .declare_live_view(name, request, schema_view)
    }

    fn close_live_view(&mut self, name: &str) -> Result<(), WorthQueryWorkspaceError> {
        self.source_adapter.close_live_view(name)
    }

    fn admit_live_view_declaration(
        &self,
        name: &str,
        request: &DeclarativeLiveQueryRequest,
        schema_view: &QuerySchemaView,
    ) -> Result<super::LiveViewDeclarationAdmissionBoundaryReceipt, WorthQueryWorkspaceError> {
        self.schema_adapter
            .admit_live_view(name, request, schema_view)
    }

    fn write(
        &mut self,
        mutation: WorthQueryBackendAdmissibleMutation,
    ) -> Result<WorthQueryMutationReceipt, WorthQueryWorkspaceError> {
        let expected_mutation = mutation.clone();
        let write_execution = self.write_authority.write(
            &self.runtime_bridge,
            self.relational_runtime.as_mut(),
            mutation,
        )?;
        if let Some(message) =
            write_execution.drift_from_backend_admissible_mutation(&expected_mutation)
        {
            return Err(WorthQueryWorkspaceError::new(message));
        }
        let receipt = write_execution.mutation_receipt().clone();
        let routed = self.signal_sink.route_write_receipt(&receipt)?;
        if let Some(message) = routed.drift_from_mutation_receipt(&receipt) {
            return Err(WorthQueryWorkspaceError::new(message));
        }
        Ok(receipt)
    }

    fn write_batch(
        &mut self,
        mutations: Vec<WorthQueryBackendAdmissibleMutation>,
    ) -> Result<Vec<WorthQueryMutationReceipt>, WorthQueryWorkspaceError> {
        let expected_mutations = mutations.clone();
        let write_executions = self.write_authority.write_batch(
            &self.runtime_bridge,
            self.relational_runtime.as_mut(),
            mutations,
        )?;
        for (mutation, execution) in expected_mutations.iter().zip(write_executions.iter()) {
            if let Some(message) = execution.drift_from_backend_admissible_mutation(mutation) {
                return Err(WorthQueryWorkspaceError::new(message));
            }
        }
        let receipts = write_executions
            .iter()
            .map(|execution| execution.mutation_receipt().clone())
            .collect::<Vec<_>>();
        let routed = self.signal_sink.route_write_batch(&receipts)?;
        if routed.len() != receipts.len() {
            return Err(WorthQueryWorkspaceError::new(format!(
                "signal invalidation routing batch width drifted from write batch: expected `{}`, found `{}`",
                receipts.len(),
                routed.len()
            )));
        }
        for (receipt, routed_receipt) in receipts.iter().zip(routed.iter()) {
            if let Some(message) = routed_receipt.drift_from_mutation_receipt(receipt) {
                return Err(WorthQueryWorkspaceError::new(message));
            }
        }
        Ok(receipts)
    }

    fn admit_existing_truth_binding(
        &self,
        _binding: &WorthQueryExistingTruthTargetBinding,
    ) -> Result<(), WorthQueryExistingTruthBindingDenial> {
        Ok(())
    }

    fn verify_existing_truth_assertion(
        &self,
        binding: &WorthQueryExistingTruthTargetBinding,
        aspects: &[crate::runtime::WorthQueryAdmittedAspectValue],
    ) -> Result<WorthQueryVerifiedExistingTruthAssertion, WorthQueryExistingTruthAssertionDenial>
    {
        let Some(adapter) = self.existing_truth_verification.as_ref() else {
            return Err(WorthQueryExistingTruthAssertionDenial::new(
                binding,
                crate::runtime::WorthQueryExistingTruthAssertionDenialKind::BackendVerificationUnsupported,
                None,
                None,
                None,
                "this runtime backend does not admit backend-verified existing-truth assertions yet",
            ));
        };
        adapter.verify_existing_truth_assertion(binding, aspects)?;
        WorthQueryVerifiedExistingTruthAssertion::from_snapshot_identity(
            binding,
            aspects,
            &self.current_snapshot_identity(),
        )
        .map_err(|error| {
            WorthQueryExistingTruthAssertionDenial::new(
                binding,
                crate::runtime::WorthQueryExistingTruthAssertionDenialKind::MissingAssertedAspect,
                None,
                None,
                None,
                error.to_string(),
            )
        })
    }

    fn probe_existing_truth(
        &self,
        request: &crate::runtime::WorthQueryExistingTruthProbeRequest,
    ) -> Result<WorthQueryExistingTruthProbe, WorthQueryExistingTruthProbeDenial> {
        let Some(adapter) = self.existing_truth_verification.as_ref() else {
            return Err(WorthQueryExistingTruthProbeDenial::new(
                request.binding(),
                crate::runtime::WorthQueryExistingTruthProbeDenialKind::BackendProbeUnsupported,
                None,
                "this runtime backend does not admit backend-verified existing-truth probes yet",
            ));
        };
        WorthQueryExistingTruthProbe::backend_verified(
            request,
            adapter.probe_existing_truth(request)?,
        )
    }

    fn execute_intent(
        &mut self,
        declaration: &WorthQueryIntentDeclaration,
    ) -> Result<WorthQueryIntentExecution, WorthQueryRuntimeError> {
        let Some(intent_authority) = self.intent_authority.as_mut() else {
            return Err(WorthQueryRuntimeError::MissingIntentAuthority);
        };
        let execution = intent_authority
            .execute_intent(
                &self.runtime_bridge,
                self.relational_runtime.as_mut(),
                declaration,
            )
            .map_err(WorthQueryRuntimeError::Workspace)?;
        Ok(execution)
    }

    fn admit_query_writeback_authority(&self) -> Result<(), WorthQueryWorkspaceError> {
        if self.runtime_bridge.writeback_authority().is_some() {
            Ok(())
        } else {
            Err(WorthQueryWorkspaceError::new(
                "bridge-backed runtime has no configured truth writeback authority",
            ))
        }
    }

    fn execute_query_writeback(
        &mut self,
        declaration: &crate::workflow::QueryWritebackDeclaration,
    ) -> Result<
        worth_runtime_bridge::facade::BridgeAdmittedWritebackExecution,
        (crate::effect_lifecycle::EffectExecutionDenialKind, String),
    > {
        crate::effect_lifecycle::execute_lowered_writeback(&self.runtime_bridge, declaration)
    }

    fn capture_query_merge_authority(
        &self,
        target_branch: BranchId,
        source_branch: BranchId,
    ) -> Result<WorthQueryBackendMergeAuthority, WorthQueryWorkspaceError> {
        let runtime = self.relational_runtime.as_ref().ok_or_else(|| {
            WorthQueryWorkspaceError::new(
                "bridge-backed runtime has no configured relational merge authority",
            )
        })?;
        WorthQueryBackendMergeAuthority::capture(runtime, target_branch, source_branch)
    }

    fn validate_query_merge_authority(
        &self,
        authority: &WorthQueryBackendMergeAuthority,
    ) -> Result<(), WorthQueryWorkspaceError> {
        let runtime = self.relational_runtime.as_ref().ok_or_else(|| {
            WorthQueryWorkspaceError::new(
                "bridge-backed runtime has no configured relational merge authority",
            )
        })?;
        authority.validate_against(runtime)
    }

    fn execute_query_merge(
        &mut self,
        authority: &WorthQueryBackendMergeAuthority,
        declaration: &crate::workflow::LoweredMergeWorkflowDeclaration,
    ) -> Result<
        worth_relational::facade::transactions::MergeExecutionOutcome,
        (crate::effect_lifecycle::EffectExecutionDenialKind, String),
    > {
        let runtime = self.relational_runtime.as_mut().ok_or_else(|| {
            (
                crate::effect_lifecycle::EffectExecutionDenialKind::MissingRelationalAuthority,
                "bridge-backed runtime has no configured relational merge authority".to_string(),
            )
        })?;
        if declaration.merge_request().target_branch() != authority.target_branch()
            || declaration.merge_request().source_branch() != authority.source_branch()
        {
            return Err((
                crate::effect_lifecycle::EffectExecutionDenialKind::AuthorityOverrideRejected,
                "lowered merge request does not match the captured branch authority".to_string(),
            ));
        }
        crate::effect_lifecycle::execute_lowered_merge(runtime, declaration)
    }

    fn execute_query_causal_inspection(
        &self,
        plan: &crate::runtime::CausalInspectionPlan,
    ) -> Result<
        crate::runtime::QueryCausalInspectionArtifact,
        crate::runtime::WorthQueryBackendInspectionError,
    > {
        plan.materialize_with_bridge(&self.runtime_bridge)
            .map_err(Into::into)
    }

    fn live_entities_for_target(
        &self,
        target: &WorthQueryLiveArtifactTarget,
    ) -> Vec<WorthQueryEntity> {
        self.source_adapter.live_entities_for_target(target)
    }

    fn drain_live_patches_for_target(
        &mut self,
        target: &WorthQueryLiveArtifactTarget,
    ) -> Vec<WorthQueryLivePatch> {
        self.source_adapter.drain_live_patches_for_target(target)
    }

    fn affected_live_view_targets(
        &self,
        receipt: &WorthQueryMutationReceipt,
    ) -> Vec<WorthQueryLiveArtifactTarget> {
        self.source_adapter.affected_live_view_targets(receipt)
    }

    fn install_live_subscription(
        &mut self,
        view_name: &str,
        activation: &SubscriptionActivationInput,
    ) -> Result<super::SubscriptionActivationReceipt, WorthQueryWorkspaceError> {
        let activation_receipt = self
            .subscription_activation
            .admit_activation(view_name, activation)?;
        if let Some(message) = activation_receipt.drift_from_activation(view_name, activation) {
            return Err(WorthQueryWorkspaceError::new(message));
        }
        Ok(activation_receipt.activation_receipt().clone())
    }

    fn admit_preview_basis(
        &self,
        label: &WorthQuerySessionLabel,
        effect_policy: WorthQueryEffectPolicy,
        authority: &WorthQueryRuntimeEvidenceAuthority,
    ) -> Result<WorthQueryPreviewBasisAdmission, WorthQueryWorkspaceError> {
        self.preview_basis
            .admit_preview_basis(label, effect_policy, authority)
    }

    fn inspect_write_receipt(
        &self,
        receipt: &WorthQueryWriteReceipt,
        authority: &WorthQueryRuntimeEvidenceAuthority,
    ) -> Result<WorthQueryRuntimeInspectionEvidence, WorthQueryWorkspaceError> {
        self.inspector_evidence
            .inspect_write_receipt(receipt, authority)
    }

    fn declaration_initialization_metadata(
        &self,
        view: &crate::program::WorthQueryDerivedView,
    ) -> Result<crate::runtime::WorthQueryMutationMetadata, WorthQueryWorkspaceError> {
        match self.declaration_initialization.as_ref() {
            Some(adapter) => adapter.declaration_initialization_metadata(view),
            None => Ok(crate::runtime::WorthQueryMutationMetadata::default()),
        }
    }
}
