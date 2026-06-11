use crate::declarative_live::{
    DeclarativeLiveQueryRequest, DeclarativeLiveViewShape, DeclarativeProjectionField,
};
use crate::identity::hash_parts;
use crate::lower_runtime_routing::{
    ForgeQueryLowerRuntimeAuthorityOwner, ForgeQueryLowerRuntimeBoundaryEnvelope,
    ForgeQueryLowerRuntimeBoundaryExecutionReceipt, ForgeQueryLowerRuntimeCapabilityEligibility,
    ForgeQueryLowerRuntimeCapabilityRequest, ForgeQueryLowerRuntimeCrossingRow,
    ForgeQueryLowerRuntimeReadmissionReceipt, ForgeQueryLowerRuntimeRouteKind,
    ForgeQueryLowerRuntimeRoutePlan, ForgeQueryLowerRuntimeSeamKey,
    LiveViewDeclarationAdmissionBoundaryReceipt, SignalInvalidationBoundaryReceipt,
    WriteAuthorityExecutionReceipt,
};
use crate::memory_workspace::{
    ForgeQueryEntity, ForgeQueryLivePatch, ForgeQueryLiveViewHandle, ForgeQueryMutationDelta,
    ForgeQueryMutationKind, ForgeQueryMutationReceipt, ForgeQueryWorkspaceError,
};
use crate::runtime::{
    ForgeQueryEffectPolicy, ForgeQueryPreviewBasisAdmission, ForgeQueryRuntimeEvidenceAuthority,
    ForgeQueryRuntimeSourceAdapter, ForgeQueryWriteCommand, LiveViewDeclarationAdmissionReceipt,
    SignalInvalidationRoutingReceipt,
};
use crate::schema_view::{QuerySchemaView, SchemaFieldKind, SchemaFieldView};
use crate::session_label::ForgeQuerySessionLabel;

use super::{ForgeQueryLowerRuntimeRepresentativeEvidenceSource, RepresentativeArtifacts};

pub(crate) fn representative_live_view_schema_row() -> RepresentativeArtifacts {
    let request = DeclarativeLiveQueryRequest::new("Task", DeclarativeLiveViewShape::table())
        .project(DeclarativeProjectionField::new("title", "value"))
        .project(DeclarativeProjectionField::new("status", "value"));
    let admission_receipt =
        LiveViewDeclarationAdmissionReceipt::from_request("tasks.table", &request);
    let boundary_receipt = LiveViewDeclarationAdmissionBoundaryReceipt::from_request(
        "tasks.table",
        &request,
        admission_receipt,
    );
    RepresentativeArtifacts {
        seam_key: ForgeQueryLowerRuntimeSeamKey::LiveViewSchemaAdmission,
        request: boundary_receipt
            .readmission_receipt()
            .eligibility()
            .request()
            .clone(),
        eligibility: boundary_receipt.readmission_receipt().eligibility().clone(),
        route_plan: None,
        boundary_receipt: boundary_receipt.boundary_execution_receipt().clone(),
        envelope: boundary_receipt.boundary_envelope().clone(),
        evidence_source: ForgeQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture,
    }
}

pub(crate) fn representative_write_authority_row() -> RepresentativeArtifacts {
    let command = ForgeQueryWriteCommand::Delete {
        entity_identity: "task-7".to_string(),
    };
    let mutation_receipt = ForgeQueryMutationReceipt {
        commit_identity: "commit-route-write-7".to_string(),
        snapshot_token: "snapshot-route-write-7".to_string(),
        deltas: vec![ForgeQueryMutationDelta {
            collection: "Task".to_string(),
            entity_identity: "task-7".to_string(),
            kind: ForgeQueryMutationKind::Deleted,
            aspect_paths: vec!["status.value".to_string()],
        }],
        bridge_authority: None,
    };
    let execution = WriteAuthorityExecutionReceipt::from_command(&command, mutation_receipt);
    RepresentativeArtifacts {
        seam_key: ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
        request: execution.route_plan().eligibility().request().clone(),
        eligibility: execution.route_plan().eligibility().clone(),
        route_plan: Some(execution.route_plan().clone()),
        boundary_receipt: execution.boundary_execution_receipt().clone(),
        envelope: execution.boundary_envelope().clone(),
        evidence_source: ForgeQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture,
    }
}

pub(crate) fn representative_signal_invalidation_row() -> RepresentativeArtifacts {
    let mutation_receipt = ForgeQueryMutationReceipt {
        commit_identity: "commit-route-signal-9".to_string(),
        snapshot_token: "snapshot-route-signal-9".to_string(),
        deltas: vec![
            ForgeQueryMutationDelta {
                collection: "Task".to_string(),
                entity_identity: "task-9".to_string(),
                kind: ForgeQueryMutationKind::Updated,
                aspect_paths: vec!["status.value".to_string()],
            },
            ForgeQueryMutationDelta {
                collection: "Task".to_string(),
                entity_identity: "task-10".to_string(),
                kind: ForgeQueryMutationKind::Updated,
                aspect_paths: vec!["priority.value".to_string()],
            },
        ],
        bridge_authority: None,
    };
    let routing = SignalInvalidationRoutingReceipt::from_mutation_receipt(&mutation_receipt);
    let boundary_receipt =
        SignalInvalidationBoundaryReceipt::from_mutation_receipt(&mutation_receipt, routing);
    RepresentativeArtifacts {
        seam_key: ForgeQueryLowerRuntimeSeamKey::SignalInvalidationRouting,
        request: boundary_receipt
            .route_plan()
            .eligibility()
            .request()
            .clone(),
        eligibility: boundary_receipt.route_plan().eligibility().clone(),
        route_plan: Some(boundary_receipt.route_plan().clone()),
        boundary_receipt: boundary_receipt.boundary_execution_receipt().clone(),
        envelope: boundary_receipt.boundary_envelope().clone(),
        evidence_source: ForgeQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture,
    }
}

pub(crate) fn representative_live_view_source_row() -> RepresentativeArtifacts {
    let request = DeclarativeLiveQueryRequest::new("Task", DeclarativeLiveViewShape::table())
        .project(DeclarativeProjectionField::new("identity", "id"))
        .project(DeclarativeProjectionField::new("title", "value"));
    let schema_view = QuerySchemaView::new(
        "certification-live-source",
        [
            SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
            SchemaFieldView::new("title", "value", SchemaFieldKind::String),
        ],
        [],
    );
    let mut source = RepresentativeSourceAdapter;
    let handle = source
        .declare_live_view(
            "tasks.certified".to_string(),
            request.clone(),
            schema_view.clone(),
        )
        .expect("live source declaration fixture should succeed");
    let request = ForgeQueryLowerRuntimeCapabilityRequest::new(
        ForgeQueryLowerRuntimeSeamKey::LiveViewSourceDeclaration,
        ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
        ForgeQueryLowerRuntimeAuthorityOwner::Query,
        "Live view source declaration",
        hash_parts(&[
            "live_view_source_route_subject_v1".to_string(),
            format!("view:{}", handle.name()),
            format!("target:{}", request.target()),
            format!("shape:{}", request.view_shape().as_str()),
            format!("schema_basis:{}", schema_view.basis().as_str()),
        ]),
    );
    let eligibility = ForgeQueryLowerRuntimeCapabilityEligibility::admitted(
        request.clone(),
        handle.name().to_string(),
    );
    let route_plan = ForgeQueryLowerRuntimeRoutePlan::new(eligibility.clone(), "live-source");
    let boundary_receipt = ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan(
        &route_plan,
        handle.name().to_string(),
    );
    let envelope = ForgeQueryLowerRuntimeBoundaryEnvelope::from_route_plan(
        ForgeQueryLowerRuntimeSeamKey::LiveViewSourceDeclaration,
        &route_plan,
        &boundary_receipt,
        handle.name(),
    );
    RepresentativeArtifacts {
        seam_key: ForgeQueryLowerRuntimeSeamKey::LiveViewSourceDeclaration,
        request,
        eligibility,
        route_plan: Some(route_plan),
        boundary_receipt,
        envelope,
        evidence_source: ForgeQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture,
    }
}

pub(crate) fn representative_preview_basis_row() -> RepresentativeArtifacts {
    let authority = ForgeQueryRuntimeEvidenceAuthority::new();
    let admission = ForgeQueryPreviewBasisAdmission::new(
        &authority,
        ForgeQuerySessionLabel::scoped_strs("lower-runtime-routing", ["preview.tasks.certified"])
            .expect("fixture label should build"),
        ForgeQueryEffectPolicy::DeriveOnly,
        ["preview-basis-evidence-a", "preview-basis-evidence-b"],
    );
    let request = ForgeQueryLowerRuntimeCapabilityRequest::new(
        ForgeQueryLowerRuntimeSeamKey::PreviewBasisAdmission,
        ForgeQueryLowerRuntimeRouteKind::ReadmissionHandoff,
        ForgeQueryLowerRuntimeAuthorityOwner::Query,
        "Preview basis admission",
        hash_parts(&[
            "preview_basis_route_subject_v1".to_string(),
            format!("label:{}", admission.label()),
            format!("policy:{}", admission.effect_policy().as_str()),
            format!("lane:{}", admission.authority_lane().as_str()),
            format!("evidence_count:{}", admission.evidence().len()),
        ]),
    );
    let eligibility = ForgeQueryLowerRuntimeCapabilityEligibility::admitted(
        request.clone(),
        hash_parts(admission.evidence()),
    );
    let handoff = ForgeQueryLowerRuntimeReadmissionReceipt::new(
        eligibility.clone(),
        admission.authority_lane().as_str(),
    );
    let boundary_receipt =
        ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_readmission_receipt(&handoff);
    let envelope = ForgeQueryLowerRuntimeBoundaryEnvelope::from_readmission_receipt(
        ForgeQueryLowerRuntimeSeamKey::PreviewBasisAdmission,
        &handoff,
        &boundary_receipt,
    );
    RepresentativeArtifacts {
        seam_key: ForgeQueryLowerRuntimeSeamKey::PreviewBasisAdmission,
        request,
        eligibility,
        route_plan: None,
        boundary_receipt,
        envelope,
        evidence_source: ForgeQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture,
    }
}

pub(crate) fn synthetic_inventory_row(
    row: &ForgeQueryLowerRuntimeCrossingRow,
) -> RepresentativeArtifacts {
    let request = ForgeQueryLowerRuntimeCapabilityRequest::new(
        row.seam_key(),
        row.route_kind(),
        row.lower_runtime_owner(),
        row.capability_label(),
        format!("{}-subject", row.seam_key().as_str()),
    );
    let eligibility = ForgeQueryLowerRuntimeCapabilityEligibility::admitted(
        request.clone(),
        format!("{}-eligibility-detail", row.seam_key().as_str()),
    );
    match row.route_kind() {
        ForgeQueryLowerRuntimeRouteKind::RoutePlanning => {
            let plan = ForgeQueryLowerRuntimeRoutePlan::new(
                eligibility.clone(),
                format!("{}-route", row.seam_key().as_str()),
            );
            let boundary_receipt = ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan(
                &plan,
                format!("{}-evidence", row.seam_key().as_str()),
            );
            let envelope = ForgeQueryLowerRuntimeBoundaryEnvelope::from_route_plan(
                row.seam_key(),
                &plan,
                &boundary_receipt,
                &format!("{}-evidence", row.seam_key().as_str()),
            );
            RepresentativeArtifacts {
                seam_key: row.seam_key(),
                request,
                eligibility,
                route_plan: Some(plan),
                boundary_receipt,
                envelope,
                evidence_source:
                    ForgeQueryLowerRuntimeRepresentativeEvidenceSource::InventorySynthesized,
            }
        }
        ForgeQueryLowerRuntimeRouteKind::ReadmissionHandoff => {
            let handoff = ForgeQueryLowerRuntimeReadmissionReceipt::new(
                eligibility.clone(),
                format!("{}-evidence", row.seam_key().as_str()),
            );
            let boundary_receipt =
                ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_readmission_receipt(&handoff);
            let envelope = ForgeQueryLowerRuntimeBoundaryEnvelope::from_readmission_receipt(
                row.seam_key(),
                &handoff,
                &boundary_receipt,
            );
            RepresentativeArtifacts {
                seam_key: row.seam_key(),
                request,
                eligibility,
                route_plan: None,
                boundary_receipt,
                envelope,
                evidence_source:
                    ForgeQueryLowerRuntimeRepresentativeEvidenceSource::InventorySynthesized,
            }
        }
    }
}

pub(crate) fn normalized_parity_digest(
    label: &str,
    envelopes: &[ForgeQueryLowerRuntimeBoundaryEnvelope],
) -> String {
    let selected = match label {
        "compose-read" => envelopes
            .iter()
            .filter(|row| {
                matches!(
                    row.seam_key().as_str(),
                    "compose-read" | "execute-read-family"
                )
            })
            .collect::<Vec<_>>(),
        _ => envelopes
            .iter()
            .filter(|row| {
                matches!(
                    row.seam_key().as_str(),
                    "basis-readmission-from-truth-view-evidence"
                        | "basis-readmission-from-subscription-evidence"
                )
            })
            .collect::<Vec<_>>(),
    };
    hash_parts(
        &selected
            .iter()
            .map(|envelope| {
                format!(
                    "{}|{}|{}|{}|{}|{}|{}",
                    envelope.authority_owner().as_str(),
                    envelope.route_kind().as_str(),
                    envelope.support_posture().as_str(),
                    envelope.route_cost_posture().as_str(),
                    envelope.route_failure_topology().as_str(),
                    envelope.artifact_strength().as_str(),
                    envelope.crossing_classification().as_str()
                )
            })
            .collect::<Vec<_>>(),
    )
}

pub(crate) fn hostile_parity_divergence_digest(
    envelopes: &[ForgeQueryLowerRuntimeBoundaryEnvelope],
) -> String {
    let readmission = envelopes
        .iter()
        .find(|row| row.seam_key().as_str() == "live-view-schema-admission")
        .expect("live-view schema admission seam should be present");
    let routing = envelopes
        .iter()
        .find(|row| row.seam_key().as_str() == "signal-invalidation-routing")
        .expect("signal invalidation routing seam should be present");

    hash_parts(&[
        "hostile-route-divergence".to_string(),
        format!(
            "{}|{}|{}|{}",
            readmission.authority_owner().as_str(),
            readmission.route_kind().as_str(),
            readmission.artifact_strength().as_str(),
            readmission.route_failure_topology().as_str(),
        ),
        format!(
            "{}|{}|{}|{}",
            routing.authority_owner().as_str(),
            routing.route_kind().as_str(),
            routing.artifact_strength().as_str(),
            routing.route_failure_topology().as_str(),
        ),
    ])
}

struct RepresentativeSourceAdapter;

impl ForgeQueryRuntimeSourceAdapter for RepresentativeSourceAdapter {
    fn declare_live_view(
        &mut self,
        name: String,
        _request: DeclarativeLiveQueryRequest,
        _schema_view: QuerySchemaView,
    ) -> Result<ForgeQueryLiveViewHandle, ForgeQueryWorkspaceError> {
        Ok(ForgeQueryLiveViewHandle::new(name))
    }

    fn live_entities(&self, _view_name: &str) -> Vec<ForgeQueryEntity> {
        Vec::new()
    }

    fn drain_live_patches(&mut self, _view_name: &str) -> Vec<ForgeQueryLivePatch> {
        Vec::new()
    }

    fn affected_live_view_ids(&self, _receipt: &ForgeQueryMutationReceipt) -> Vec<String> {
        Vec::new()
    }

    fn snapshot_token(&self) -> String {
        "representative-source-snapshot".to_string()
    }
}
