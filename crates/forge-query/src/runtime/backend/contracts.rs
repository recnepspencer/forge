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
use crate::view_shape_live::ForgeQueryGroupedBaselineMember;
use forge_relational::facade::runtime::RelationalRuntime;
use forge_runtime_bridge::facade::{BridgeMutationAuthorityBundle, RuntimeBridge};

use crate::runtime::remask_posture::ForgeQueryRuntimeRemaskProjection;
use crate::runtime::{
    ForgeQueryBackendAdmissibleMutation, ForgeQueryEffectPolicy,
    ForgeQueryExistingTruthAssertionDenial, ForgeQueryExistingTruthBindingDenial,
    ForgeQueryExistingTruthProbe, ForgeQueryExistingTruthProbeDenial,
    ForgeQueryExistingTruthProbeField, ForgeQueryExistingTruthProbeRequest,
    ForgeQueryExistingTruthTargetBinding, ForgeQueryIntentDeclaration, ForgeQueryIntentExecution,
    ForgeQueryLiveArtifactTarget, ForgeQueryPreviewBasisAdmission, ForgeQueryRuntimeError,
    ForgeQueryRuntimeEvidenceAuthority, ForgeQueryRuntimeInspectionEvidence,
    ForgeQueryRuntimeSupportProfile, ForgeQueryVerifiedExistingTruthAssertion,
    ForgeQueryWriteCommand, ForgeQueryWriteReceipt,
};

pub fn runtime_subscription_support_evidence_identity(
    support_label: &str,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::SubscriptionActivationReceipt)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "runtime_subscription_activation_support_evidence_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("support_label"), support_label)
        .seal()
}

pub trait ForgeQueryRuntimeBackend {
    fn support_profile(&self) -> ForgeQueryRuntimeSupportProfile;

    fn current_snapshot_identity(&self) -> ForgeQuerySnapshotIdentity {
        super::unavailable_snapshot_identity()
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
        mutation: ForgeQueryBackendAdmissibleMutation,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError>;

    fn write_batch(
        &mut self,
        mutations: Vec<ForgeQueryBackendAdmissibleMutation>,
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
        _aspects: &[crate::runtime::ForgeQueryAdmittedAspectValue],
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

    fn live_entities_for_target(
        &self,
        target: &ForgeQueryLiveArtifactTarget,
    ) -> Vec<ForgeQueryEntity>;

    fn drain_live_patches_for_target(
        &mut self,
        target: &ForgeQueryLiveArtifactTarget,
    ) -> Vec<ForgeQueryLivePatch>;

    fn affected_live_view_targets(
        &self,
        receipt: &ForgeQueryMutationReceipt,
    ) -> Vec<ForgeQueryLiveArtifactTarget>;

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

    fn admit_preview_write_command(
        &self,
        _command: &ForgeQueryWriteCommand,
    ) -> Result<(), ForgeQueryWorkspaceError> {
        Ok(())
    }

    fn declaration_initialization_metadata(
        &self,
        _view: &ForgeQueryDerivedView,
    ) -> Result<crate::runtime::ForgeQueryMutationMetadata, ForgeQueryWorkspaceError> {
        Ok(crate::runtime::ForgeQueryMutationMetadata::default())
    }

    fn grouped_baseline_members(
        &self,
        _request: &DeclarativeLiveQueryRequest,
    ) -> Result<Option<Vec<ForgeQueryGroupedBaselineMember>>, ForgeQueryWorkspaceError> {
        Ok(None)
    }
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

    fn live_entities_for_target(
        &self,
        target: &ForgeQueryLiveArtifactTarget,
    ) -> Vec<ForgeQueryEntity>;

    fn drain_live_patches_for_target(
        &mut self,
        target: &ForgeQueryLiveArtifactTarget,
    ) -> Vec<ForgeQueryLivePatch>;

    fn affected_live_view_targets(
        &self,
        receipt: &ForgeQueryMutationReceipt,
    ) -> Vec<ForgeQueryLiveArtifactTarget>;
}

pub trait ForgeQueryRuntimeSnapshotIdentityAdapter {
    fn current_snapshot_identity(&self) -> ForgeQuerySnapshotIdentity;
}

pub trait ForgeQueryRuntimeExistingTruthVerificationAdapter {
    fn verify_existing_truth_assertion(
        &self,
        binding: &ForgeQueryExistingTruthTargetBinding,
        aspects: &[crate::runtime::ForgeQueryAdmittedAspectValue],
    ) -> Result<(), ForgeQueryExistingTruthAssertionDenial>;

    fn probe_existing_truth(
        &self,
        request: &ForgeQueryExistingTruthProbeRequest,
    ) -> Result<Vec<ForgeQueryExistingTruthProbeField>, ForgeQueryExistingTruthProbeDenial>;
}

pub trait ForgeQueryRuntimeWriteAuthorityAdapter {
    fn build_bridge_mutation_authority_bundle(
        &self,
        bridge: &RuntimeBridge,
        snapshot_identity: &ForgeQuerySnapshotIdentity,
        mutation: &ForgeQueryBackendAdmissibleMutation,
        collection: &str,
        entity_identity: &ForgeQueryEntityIdentity,
        mutation_kind: ForgeQueryMutationKind,
    ) -> Result<BridgeMutationAuthorityBundle, ForgeQueryWorkspaceError> {
        super::build_bridge_authority_bundle(
            bridge,
            snapshot_identity,
            mutation,
            collection,
            entity_identity,
            mutation_kind,
        )
    }

    fn build_write_authority_execution_receipt(
        &self,
        mutation: &ForgeQueryBackendAdmissibleMutation,
        receipt: ForgeQueryMutationReceipt,
    ) -> WriteAuthorityExecutionReceipt {
        WriteAuthorityExecutionReceipt::from_backend_admissible_mutation(mutation, receipt)
    }

    fn write(
        &mut self,
        bridge: &RuntimeBridge,
        relational_runtime: Option<&mut RelationalRuntime>,
        mutation: ForgeQueryBackendAdmissibleMutation,
    ) -> Result<WriteAuthorityExecutionReceipt, ForgeQueryWorkspaceError>;

    fn write_batch(
        &mut self,
        bridge: &RuntimeBridge,
        mut relational_runtime: Option<&mut RelationalRuntime>,
        mutations: Vec<ForgeQueryBackendAdmissibleMutation>,
    ) -> Result<Vec<WriteAuthorityExecutionReceipt>, ForgeQueryWorkspaceError> {
        let mut receipts = Vec::with_capacity(mutations.len());
        for mutation in mutations {
            receipts.push(self.write(bridge, relational_runtime.as_deref_mut(), mutation)?);
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
    fn support_evidence_identity(&self) -> ForgeQueryEvidenceIdentity;

    fn support_evidence_for_reporting(&self) -> String {
        self.support_evidence_identity().as_str().to_string()
    }

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
            self.support_evidence_identity(),
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
