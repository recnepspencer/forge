use super::bridge::certification_bridge;
use crate::declarative_live::{DeclarativeLiveQueryRequest, DeclarativeLiveViewShape};
use crate::evidence_identity::ForgeQueryEvidenceIdentity;
use crate::facade::{
    runtime_subscription_support_evidence_identity, DeclarativeProjectionField,
    ForgeQueryAuthorityLane, ForgeQueryBasisAdmissionEvidenceRow, ForgeQueryEffectPolicy,
    ForgeQueryExistingTruthAssertionDenial, ForgeQueryExistingTruthAssertionDenialKind,
    ForgeQueryExistingTruthProbeDenial, ForgeQueryExistingTruthProbeDenialKind,
    ForgeQueryIntentAuthorityAdapter, ForgeQueryIntentDeclaration, ForgeQueryIntentExecution,
    ForgeQueryLiveViewHandle, ForgeQueryMutationDelta, ForgeQueryMutationKind,
    ForgeQueryMutationReceipt, ForgeQueryPreviewBasisAdmission, ForgeQueryRuntime,
    ForgeQueryRuntimeEvidenceAuthority, ForgeQueryRuntimeExistingTruthVerificationAdapter,
    ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimeFamilySupport,
    ForgeQueryRuntimeInspectionEvidence, ForgeQueryRuntimeInspectorEvidenceAdapter,
    ForgeQueryRuntimePreviewBasisAdapter, ForgeQueryRuntimeSchemaAdapter,
    ForgeQueryRuntimeSignalSinkAdapter, ForgeQueryRuntimeSnapshotIdentityAdapter,
    ForgeQueryRuntimeSourceAdapter, ForgeQueryRuntimeSubscriptionActivationAdapter,
    ForgeQueryRuntimeSupportProfile, ForgeQuerySessionLabel, ForgeQueryWorkspaceError,
    ForgeQueryWriteReceipt, LiveViewDeclarationAdmissionBoundaryReceipt, QuerySchemaView,
    SchemaFieldKind, SchemaFieldView, SignalInvalidationBoundaryReceipt,
    SubscriptionActivationBoundaryReceipt, SubscriptionActivationInput,
};
use crate::identity::hash_parts;
use crate::memory_workspace::ForgeQuerySnapshotIdentity;
use crate::memory_workspace::{ForgeQueryEntity, ForgeQueryLivePatch};
use crate::runtime::ForgeQueryVerifiedExistingTruthAssertion;
use forge_relational::facade::runtime::RelationalRuntime;
use forge_runtime_bridge::facade::RuntimeBridge;
use serde_json::Value;
use std::collections::BTreeMap;

use super::write_authority::CertificationWriteAuthority;
use super::{
    certification_commit_identity_for, certification_entity_identity,
    certification_snapshot_identity, certification_snapshot_identity_for,
};

pub(crate) fn certification_runtime() -> ForgeQueryRuntime {
    ForgeQueryRuntime::builder()
        .runtime_bridge(certification_bridge())
        .schema_adapter(CertificationSchemaAdapter)
        .source_adapter(CertificationSourceAdapter::default())
        .snapshot_identity(CertificationSnapshotIdentity)
        .existing_truth_verification(CertificationExistingTruthVerification)
        .write_authority(CertificationWriteAuthority)
        .signal_sink(CertificationSignalSink)
        .subscription_activation(CertificationSubscriptionActivation)
        .preview_basis(CertificationPreviewBasis)
        .inspector_evidence(CertificationInspectorEvidence)
        .intent_authority(CertificationIntentAuthority)
        .support_profile(certification_support_profile())
        .build_backend_from_parts()
        .build()
        .expect("certification runtime backend parts should build")
}

pub(crate) fn certification_runtime_with_invariant_violation_authority() -> ForgeQueryRuntime {
    ForgeQueryRuntime::builder()
        .runtime_bridge(certification_bridge())
        .schema_adapter(CertificationSchemaAdapter)
        .source_adapter(CertificationSourceAdapter::default())
        .snapshot_identity(CertificationSnapshotIdentity)
        .existing_truth_verification(CertificationExistingTruthVerification)
        .write_authority(CertificationWriteAuthority)
        .signal_sink(CertificationSignalSink)
        .subscription_activation(CertificationSubscriptionActivation)
        .preview_basis(CertificationPreviewBasis)
        .inspector_evidence(CertificationInspectorEvidence)
        .intent_authority(InvariantViolationCertificationIntentAuthority)
        .support_profile(certification_support_profile())
        .build_backend_from_parts()
        .build()
        .expect("certification runtime backend parts should build")
}

struct CertificationSnapshotIdentity;

impl ForgeQueryRuntimeSnapshotIdentityAdapter for CertificationSnapshotIdentity {
    fn current_snapshot_identity(&self) -> ForgeQuerySnapshotIdentity {
        certification_snapshot_identity("certification-runtime-current-snapshot")
    }
}

pub(super) fn certification_support_profile() -> ForgeQueryRuntimeSupportProfile {
    ForgeQueryRuntimeSupportProfile::bridge_backed(
        "certification-subscription-activation",
        "certification-preview-basis",
        "certification-inspector-evidence",
    )
    .with_family_support(ForgeQueryRuntimeFamilySupport::supported(
        ForgeQueryRuntimeFacadeFamily::Intent,
        [
            ForgeQueryAuthorityLane::AuthoritativeTruth,
            ForgeQueryAuthorityLane::BranchLocalTruth,
            ForgeQueryAuthorityLane::PreviewTruth,
        ],
        [],
        ["certification-intent-authority"],
    ))
    .with_bridge_backed_verification_support(
        "probe_existing",
        "direct_entity_identity",
        true,
        true,
        None,
    )
}

pub(crate) fn certification_task_live_request() -> DeclarativeLiveQueryRequest {
    DeclarativeLiveQueryRequest::new("Task", DeclarativeLiveViewShape::table())
        .project(DeclarativeProjectionField::new("identity", "id").delivered_as("identity.id"))
        .project(DeclarativeProjectionField::new("title", "value").delivered_as("title"))
        .order_by(DeclarativeProjectionField::new("title", "value"))
}

pub(crate) fn certification_task_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "certification-task",
        [
            SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
            SchemaFieldView::new("title", "value", SchemaFieldKind::String),
        ],
        [],
    )
}

struct CertificationSchemaAdapter;

impl ForgeQueryRuntimeSchemaAdapter for CertificationSchemaAdapter {
    fn admit_live_view(
        &self,
        name: &str,
        request: &DeclarativeLiveQueryRequest,
        _schema_view: &QuerySchemaView,
    ) -> Result<LiveViewDeclarationAdmissionBoundaryReceipt, ForgeQueryWorkspaceError> {
        let receipt = self.build_live_view_declaration_admission_receipt(name, request);
        Ok(self.build_live_view_declaration_boundary_receipt(name, request, receipt))
    }
}

#[derive(Default)]
struct CertificationSourceAdapter {
    live_views: BTreeMap<String, String>,
}

impl ForgeQueryRuntimeSourceAdapter for CertificationSourceAdapter {
    fn declare_live_view(
        &mut self,
        name: String,
        request: DeclarativeLiveQueryRequest,
        _schema_view: QuerySchemaView,
    ) -> Result<ForgeQueryLiveViewHandle, ForgeQueryWorkspaceError> {
        self.live_views
            .insert(name.clone(), request.target().to_string());
        Ok(ForgeQueryLiveViewHandle::new(name))
    }

    fn live_entities(&self, _view_name: &str) -> Vec<ForgeQueryEntity> {
        Vec::new()
    }

    fn drain_live_patches(&mut self, _view_name: &str) -> Vec<ForgeQueryLivePatch> {
        Vec::new()
    }

    fn affected_live_view_ids(&self, receipt: &ForgeQueryMutationReceipt) -> Vec<String> {
        let mut affected = receipt
            .deltas
            .iter()
            .flat_map(|delta| {
                self.live_views
                    .iter()
                    .filter(move |(_, collection)| *collection == &delta.collection)
                    .map(|(name, _)| name.clone())
            })
            .collect::<Vec<_>>();
        affected.sort();
        affected.dedup();
        affected
    }
}

struct CertificationExistingTruthVerification;

impl ForgeQueryRuntimeExistingTruthVerificationAdapter for CertificationExistingTruthVerification {
    fn verify_existing_truth_assertion(
        &self,
        binding: &crate::runtime::ForgeQueryExistingTruthTargetBinding,
        aspects: &[crate::runtime::ForgeQueryAspectValue],
    ) -> Result<ForgeQueryVerifiedExistingTruthAssertion, ForgeQueryExistingTruthAssertionDenial>
    {
        ForgeQueryVerifiedExistingTruthAssertion::new(
            binding,
            aspects,
            certification_snapshot_identity("certification-existing-truth-verification-snapshot"),
        )
        .map_err(|error| {
            ForgeQueryExistingTruthAssertionDenial::new(
                binding,
                ForgeQueryExistingTruthAssertionDenialKind::MissingAssertedAspect,
                None,
                None,
                None,
                error.to_string(),
            )
        })
    }

    fn probe_existing_truth(
        &self,
        request: &crate::runtime::ForgeQueryExistingTruthProbeRequest,
    ) -> Result<Vec<(String, Value)>, ForgeQueryExistingTruthProbeDenial> {
        let mut values = Vec::with_capacity(request.aspect_paths().len());
        for aspect_path in request.aspect_paths() {
            let value = match aspect_path.as_str() {
                "identity.id" => Value::String("task-1".to_string()),
                "title.value" => Value::String("Seed title".to_string()),
                other => {
                    return Err(ForgeQueryExistingTruthProbeDenial::new(
                        request.binding(),
                        ForgeQueryExistingTruthProbeDenialKind::MissingProbedAspect,
                        Some(other.to_string()),
                        "certification verification adapter does not expose that aspect",
                    ));
                }
            };
            values.push((aspect_path.clone(), value));
        }
        Ok(values)
    }
}
struct CertificationIntentAuthority;

impl ForgeQueryIntentAuthorityAdapter for CertificationIntentAuthority {
    fn execute_intent(
        &mut self,
        _bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut RelationalRuntime>,
        declaration: &ForgeQueryIntentDeclaration,
    ) -> Result<ForgeQueryIntentExecution, ForgeQueryWorkspaceError> {
        let collection = declaration
            .input()
            .get("collection")
            .and_then(Value::as_str)
            .unwrap_or("Task")
            .to_string();
        let commit_identity =
            certification_commit_identity_for("certification-intent-commit", &collection);
        let snapshot_identity =
            certification_snapshot_identity_for("certification-intent-snapshot", &collection);
        let mutation_receipt = ForgeQueryMutationReceipt::from_authoritative_parts(
            commit_identity,
            snapshot_identity,
            vec![ForgeQueryMutationDelta::new(
                collection,
                certification_entity_identity("certification-intent-entity-1"),
                ForgeQueryMutationKind::Updated,
                vec!["title.value".to_string()],
            )],
        );
        Ok(ForgeQueryIntentExecution::admitted(
            declaration.strategy_name(),
            declaration.strategy_version(),
            "certification-strategy-descriptor-digest",
            declaration.input_digest(),
            hash_parts(&[
                "certification-intent-produced-mutation".to_string(),
                mutation_receipt
                    .commit_identity
                    .evidence_identity()
                    .as_str()
                    .to_string(),
                mutation_receipt
                    .snapshot_identity
                    .evidence_identity()
                    .as_str()
                    .to_string(),
            ]),
            [
                "certification-relational-invariant:acyclic",
                "certification-relational-invariant:authority-lane",
            ],
            mutation_receipt,
        ))
    }
}

struct CertificationSignalSink;

impl ForgeQueryRuntimeSignalSinkAdapter for CertificationSignalSink {
    fn route_write_receipt(
        &mut self,
        receipt: &ForgeQueryMutationReceipt,
    ) -> Result<SignalInvalidationBoundaryReceipt, ForgeQueryWorkspaceError> {
        let routed = self.build_signal_invalidation_routing_receipt(receipt)?;
        self.build_signal_invalidation_boundary_receipt(receipt, routed)
    }
}

struct CertificationSubscriptionActivation;

impl ForgeQueryRuntimeSubscriptionActivationAdapter for CertificationSubscriptionActivation {
    fn support_evidence_identity(&self) -> ForgeQueryEvidenceIdentity {
        runtime_subscription_support_evidence_identity("certification-subscription-activation")
    }

    fn admit_activation(
        &mut self,
        view_name: &str,
        activation: &SubscriptionActivationInput,
    ) -> Result<SubscriptionActivationBoundaryReceipt, ForgeQueryWorkspaceError> {
        let receipt = self.build_subscription_activation_receipt(view_name, activation);
        Ok(self.build_subscription_activation_boundary_receipt(view_name, activation, receipt))
    }
}

struct CertificationPreviewBasis;

impl ForgeQueryRuntimePreviewBasisAdapter for CertificationPreviewBasis {
    fn admit_preview_basis(
        &self,
        label: &ForgeQuerySessionLabel,
        effect_policy: ForgeQueryEffectPolicy,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryPreviewBasisAdmission, ForgeQueryWorkspaceError> {
        Ok(ForgeQueryPreviewBasisAdmission::new(
            authority,
            label.clone(),
            effect_policy,
            ForgeQueryBasisAdmissionEvidenceRow::rows_from_values(["certification-preview-basis"]),
        ))
    }
}

struct CertificationInspectorEvidence;

impl ForgeQueryRuntimeInspectorEvidenceAdapter for CertificationInspectorEvidence {
    fn inspect_write_receipt(
        &self,
        receipt: &ForgeQueryWriteReceipt,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryRuntimeInspectionEvidence, ForgeQueryWorkspaceError> {
        Ok(ForgeQueryRuntimeInspectionEvidence::new(
            authority,
            "certification-write-receipt",
            receipt.authority_lane(),
            ["certification-inspector-evidence"],
        ))
    }
}

struct InvariantViolationCertificationIntentAuthority;

impl ForgeQueryIntentAuthorityAdapter for InvariantViolationCertificationIntentAuthority {
    fn execute_intent(
        &mut self,
        _bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut RelationalRuntime>,
        declaration: &ForgeQueryIntentDeclaration,
    ) -> Result<ForgeQueryIntentExecution, ForgeQueryWorkspaceError> {
        Ok(ForgeQueryIntentExecution::invariant_violation(
            declaration.strategy_name(),
            declaration.strategy_version(),
            "certification-strategy-descriptor-digest",
            declaration.input_digest(),
            hash_parts(&[
                "certification-invariant-violation".to_string(),
                declaration.name().to_string(),
            ]),
            [
                "certification-invariant:violated",
                "certification-invariant:authority-lane",
            ],
            certification_snapshot_identity("certification-invariant-snapshot"),
        ))
    }
}
