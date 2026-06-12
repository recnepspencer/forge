use forge_relational::facade::runtime::RelationalRuntime;
use forge_runtime_bridge::facade::{BridgeMutationAuthorityBundle, RuntimeBridge};
use serde_json::Value;

use super::{
    LiveViewDeclarationAdmissionBoundaryReceipt, LiveViewDeclarationAdmissionReceipt,
    SignalInvalidationBoundaryReceipt, SignalInvalidationRoutingReceipt,
    SubscriptionActivationBoundaryReceipt, SubscriptionActivationReceipt,
    WriteAuthorityExecutionReceipt,
};
use crate::declarative_live::DeclarativeLiveQueryRequest;
use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::memory_workspace::{
    ForgeQueryEntity, ForgeQueryEntityIdentity, ForgeQueryLivePatch, ForgeQueryLiveViewHandle,
    ForgeQueryMutationKind, ForgeQueryMutationReceipt, ForgeQuerySnapshotIdentity,
    ForgeQueryWorkspaceError,
};
use crate::program::ForgeQueryDerivedView;
use crate::schema_view::QuerySchemaView;
use crate::session_label::ForgeQuerySessionLabel;
use crate::subscription::SubscriptionActivationInput;

use crate::runtime::remask_posture::ForgeQueryRuntimeRemaskProjection;
use crate::runtime::{
    ForgeQueryEffectPolicy, ForgeQueryExistingTruthAssertionDenial,
    ForgeQueryExistingTruthBindingDenial, ForgeQueryExistingTruthProbe,
    ForgeQueryExistingTruthProbeDenial, ForgeQueryExistingTruthProbeRequest,
    ForgeQueryExistingTruthTargetBinding, ForgeQueryIntentDeclaration, ForgeQueryIntentExecution,
    ForgeQueryPreviewBasisAdmission, ForgeQueryRuntimeError, ForgeQueryRuntimeEvidenceAuthority,
    ForgeQueryRuntimeInspectionEvidence, ForgeQueryRuntimeSupportProfile,
    ForgeQueryVerifiedExistingTruthAssertion, ForgeQueryWriteCommand, ForgeQueryWriteReceipt,
};

pub trait ForgeQueryRuntimeBackend {
    fn support_profile(&self) -> ForgeQueryRuntimeSupportProfile;

    fn current_snapshot_identity(&self) -> ForgeQuerySnapshotIdentity {
        unavailable_snapshot_identity()
    }

    fn admit_live_view_declaration(
        &self,
        name: &str,
        request: &DeclarativeLiveQueryRequest,
        schema_view: &QuerySchemaView,
    ) -> Result<LiveViewDeclarationAdmissionBoundaryReceipt, ForgeQueryWorkspaceError>;

    fn declare_live_view(
        &mut self,
        name: String,
        request: DeclarativeLiveQueryRequest,
        schema_view: QuerySchemaView,
    ) -> Result<ForgeQueryLiveViewHandle, ForgeQueryWorkspaceError>;

    fn write(
        &mut self,
        command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError>;

    fn write_batch(
        &mut self,
        commands: Vec<ForgeQueryWriteCommand>,
    ) -> Result<Vec<ForgeQueryMutationReceipt>, ForgeQueryWorkspaceError>;

    fn admit_existing_truth_binding(
        &self,
        _binding: &ForgeQueryExistingTruthTargetBinding,
    ) -> Result<(), ForgeQueryExistingTruthBindingDenial> {
        Ok(())
    }

    fn verify_existing_truth_assertion(
        &self,
        binding: &ForgeQueryExistingTruthTargetBinding,
        _aspects: &[crate::runtime::ForgeQueryAspectValue],
    ) -> Result<ForgeQueryVerifiedExistingTruthAssertion, ForgeQueryExistingTruthAssertionDenial>
    {
        Err(ForgeQueryExistingTruthAssertionDenial::new(
            binding,
            crate::runtime::ForgeQueryExistingTruthAssertionDenialKind::BackendVerificationUnsupported,
            None,
            None,
            None,
            "this runtime backend does not admit backend-verified existing-truth assertions yet",
        ))
    }

    fn probe_existing_truth(
        &self,
        request: &ForgeQueryExistingTruthProbeRequest,
    ) -> Result<ForgeQueryExistingTruthProbe, ForgeQueryExistingTruthProbeDenial> {
        Err(ForgeQueryExistingTruthProbeDenial::new(
            request.binding(),
            crate::runtime::ForgeQueryExistingTruthProbeDenialKind::BackendProbeUnsupported,
            None,
            "this runtime backend does not admit backend-verified existing-truth probes yet",
        ))
    }

    fn execute_intent(
        &mut self,
        declaration: &ForgeQueryIntentDeclaration,
    ) -> Result<ForgeQueryIntentExecution, ForgeQueryRuntimeError>;

    fn live_entities(&self, view_name: &str) -> Vec<ForgeQueryEntity>;

    fn drain_live_patches(&mut self, view_name: &str) -> Vec<ForgeQueryLivePatch>;

    fn affected_live_view_ids(&self, receipt: &ForgeQueryMutationReceipt) -> Vec<String>;

    fn install_live_subscription(
        &mut self,
        view_name: &str,
        activation: &SubscriptionActivationInput,
    ) -> Result<SubscriptionActivationReceipt, ForgeQueryWorkspaceError>;

    fn admit_preview_basis(
        &self,
        label: &ForgeQuerySessionLabel,
        effect_policy: ForgeQueryEffectPolicy,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryPreviewBasisAdmission, ForgeQueryWorkspaceError>;

    fn inspect_write_receipt(
        &self,
        receipt: &ForgeQueryWriteReceipt,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryRuntimeInspectionEvidence, ForgeQueryWorkspaceError>;

    fn declaration_initialization_metadata(
        &self,
        _view: &ForgeQueryDerivedView,
    ) -> Result<crate::runtime::ForgeQueryMutationMetadata, ForgeQueryWorkspaceError> {
        Ok(crate::runtime::ForgeQueryMutationMetadata::default())
    }

    fn grouped_baseline_members(
        &self,
        _request: &DeclarativeLiveQueryRequest,
    ) -> Result<Option<Vec<(String, String)>>, ForgeQueryWorkspaceError> {
        Ok(None)
    }
}

pub(in crate::runtime) fn unavailable_snapshot_identity() -> ForgeQuerySnapshotIdentity {
    ForgeQuerySnapshotIdentity::preview(
        ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::RuntimeStateSnapshot)
            .field_shape(
                ForgeQueryEvidenceTag::new("snapshot_authority"),
                "unavailable",
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("snapshot_contract"),
                "backend-must-override-for-authoritative-truth",
            )
            .seal(),
    )
}

pub trait ForgeQueryRuntimeSchemaAdapter {
    fn build_live_view_declaration_admission_receipt(
        &self,
        name: &str,
        request: &DeclarativeLiveQueryRequest,
    ) -> LiveViewDeclarationAdmissionReceipt {
        LiveViewDeclarationAdmissionReceipt::from_request(name, request)
    }

    fn build_live_view_declaration_boundary_receipt(
        &self,
        name: &str,
        request: &DeclarativeLiveQueryRequest,
        admission_receipt: LiveViewDeclarationAdmissionReceipt,
    ) -> LiveViewDeclarationAdmissionBoundaryReceipt {
        LiveViewDeclarationAdmissionBoundaryReceipt::from_request(name, request, admission_receipt)
    }

    fn admit_live_view(
        &self,
        name: &str,
        request: &DeclarativeLiveQueryRequest,
        schema_view: &QuerySchemaView,
    ) -> Result<LiveViewDeclarationAdmissionBoundaryReceipt, ForgeQueryWorkspaceError>;
}

pub trait ForgeQueryRuntimeSourceAdapter {
    fn declare_live_view(
        &mut self,
        name: String,
        request: DeclarativeLiveQueryRequest,
        schema_view: QuerySchemaView,
    ) -> Result<ForgeQueryLiveViewHandle, ForgeQueryWorkspaceError>;

    fn live_entities(&self, view_name: &str) -> Vec<ForgeQueryEntity>;

    fn drain_live_patches(&mut self, view_name: &str) -> Vec<ForgeQueryLivePatch>;

    fn affected_live_view_ids(&self, receipt: &ForgeQueryMutationReceipt) -> Vec<String>;
}

pub trait ForgeQueryRuntimeSnapshotIdentityAdapter {
    fn current_snapshot_identity(&self) -> ForgeQuerySnapshotIdentity;
}

pub trait ForgeQueryRuntimeExistingTruthVerificationAdapter {
    fn verify_existing_truth_assertion(
        &self,
        binding: &ForgeQueryExistingTruthTargetBinding,
        aspects: &[crate::runtime::ForgeQueryAspectValue],
    ) -> Result<ForgeQueryVerifiedExistingTruthAssertion, ForgeQueryExistingTruthAssertionDenial>;

    fn probe_existing_truth(
        &self,
        request: &ForgeQueryExistingTruthProbeRequest,
    ) -> Result<Vec<(String, Value)>, ForgeQueryExistingTruthProbeDenial>;
}

pub trait ForgeQueryRuntimeWriteAuthorityAdapter {
    fn build_bridge_mutation_authority_bundle(
        &self,
        bridge: &RuntimeBridge,
        snapshot_identity: &ForgeQuerySnapshotIdentity,
        command: &ForgeQueryWriteCommand,
        collection: &str,
        entity_identity: &ForgeQueryEntityIdentity,
        mutation_kind: ForgeQueryMutationKind,
    ) -> Result<BridgeMutationAuthorityBundle, ForgeQueryWorkspaceError> {
        super::build_bridge_authority_bundle(
            bridge,
            snapshot_identity,
            command,
            collection,
            entity_identity,
            mutation_kind,
        )
    }

    fn build_write_authority_execution_receipt(
        &self,
        command: &ForgeQueryWriteCommand,
        receipt: ForgeQueryMutationReceipt,
    ) -> WriteAuthorityExecutionReceipt {
        WriteAuthorityExecutionReceipt::from_command(command, receipt)
    }

    fn write(
        &mut self,
        bridge: &RuntimeBridge,
        relational_runtime: Option<&mut RelationalRuntime>,
        command: ForgeQueryWriteCommand,
    ) -> Result<WriteAuthorityExecutionReceipt, ForgeQueryWorkspaceError>;

    fn write_batch(
        &mut self,
        bridge: &RuntimeBridge,
        mut relational_runtime: Option<&mut RelationalRuntime>,
        commands: Vec<ForgeQueryWriteCommand>,
    ) -> Result<Vec<WriteAuthorityExecutionReceipt>, ForgeQueryWorkspaceError> {
        let mut receipts = Vec::with_capacity(commands.len());
        for command in commands {
            receipts.push(self.write(bridge, relational_runtime.as_deref_mut(), command)?);
        }
        Ok(receipts)
    }
}

pub trait ForgeQueryRuntimeSignalSinkAdapter {
    fn build_signal_invalidation_routing_receipt(
        &self,
        receipt: &ForgeQueryMutationReceipt,
    ) -> Result<SignalInvalidationRoutingReceipt, ForgeQueryWorkspaceError> {
        SignalInvalidationRoutingReceipt::from_mutation_receipt(receipt)
    }

    fn build_signal_invalidation_boundary_receipt(
        &self,
        receipt: &ForgeQueryMutationReceipt,
        routing_receipt: SignalInvalidationRoutingReceipt,
    ) -> Result<SignalInvalidationBoundaryReceipt, ForgeQueryWorkspaceError> {
        Ok(SignalInvalidationBoundaryReceipt::from_mutation_receipt(
            receipt,
            routing_receipt,
        ))
    }

    fn route_write_receipt(
        &mut self,
        receipt: &ForgeQueryMutationReceipt,
    ) -> Result<SignalInvalidationBoundaryReceipt, ForgeQueryWorkspaceError>;

    fn route_write_batch(
        &mut self,
        receipts: &[ForgeQueryMutationReceipt],
    ) -> Result<Vec<SignalInvalidationBoundaryReceipt>, ForgeQueryWorkspaceError> {
        let mut routed = Vec::with_capacity(receipts.len());
        for receipt in receipts {
            routed.push(self.route_write_receipt(receipt)?);
        }
        Ok(routed)
    }
}

pub trait ForgeQueryRuntimeSubscriptionActivationAdapter {
    fn support_evidence(&self) -> String;

    fn remask_projection(
        &self,
        _view_name: &str,
        _activation: &SubscriptionActivationInput,
    ) -> Option<ForgeQueryRuntimeRemaskProjection> {
        None
    }

    fn build_subscription_activation_receipt(
        &self,
        view_name: &str,
        activation: &SubscriptionActivationInput,
    ) -> SubscriptionActivationReceipt {
        SubscriptionActivationReceipt::from_activation(
            view_name,
            activation,
            self.support_evidence(),
            self.remask_projection(view_name, activation),
        )
    }

    fn build_subscription_activation_boundary_receipt(
        &self,
        view_name: &str,
        activation: &SubscriptionActivationInput,
        activation_receipt: SubscriptionActivationReceipt,
    ) -> SubscriptionActivationBoundaryReceipt {
        SubscriptionActivationBoundaryReceipt::from_activation(
            view_name,
            activation,
            activation_receipt,
        )
    }

    fn admit_activation(
        &mut self,
        view_name: &str,
        activation: &SubscriptionActivationInput,
    ) -> Result<SubscriptionActivationBoundaryReceipt, ForgeQueryWorkspaceError>;
}

pub trait ForgeQueryRuntimePreviewBasisAdapter {
    fn admit_preview_basis(
        &self,
        label: &ForgeQuerySessionLabel,
        effect_policy: ForgeQueryEffectPolicy,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryPreviewBasisAdmission, ForgeQueryWorkspaceError>;
}

pub trait ForgeQueryRuntimeInspectorEvidenceAdapter {
    fn inspect_write_receipt(
        &self,
        receipt: &ForgeQueryWriteReceipt,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryRuntimeInspectionEvidence, ForgeQueryWorkspaceError>;
}

pub trait ForgeQueryRuntimeDeclarationInitializationAdapter {
    fn declaration_initialization_metadata(
        &self,
        view: &ForgeQueryDerivedView,
    ) -> Result<crate::runtime::ForgeQueryMutationMetadata, ForgeQueryWorkspaceError>;
}
