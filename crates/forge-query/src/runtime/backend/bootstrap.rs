use forge_relational::facade::runtime::RelationalRuntime;
use forge_runtime_bridge::facade::RuntimeBridge;

use crate::runtime::{ForgeQueryRuntimeError, ForgeQueryRuntimeSupportProfile};

use super::{
    ForgeQueryIntentAuthorityAdapter, ForgeQueryRuntimeBackendParts,
    ForgeQueryRuntimeDeclarationInitializationAdapter,
    ForgeQueryRuntimeExistingTruthVerificationAdapter, ForgeQueryRuntimeInspectorEvidenceAdapter,
    ForgeQueryRuntimePreviewBasisAdapter, ForgeQueryRuntimeSchemaAdapter,
    ForgeQueryRuntimeSignalSinkAdapter, ForgeQueryRuntimeSnapshotIdentityAdapter,
    ForgeQueryRuntimeSourceAdapter, ForgeQueryRuntimeSubscriptionActivationAdapter,
    ForgeQueryRuntimeWriteAuthorityAdapter,
};

pub(in crate::runtime) struct BridgeBackedRuntimeBootstrap {
    pub(super) relational_runtime: Option<RelationalRuntime>,
    pub(super) runtime_bridge: RuntimeBridge,
    pub(super) schema_adapter: Box<dyn ForgeQueryRuntimeSchemaAdapter>,
    pub(super) source_adapter: Box<dyn ForgeQueryRuntimeSourceAdapter>,
    pub(super) snapshot_identity: Option<Box<dyn ForgeQueryRuntimeSnapshotIdentityAdapter>>,
    pub(super) existing_truth_verification:
        Option<Box<dyn ForgeQueryRuntimeExistingTruthVerificationAdapter>>,
    pub(super) write_authority: Box<dyn ForgeQueryRuntimeWriteAuthorityAdapter>,
    pub(super) signal_sink: Box<dyn ForgeQueryRuntimeSignalSinkAdapter>,
    pub(super) subscription_activation: Box<dyn ForgeQueryRuntimeSubscriptionActivationAdapter>,
    pub(super) preview_basis: Box<dyn ForgeQueryRuntimePreviewBasisAdapter>,
    pub(super) inspector_evidence: Box<dyn ForgeQueryRuntimeInspectorEvidenceAdapter>,
    pub(super) declaration_initialization:
        Option<Box<dyn ForgeQueryRuntimeDeclarationInitializationAdapter>>,
    pub(super) intent_authority: Option<Box<dyn ForgeQueryIntentAuthorityAdapter>>,
    pub(super) support_profile: ForgeQueryRuntimeSupportProfile,
}

impl BridgeBackedRuntimeBootstrap {
    pub(super) fn lower_from_parts(
        parts: ForgeQueryRuntimeBackendParts,
    ) -> Result<Self, ForgeQueryRuntimeError> {
        let relational_runtime = parts.relational_runtime;
        let runtime_bridge = parts
            .runtime_bridge
            .ok_or(ForgeQueryRuntimeError::MissingRuntimeBridge)?;
        let schema_adapter = parts
            .schema_adapter
            .ok_or(ForgeQueryRuntimeError::MissingSchemaAdapter)?;
        let source_adapter = parts
            .source_adapter
            .ok_or(ForgeQueryRuntimeError::MissingSourceAdapter)?;
        let snapshot_identity = parts
            .snapshot_identity
            .ok_or(ForgeQueryRuntimeError::MissingSnapshotIdentityAdapter)?;
        let existing_truth_verification = parts.existing_truth_verification;
        let write_authority = parts
            .write_authority
            .ok_or(ForgeQueryRuntimeError::MissingWriteAuthority)?;
        let signal_sink = parts
            .signal_sink
            .ok_or(ForgeQueryRuntimeError::MissingSignalSink)?;
        let subscription_activation = parts
            .subscription_activation
            .ok_or(ForgeQueryRuntimeError::MissingSubscriptionActivation)?;
        let preview_basis = parts
            .preview_basis
            .ok_or(ForgeQueryRuntimeError::MissingPreviewBasis)?;
        let inspector_evidence = parts
            .inspector_evidence
            .ok_or(ForgeQueryRuntimeError::MissingInspectorEvidence)?;
        let declaration_initialization = parts.declaration_initialization;
        let intent_authority = parts.intent_authority;
        let support_profile = parts.support_profile.unwrap_or_else(|| {
            ForgeQueryRuntimeSupportProfile::bridge_backed(
                subscription_activation.support_evidence_for_reporting(),
                "preview-basis-admission",
                "inspector-evidence-adapter",
            )
        });

        support_profile
            .validate_backend_claims(intent_authority.is_some())
            .map_err(ForgeQueryRuntimeError::UnsupportedFacadeFamily)?;

        Ok(Self {
            relational_runtime,
            runtime_bridge,
            schema_adapter,
            source_adapter,
            snapshot_identity: Some(snapshot_identity),
            existing_truth_verification,
            write_authority,
            signal_sink,
            subscription_activation,
            preview_basis,
            inspector_evidence,
            declaration_initialization,
            intent_authority,
            support_profile,
        })
    }
}
