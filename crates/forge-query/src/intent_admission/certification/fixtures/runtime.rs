use std::collections::BTreeMap;

use forge_relational::facade::runtime::RelationalRuntime;
use forge_runtime_bridge::facade::RuntimeBridge;
use serde_json::Value;

use crate::declarative_live::{DeclarativeLiveQueryRequest, DeclarativeLiveViewShape};
use crate::facade::{
    DeclarativeProjectionField, ForgeQueryAuthorityLane, ForgeQueryEffectPolicy,
    ForgeQueryIntentAuthorityAdapter, ForgeQueryIntentDeclaration, ForgeQueryIntentExecution,
    ForgeQueryLiveViewHandle, ForgeQueryMutationDelta, ForgeQueryMutationKind,
    ForgeQueryMutationReceipt, ForgeQueryPreviewBasisAdmission, ForgeQueryRuntime,
    ForgeQueryRuntimeEvidenceAuthority, ForgeQueryRuntimeFacadeFamily,
    ForgeQueryRuntimeFamilySupport, ForgeQueryRuntimeInspectionEvidence,
    ForgeQueryRuntimeInspectorEvidenceAdapter, ForgeQueryRuntimePreviewBasisAdapter,
    ForgeQueryRuntimeSchemaAdapter, ForgeQueryRuntimeSignalSinkAdapter,
    ForgeQueryRuntimeSourceAdapter, ForgeQueryRuntimeSubscriptionActivationAdapter,
    ForgeQueryRuntimeSupportProfile, ForgeQueryRuntimeWriteAuthorityAdapter,
    ForgeQueryWorkspaceError, ForgeQueryWriteCommand, ForgeQueryWriteReceipt, QuerySchemaView,
    SchemaFieldKind, SchemaFieldView, SubscriptionActivationInput,
};
use crate::identity::hash_parts;
use crate::memory_workspace::{ForgeQueryEntity, ForgeQueryLivePatch};

use super::bridge::certification_bridge;

pub(in crate::intent_admission::certification) fn certification_runtime() -> ForgeQueryRuntime {
    ForgeQueryRuntime::builder()
        .runtime_bridge(certification_bridge())
        .schema_adapter(CertificationSchemaAdapter)
        .source_adapter(CertificationSourceAdapter::default())
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

pub(in crate::intent_admission::certification) fn certification_runtime_with_invariant_violation_authority(
) -> ForgeQueryRuntime {
    ForgeQueryRuntime::builder()
        .runtime_bridge(certification_bridge())
        .schema_adapter(CertificationSchemaAdapter)
        .source_adapter(CertificationSourceAdapter::default())
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
}

pub(in crate::intent_admission::certification) fn certification_task_live_request(
) -> DeclarativeLiveQueryRequest {
    DeclarativeLiveQueryRequest::new("Task", DeclarativeLiveViewShape::table())
        .project(DeclarativeProjectionField::new("identity", "id").delivered_as("identity.id"))
        .project(DeclarativeProjectionField::new("title", "value").delivered_as("title"))
        .order_by(DeclarativeProjectionField::new("title", "value"))
}

pub(in crate::intent_admission::certification) fn certification_task_schema() -> QuerySchemaView {
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
        _name: &str,
        _request: &DeclarativeLiveQueryRequest,
        _schema_view: &QuerySchemaView,
    ) -> Result<(), ForgeQueryWorkspaceError> {
        Ok(())
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

    fn snapshot_token(&self) -> String {
        "certification-external-snapshot".to_string()
    }
}

struct CertificationWriteAuthority;

impl ForgeQueryRuntimeWriteAuthorityAdapter for CertificationWriteAuthority {
    fn write(
        &mut self,
        _bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut RelationalRuntime>,
        command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        let (collection, aspect_paths) = match command {
            ForgeQueryWriteCommand::UpdateAspect { aspect_path, .. } => {
                ("Task".to_string(), vec![aspect_path])
            }
            ForgeQueryWriteCommand::UpdateAspects { aspects, .. } => (
                "Task".to_string(),
                aspects
                    .iter()
                    .map(|aspect| aspect.aspect_path().to_string())
                    .collect(),
            ),
            ForgeQueryWriteCommand::InsertAspects {
                collection,
                aspects,
                ..
            } => (
                collection,
                aspects
                    .iter()
                    .map(|aspect| aspect.aspect_path().to_string())
                    .collect(),
            ),
            ForgeQueryWriteCommand::UpdateExistingAspects {
                aspects, binding, ..
            }
            | ForgeQueryWriteCommand::VerifyThenUpdateExistingAspects {
                aspects, binding, ..
            }
            | ForgeQueryWriteCommand::AssertExistingAspects {
                aspects, binding, ..
            }
            | ForgeQueryWriteCommand::VerifyExistingAspects {
                aspects, binding, ..
            } => (
                binding.target_collection().unwrap_or("Task").to_string(),
                aspects
                    .iter()
                    .map(|aspect| aspect.aspect_path().to_string())
                    .collect(),
            ),
            ForgeQueryWriteCommand::VerifyThenDeleteExistingAspects {
                binding,
                touched_aspect_paths,
                ..
            }
            | ForgeQueryWriteCommand::DeleteExistingAspects {
                binding,
                touched_aspect_paths,
                ..
            } => (
                binding.target_collection().unwrap_or("Task").to_string(),
                touched_aspect_paths,
            ),
            ForgeQueryWriteCommand::UpdateSymbolicAspects {
                aspects, reference, ..
            } => (
                reference.target_collection().unwrap_or("Task").to_string(),
                aspects
                    .iter()
                    .map(|aspect| aspect.aspect_path().to_string())
                    .collect(),
            ),
            ForgeQueryWriteCommand::DeleteAspects {
                touched_aspect_paths,
                ..
            }
            | ForgeQueryWriteCommand::DeleteSymbolicAspects {
                touched_aspect_paths,
                ..
            } => ("Task".to_string(), touched_aspect_paths),
            ForgeQueryWriteCommand::Delete { .. } => ("Task".to_string(), Vec::new()),
        };
        Ok(ForgeQueryMutationReceipt {
            commit_identity: format!("certification-commit:{collection}"),
            snapshot_token: format!("certification-snapshot:{collection}"),
            deltas: vec![ForgeQueryMutationDelta {
                collection,
                entity_identity: "certification-entity-1".to_string(),
                kind: ForgeQueryMutationKind::Updated,
                aspect_paths,
            }],
            bridge_authority: None,
        })
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
        let mutation_receipt = ForgeQueryMutationReceipt {
            commit_identity: format!("certification-intent-commit:{collection}"),
            snapshot_token: format!("certification-intent-snapshot:{collection}"),
            deltas: vec![ForgeQueryMutationDelta {
                collection,
                entity_identity: "certification-intent-entity-1".to_string(),
                kind: ForgeQueryMutationKind::Updated,
                aspect_paths: vec!["title.value".to_string()],
            }],
            bridge_authority: None,
        };
        Ok(ForgeQueryIntentExecution::admitted(
            declaration.strategy_name(),
            declaration.strategy_version(),
            "certification-strategy-descriptor-digest",
            declaration.input_digest(),
            hash_parts(&[
                "certification-intent-produced-mutation".to_string(),
                mutation_receipt.commit_identity.clone(),
                mutation_receipt.snapshot_token.clone(),
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
        _receipt: &ForgeQueryMutationReceipt,
    ) -> Result<(), ForgeQueryWorkspaceError> {
        Ok(())
    }
}

struct CertificationSubscriptionActivation;

impl ForgeQueryRuntimeSubscriptionActivationAdapter for CertificationSubscriptionActivation {
    fn support_evidence(&self) -> String {
        "certification-subscription-activation".to_string()
    }

    fn admit_activation(
        &mut self,
        view_name: &str,
        activation: &SubscriptionActivationInput,
    ) -> Result<String, ForgeQueryWorkspaceError> {
        Ok(format!(
            "certification-subscription-activation:{view_name}:{}",
            activation.activation_digest()
        ))
    }
}

struct CertificationPreviewBasis;

impl ForgeQueryRuntimePreviewBasisAdapter for CertificationPreviewBasis {
    fn admit_preview_basis(
        &self,
        label: &str,
        effect_policy: ForgeQueryEffectPolicy,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryPreviewBasisAdmission, ForgeQueryWorkspaceError> {
        Ok(ForgeQueryPreviewBasisAdmission::new(
            authority,
            label,
            effect_policy,
            ["certification-preview-basis"],
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
            "certification-invariant-snapshot",
        ))
    }
}
