use forge_relational::facade::runtime::RelationalRuntime;
use forge_runtime_bridge::facade::RuntimeBridge;

use crate::declarative_live::DeclarativeLiveQueryRequest;
use crate::memory_workspace::{
    ForgeQueryEntity, ForgeQueryLivePatch, ForgeQueryLiveViewHandle, ForgeQueryMutationReceipt,
    ForgeQuerySnapshotIdentity, ForgeQueryWorkspaceError,
};
use crate::schema_view::QuerySchemaView;
use crate::session_label::ForgeQuerySessionLabel;
use crate::subscription::SubscriptionActivationInput;

use crate::runtime::{
    ForgeQueryEffectPolicy, ForgeQueryExistingTruthAssertionDenial,
    ForgeQueryExistingTruthBindingDenial, ForgeQueryExistingTruthProbe,
    ForgeQueryExistingTruthProbeDenial, ForgeQueryExistingTruthTargetBinding,
    ForgeQueryIntentDeclaration, ForgeQueryIntentExecution, ForgeQueryPreviewBasisAdmission,
    ForgeQueryRuntimeBackend, ForgeQueryRuntimeError, ForgeQueryRuntimeEvidenceAuthority,
    ForgeQueryRuntimeInspectionEvidence, ForgeQueryRuntimeSupportProfile,
    ForgeQueryVerifiedExistingTruthAssertion, ForgeQueryWriteCommand, ForgeQueryWriteReceipt,
};

use super::bootstrap::BridgeBackedRuntimeBootstrap;

pub struct ForgeQueryBridgeBackedRuntimeBackend {
    relational_runtime: Option<RelationalRuntime>,
    runtime_bridge: RuntimeBridge,
    schema_adapter: Box<dyn super::ForgeQueryRuntimeSchemaAdapter>,
    source_adapter: Box<dyn super::ForgeQueryRuntimeSourceAdapter>,
    snapshot_identity: Option<Box<dyn super::ForgeQueryRuntimeSnapshotIdentityAdapter>>,
    existing_truth_verification:
        Option<Box<dyn super::ForgeQueryRuntimeExistingTruthVerificationAdapter>>,
    write_authority: Box<dyn super::ForgeQueryRuntimeWriteAuthorityAdapter>,
    signal_sink: Box<dyn super::ForgeQueryRuntimeSignalSinkAdapter>,
    subscription_activation: Box<dyn super::ForgeQueryRuntimeSubscriptionActivationAdapter>,
    preview_basis: Box<dyn super::ForgeQueryRuntimePreviewBasisAdapter>,
    inspector_evidence: Box<dyn super::ForgeQueryRuntimeInspectorEvidenceAdapter>,
    declaration_initialization:
        Option<Box<dyn super::ForgeQueryRuntimeDeclarationInitializationAdapter>>,
    intent_authority: Option<Box<dyn super::ForgeQueryIntentAuthorityAdapter>>,
    support_profile: ForgeQueryRuntimeSupportProfile,
}

impl ForgeQueryBridgeBackedRuntimeBackend {
    #[cfg(test)]
    pub(crate) fn from_parts(
        parts: super::ForgeQueryRuntimeBackendParts,
    ) -> Result<Self, ForgeQueryRuntimeError> {
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

impl ForgeQueryRuntimeBackend for ForgeQueryBridgeBackedRuntimeBackend {
    fn support_profile(&self) -> ForgeQueryRuntimeSupportProfile {
        self.support_profile.clone()
    }

    fn current_snapshot_identity(&self) -> ForgeQuerySnapshotIdentity {
        match self.snapshot_identity.as_ref() {
            Some(adapter) => adapter.current_snapshot_identity(),
            None => super::contracts::unavailable_snapshot_identity(),
        }
    }

    fn declare_live_view(
        &mut self,
        name: String,
        request: DeclarativeLiveQueryRequest,
        schema_view: QuerySchemaView,
    ) -> Result<ForgeQueryLiveViewHandle, ForgeQueryWorkspaceError> {
        self.source_adapter
            .declare_live_view(name, request, schema_view)
    }

    fn admit_live_view_declaration(
        &self,
        name: &str,
        request: &DeclarativeLiveQueryRequest,
        schema_view: &QuerySchemaView,
    ) -> Result<super::LiveViewDeclarationAdmissionBoundaryReceipt, ForgeQueryWorkspaceError> {
        self.schema_adapter
            .admit_live_view(name, request, schema_view)
    }

    fn write(
        &mut self,
        command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        let write_execution = self.write_authority.write(
            &self.runtime_bridge,
            self.relational_runtime.as_mut(),
            command.clone(),
        )?;
        if let Some(message) = write_execution.drift_from_command(&command) {
            return Err(ForgeQueryWorkspaceError::new(message));
        }
        let receipt = write_execution.mutation_receipt().clone();
        let routed = self.signal_sink.route_write_receipt(&receipt)?;
        if let Some(message) = routed.drift_from_mutation_receipt(&receipt) {
            return Err(ForgeQueryWorkspaceError::new(message));
        }
        Ok(receipt)
    }

    fn write_batch(
        &mut self,
        commands: Vec<ForgeQueryWriteCommand>,
    ) -> Result<Vec<ForgeQueryMutationReceipt>, ForgeQueryWorkspaceError> {
        let write_executions = self.write_authority.write_batch(
            &self.runtime_bridge,
            self.relational_runtime.as_mut(),
            commands.clone(),
        )?;
        for (command, execution) in commands.iter().zip(write_executions.iter()) {
            if let Some(message) = execution.drift_from_command(command) {
                return Err(ForgeQueryWorkspaceError::new(message));
            }
        }
        let receipts = write_executions
            .iter()
            .map(|execution| execution.mutation_receipt().clone())
            .collect::<Vec<_>>();
        let routed = self.signal_sink.route_write_batch(&receipts)?;
        if routed.len() != receipts.len() {
            return Err(ForgeQueryWorkspaceError::new(format!(
                "signal invalidation routing batch width drifted from write batch: expected `{}`, found `{}`",
                receipts.len(),
                routed.len()
            )));
        }
        for (receipt, routed_receipt) in receipts.iter().zip(routed.iter()) {
            if let Some(message) = routed_receipt.drift_from_mutation_receipt(receipt) {
                return Err(ForgeQueryWorkspaceError::new(message));
            }
        }
        Ok(receipts)
    }

    fn admit_existing_truth_binding(
        &self,
        _binding: &ForgeQueryExistingTruthTargetBinding,
    ) -> Result<(), ForgeQueryExistingTruthBindingDenial> {
        Ok(())
    }

    fn verify_existing_truth_assertion(
        &self,
        binding: &ForgeQueryExistingTruthTargetBinding,
        aspects: &[crate::runtime::ForgeQueryAspectValue],
    ) -> Result<ForgeQueryVerifiedExistingTruthAssertion, ForgeQueryExistingTruthAssertionDenial>
    {
        let Some(adapter) = self.existing_truth_verification.as_ref() else {
            return Err(ForgeQueryExistingTruthAssertionDenial::new(
                binding,
                crate::runtime::ForgeQueryExistingTruthAssertionDenialKind::BackendVerificationUnsupported,
                None,
                None,
                None,
                "this runtime backend does not admit backend-verified existing-truth assertions yet",
            ));
        };
        adapter.verify_existing_truth_assertion(binding, aspects)
    }

    fn probe_existing_truth(
        &self,
        request: &crate::runtime::ForgeQueryExistingTruthProbeRequest,
    ) -> Result<ForgeQueryExistingTruthProbe, ForgeQueryExistingTruthProbeDenial> {
        let Some(adapter) = self.existing_truth_verification.as_ref() else {
            return Err(ForgeQueryExistingTruthProbeDenial::new(
                request.binding(),
                crate::runtime::ForgeQueryExistingTruthProbeDenialKind::BackendProbeUnsupported,
                None,
                "this runtime backend does not admit backend-verified existing-truth probes yet",
            ));
        };
        Ok(ForgeQueryExistingTruthProbe::backend_verified(
            request,
            adapter.probe_existing_truth(request)?,
        ))
    }

    fn execute_intent(
        &mut self,
        declaration: &ForgeQueryIntentDeclaration,
    ) -> Result<ForgeQueryIntentExecution, ForgeQueryRuntimeError> {
        let Some(intent_authority) = self.intent_authority.as_mut() else {
            return Err(ForgeQueryRuntimeError::MissingIntentAuthority);
        };
        let execution = intent_authority
            .execute_intent(
                &self.runtime_bridge,
                self.relational_runtime.as_mut(),
                declaration,
            )
            .map_err(ForgeQueryRuntimeError::Workspace)?;
        Ok(execution)
    }

    fn live_entities(&self, view_name: &str) -> Vec<ForgeQueryEntity> {
        self.source_adapter.live_entities(view_name)
    }

    fn drain_live_patches(&mut self, view_name: &str) -> Vec<ForgeQueryLivePatch> {
        self.source_adapter.drain_live_patches(view_name)
    }

    fn affected_live_view_ids(&self, receipt: &ForgeQueryMutationReceipt) -> Vec<String> {
        self.source_adapter.affected_live_view_ids(receipt)
    }

    fn install_live_subscription(
        &mut self,
        view_name: &str,
        activation: &SubscriptionActivationInput,
    ) -> Result<super::SubscriptionActivationReceipt, ForgeQueryWorkspaceError> {
        let activation_receipt = self
            .subscription_activation
            .admit_activation(view_name, activation)?;
        if let Some(message) = activation_receipt.drift_from_activation(view_name, activation) {
            return Err(ForgeQueryWorkspaceError::new(message));
        }
        Ok(activation_receipt.activation_receipt().clone())
    }

    fn admit_preview_basis(
        &self,
        label: &ForgeQuerySessionLabel,
        effect_policy: ForgeQueryEffectPolicy,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryPreviewBasisAdmission, ForgeQueryWorkspaceError> {
        self.preview_basis
            .admit_preview_basis(label, effect_policy, authority)
    }

    fn inspect_write_receipt(
        &self,
        receipt: &ForgeQueryWriteReceipt,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryRuntimeInspectionEvidence, ForgeQueryWorkspaceError> {
        self.inspector_evidence
            .inspect_write_receipt(receipt, authority)
    }

    fn declaration_initialization_metadata(
        &self,
        view: &crate::program::ForgeQueryDerivedView,
    ) -> Result<crate::runtime::ForgeQueryMutationMetadata, ForgeQueryWorkspaceError> {
        match self.declaration_initialization.as_ref() {
            Some(adapter) => adapter.declaration_initialization_metadata(view),
            None => Ok(crate::runtime::ForgeQueryMutationMetadata::default()),
        }
    }
}
