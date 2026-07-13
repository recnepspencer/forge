use super::{
    LiveViewDeclarationAdmissionBoundaryReceipt, LiveViewDeclarationAdmissionReceipt,
    SignalInvalidationBoundaryReceipt, SignalInvalidationRoutingReceipt,
    SubscriptionActivationBoundaryReceipt, SubscriptionActivationReceipt,
    WriteAuthorityExecutionReceipt,
};
use crate::declarative_live::DeclarativeLiveQueryRequest;
use crate::evidence_identity::{
    WorthQueryEvidenceIdentity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};
use crate::memory_workspace::{
    WorthQueryEntity, WorthQueryEntityIdentity, WorthQueryLivePatch, WorthQueryLiveViewHandle,
    WorthQueryMutationKind, WorthQueryMutationReceipt, WorthQuerySnapshotIdentity,
    WorthQueryWorkspaceError,
};
use crate::program::WorthQueryDerivedView;
use crate::schema_view::QuerySchemaView;
use crate::session_label::WorthQuerySessionLabel;
use crate::subscription::SubscriptionActivationInput;
use crate::view_shape_live::WorthQueryGroupedBaselineMember;
use worth_relational::facade::runtime::RelationalRuntime;
use worth_runtime_bridge::facade::{BridgeMutationAuthorityBundle, RuntimeBridge};

use crate::runtime::remask_posture::WorthQueryRuntimeRemaskProjection;
use crate::runtime::{
    WorthQueryBackendAdmissibleMutation, WorthQueryEffectPolicy,
    WorthQueryExistingTruthAssertionDenial, WorthQueryExistingTruthBindingDenial,
    WorthQueryExistingTruthProbe, WorthQueryExistingTruthProbeDenial,
    WorthQueryExistingTruthProbeField, WorthQueryExistingTruthProbeRequest,
    WorthQueryExistingTruthTargetBinding, WorthQueryIntentDeclaration, WorthQueryIntentExecution,
    WorthQueryLiveArtifactTarget, WorthQueryPreviewBasisAdmission, WorthQueryRuntimeError,
    WorthQueryRuntimeEvidenceAuthority, WorthQueryRuntimeInspectionEvidence,
    WorthQueryRuntimeSupportProfile, WorthQueryVerifiedExistingTruthAssertion,
    WorthQueryWriteCommand, WorthQueryWriteReceipt,
};

pub fn runtime_subscription_support_evidence_identity(
    support_label: &str,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::SubscriptionActivationReceipt)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "runtime_subscription_activation_support_evidence_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("support_label"), support_label)
        .seal()
}

pub trait WorthQueryRuntimeBackend {
    fn support_profile(&self) -> WorthQueryRuntimeSupportProfile;

    fn current_snapshot_identity(&self) -> WorthQuerySnapshotIdentity {
        super::unavailable_snapshot_identity()
    }

    fn admit_live_view_declaration(
        &self,
        name: &str,
        request: &DeclarativeLiveQueryRequest,
        schema_view: &QuerySchemaView,
    ) -> Result<LiveViewDeclarationAdmissionBoundaryReceipt, WorthQueryWorkspaceError>;

    fn declare_live_view(
        &mut self,
        name: String,
        request: DeclarativeLiveQueryRequest,
        schema_view: QuerySchemaView,
    ) -> Result<WorthQueryLiveViewHandle, WorthQueryWorkspaceError>;

    fn close_live_view(&mut self, name: &str) -> Result<(), WorthQueryWorkspaceError>;

    fn write(
        &mut self,
        mutation: WorthQueryBackendAdmissibleMutation,
    ) -> Result<WorthQueryMutationReceipt, WorthQueryWorkspaceError>;

    fn write_batch(
        &mut self,
        mutations: Vec<WorthQueryBackendAdmissibleMutation>,
    ) -> Result<Vec<WorthQueryMutationReceipt>, WorthQueryWorkspaceError>;

    fn admit_existing_truth_binding(
        &self,
        _binding: &WorthQueryExistingTruthTargetBinding,
    ) -> Result<(), WorthQueryExistingTruthBindingDenial> {
        Ok(())
    }

    fn verify_existing_truth_assertion(
        &self,
        binding: &WorthQueryExistingTruthTargetBinding,
        _aspects: &[crate::runtime::WorthQueryAdmittedAspectValue],
    ) -> Result<WorthQueryVerifiedExistingTruthAssertion, WorthQueryExistingTruthAssertionDenial>
    {
        Err(WorthQueryExistingTruthAssertionDenial::new(
            binding,
            crate::runtime::WorthQueryExistingTruthAssertionDenialKind::BackendVerificationUnsupported,
            None,
            None,
            None,
            "this runtime backend does not admit backend-verified existing-truth assertions yet",
        ))
    }

    fn probe_existing_truth(
        &self,
        request: &WorthQueryExistingTruthProbeRequest,
    ) -> Result<WorthQueryExistingTruthProbe, WorthQueryExistingTruthProbeDenial> {
        Err(WorthQueryExistingTruthProbeDenial::new(
            request.binding(),
            crate::runtime::WorthQueryExistingTruthProbeDenialKind::BackendProbeUnsupported,
            None,
            "this runtime backend does not admit backend-verified existing-truth probes yet",
        ))
    }

    fn execute_intent(
        &mut self,
        declaration: &WorthQueryIntentDeclaration,
    ) -> Result<WorthQueryIntentExecution, WorthQueryRuntimeError>;

    fn live_entities_for_target(
        &self,
        target: &WorthQueryLiveArtifactTarget,
    ) -> Vec<WorthQueryEntity>;

    fn drain_live_patches_for_target(
        &mut self,
        target: &WorthQueryLiveArtifactTarget,
    ) -> Vec<WorthQueryLivePatch>;

    fn affected_live_view_targets(
        &self,
        receipt: &WorthQueryMutationReceipt,
    ) -> Vec<WorthQueryLiveArtifactTarget>;

    fn install_live_subscription(
        &mut self,
        view_name: &str,
        activation: &SubscriptionActivationInput,
    ) -> Result<SubscriptionActivationReceipt, WorthQueryWorkspaceError>;

    fn admit_preview_basis(
        &self,
        label: &WorthQuerySessionLabel,
        effect_policy: WorthQueryEffectPolicy,
        authority: &WorthQueryRuntimeEvidenceAuthority,
    ) -> Result<WorthQueryPreviewBasisAdmission, WorthQueryWorkspaceError>;

    fn inspect_write_receipt(
        &self,
        receipt: &WorthQueryWriteReceipt,
        authority: &WorthQueryRuntimeEvidenceAuthority,
    ) -> Result<WorthQueryRuntimeInspectionEvidence, WorthQueryWorkspaceError>;

    fn admit_preview_write_command(
        &self,
        _command: &WorthQueryWriteCommand,
    ) -> Result<(), WorthQueryWorkspaceError> {
        Ok(())
    }

    fn declaration_initialization_metadata(
        &self,
        _view: &WorthQueryDerivedView,
    ) -> Result<crate::runtime::WorthQueryMutationMetadata, WorthQueryWorkspaceError> {
        Ok(crate::runtime::WorthQueryMutationMetadata::default())
    }

    fn grouped_baseline_members(
        &self,
        _request: &DeclarativeLiveQueryRequest,
    ) -> Result<Option<Vec<WorthQueryGroupedBaselineMember>>, WorthQueryWorkspaceError> {
        Ok(None)
    }
}

pub trait WorthQueryRuntimeSchemaAdapter {
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
    ) -> Result<LiveViewDeclarationAdmissionBoundaryReceipt, WorthQueryWorkspaceError>;
}

pub trait WorthQueryRuntimeSourceAdapter {
    fn declare_live_view(
        &mut self,
        name: String,
        request: DeclarativeLiveQueryRequest,
        schema_view: QuerySchemaView,
    ) -> Result<WorthQueryLiveViewHandle, WorthQueryWorkspaceError>;

    fn close_live_view(&mut self, name: &str) -> Result<(), WorthQueryWorkspaceError>;

    fn live_entities_for_target(
        &self,
        target: &WorthQueryLiveArtifactTarget,
    ) -> Vec<WorthQueryEntity>;

    fn drain_live_patches_for_target(
        &mut self,
        target: &WorthQueryLiveArtifactTarget,
    ) -> Vec<WorthQueryLivePatch>;

    fn affected_live_view_targets(
        &self,
        receipt: &WorthQueryMutationReceipt,
    ) -> Vec<WorthQueryLiveArtifactTarget>;
}

pub trait WorthQueryRuntimeSnapshotIdentityAdapter {
    fn current_snapshot_identity(&self) -> WorthQuerySnapshotIdentity;
}

pub trait WorthQueryRuntimeExistingTruthVerificationAdapter {
    fn verify_existing_truth_assertion(
        &self,
        binding: &WorthQueryExistingTruthTargetBinding,
        aspects: &[crate::runtime::WorthQueryAdmittedAspectValue],
    ) -> Result<(), WorthQueryExistingTruthAssertionDenial>;

    fn probe_existing_truth(
        &self,
        request: &WorthQueryExistingTruthProbeRequest,
    ) -> Result<Vec<WorthQueryExistingTruthProbeField>, WorthQueryExistingTruthProbeDenial>;
}

pub trait WorthQueryRuntimeWriteAuthorityAdapter {
    fn build_bridge_mutation_authority_bundle(
        &self,
        bridge: &RuntimeBridge,
        snapshot_identity: &WorthQuerySnapshotIdentity,
        mutation: &WorthQueryBackendAdmissibleMutation,
        collection: &str,
        entity_identity: &WorthQueryEntityIdentity,
        mutation_kind: WorthQueryMutationKind,
    ) -> Result<BridgeMutationAuthorityBundle, WorthQueryWorkspaceError> {
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
        mutation: &WorthQueryBackendAdmissibleMutation,
        receipt: WorthQueryMutationReceipt,
    ) -> WriteAuthorityExecutionReceipt {
        WriteAuthorityExecutionReceipt::from_backend_admissible_mutation(mutation, receipt)
    }

    fn write(
        &mut self,
        bridge: &RuntimeBridge,
        relational_runtime: Option<&mut RelationalRuntime>,
        mutation: WorthQueryBackendAdmissibleMutation,
    ) -> Result<WriteAuthorityExecutionReceipt, WorthQueryWorkspaceError>;

    fn write_batch(
        &mut self,
        bridge: &RuntimeBridge,
        mut relational_runtime: Option<&mut RelationalRuntime>,
        mutations: Vec<WorthQueryBackendAdmissibleMutation>,
    ) -> Result<Vec<WriteAuthorityExecutionReceipt>, WorthQueryWorkspaceError> {
        let mut receipts = Vec::with_capacity(mutations.len());
        for mutation in mutations {
            receipts.push(self.write(bridge, relational_runtime.as_deref_mut(), mutation)?);
        }
        Ok(receipts)
    }
}

pub trait WorthQueryRuntimeSignalSinkAdapter {
    fn build_signal_invalidation_routing_receipt(
        &self,
        receipt: &WorthQueryMutationReceipt,
    ) -> Result<SignalInvalidationRoutingReceipt, WorthQueryWorkspaceError> {
        SignalInvalidationRoutingReceipt::from_mutation_receipt(receipt)
    }

    fn build_signal_invalidation_boundary_receipt(
        &self,
        receipt: &WorthQueryMutationReceipt,
        routing_receipt: SignalInvalidationRoutingReceipt,
    ) -> Result<SignalInvalidationBoundaryReceipt, WorthQueryWorkspaceError> {
        Ok(SignalInvalidationBoundaryReceipt::from_mutation_receipt(
            receipt,
            routing_receipt,
        ))
    }

    fn route_write_receipt(
        &mut self,
        receipt: &WorthQueryMutationReceipt,
    ) -> Result<SignalInvalidationBoundaryReceipt, WorthQueryWorkspaceError>;

    fn route_write_batch(
        &mut self,
        receipts: &[WorthQueryMutationReceipt],
    ) -> Result<Vec<SignalInvalidationBoundaryReceipt>, WorthQueryWorkspaceError> {
        let mut routed = Vec::with_capacity(receipts.len());
        for receipt in receipts {
            routed.push(self.route_write_receipt(receipt)?);
        }
        Ok(routed)
    }
}

pub trait WorthQueryRuntimeSubscriptionActivationAdapter {
    fn support_evidence_identity(&self) -> WorthQueryEvidenceIdentity;

    fn support_evidence_for_reporting(&self) -> String {
        self.support_evidence_identity().as_str().to_string()
    }

    fn remask_projection(
        &self,
        _view_name: &str,
        _activation: &SubscriptionActivationInput,
    ) -> Option<WorthQueryRuntimeRemaskProjection> {
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
    ) -> Result<SubscriptionActivationBoundaryReceipt, WorthQueryWorkspaceError>;
}

pub trait WorthQueryRuntimePreviewBasisAdapter {
    fn admit_preview_basis(
        &self,
        label: &WorthQuerySessionLabel,
        effect_policy: WorthQueryEffectPolicy,
        authority: &WorthQueryRuntimeEvidenceAuthority,
    ) -> Result<WorthQueryPreviewBasisAdmission, WorthQueryWorkspaceError>;
}

pub trait WorthQueryRuntimeInspectorEvidenceAdapter {
    fn inspect_write_receipt(
        &self,
        receipt: &WorthQueryWriteReceipt,
        authority: &WorthQueryRuntimeEvidenceAuthority,
    ) -> Result<WorthQueryRuntimeInspectionEvidence, WorthQueryWorkspaceError>;
}

pub trait WorthQueryRuntimeDeclarationInitializationAdapter {
    fn declaration_initialization_metadata(
        &self,
        view: &WorthQueryDerivedView,
    ) -> Result<crate::runtime::WorthQueryMutationMetadata, WorthQueryWorkspaceError>;
}
