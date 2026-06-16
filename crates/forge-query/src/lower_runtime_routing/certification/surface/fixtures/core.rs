use crate::declarative_live::{
    DeclarativeLiveQueryRequest, DeclarativeLiveViewShape, DeclarativeProjectionField,
};
use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::lower_runtime_routing::{
    ForgeQueryLowerRuntimeAuthorityOwner, ForgeQueryLowerRuntimeBoundaryEnvelope,
    ForgeQueryLowerRuntimeBoundaryExecutionReceipt, ForgeQueryLowerRuntimeCapabilityEligibility,
    ForgeQueryLowerRuntimeCapabilityRequest, ForgeQueryLowerRuntimeCrossingRow,
    ForgeQueryLowerRuntimeReadmissionReceipt, ForgeQueryLowerRuntimeRouteKind,
    ForgeQueryLowerRuntimeRoutePlan, ForgeQueryLowerRuntimeRouteSubjectIdentity,
    ForgeQueryLowerRuntimeSeamKey, ForgeQueryLowerRuntimeSubjectIdentity,
    LiveViewDeclarationAdmissionBoundaryReceipt, SignalInvalidationBoundaryReceipt,
    WriteAuthorityExecutionReceipt,
};
use crate::memory_workspace::{
    ForgeQueryCommitIdentity, ForgeQueryEntityIdentity, ForgeQueryMutationDelta,
    ForgeQueryMutationKind, ForgeQueryMutationReceipt, ForgeQuerySnapshotIdentity,
};
use crate::runtime::{
    build_bridge_authority_bundle, ForgeQueryAspectValue, ForgeQueryBasisAdmissionEvidenceRow,
    ForgeQueryEffectPolicy, ForgeQueryPreviewBasisAdmission, ForgeQueryRuntimeEvidenceAuthority,
    ForgeQueryRuntimeSourceAdapter, ForgeQueryWriteCommand, LiveViewDeclarationAdmissionReceipt,
    SignalInvalidationRoutingReceipt,
};
use crate::schema_view::{QuerySchemaView, SchemaFieldKind, SchemaFieldView};
use crate::session_label::ForgeQuerySessionLabel;
use forge_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts;

use super::{
    representative_bridge_authority_runtime, ForgeQueryLowerRuntimeRepresentativeEvidenceSource,
    RepresentativeArtifacts, RepresentativeSourceAdapter,
};

fn fixture_retained_evidence_identity(
    fixture_family: impl AsRef<str>,
    retained_label: impl AsRef<str>,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(ForgeQueryEvidenceTag::new("fixture_family"), fixture_family)
        .field_value(
            ForgeQueryEvidenceTag::new("fixture_retained_label"),
            retained_label,
        )
        .seal()
}

fn fixture_subject_identity(
    subject_family: impl AsRef<str>,
    subject_label: impl AsRef<str>,
) -> ForgeQueryLowerRuntimeSubjectIdentity {
    let evidence_identity =
        fixture_retained_evidence_identity(subject_family.as_ref(), subject_label);
    ForgeQueryLowerRuntimeSubjectIdentity::compose(subject_family)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("fixture_subject"),
            &evidence_identity,
        )
        .seal()
}

fn fixture_route_subject_identity(
    route_family: impl AsRef<str>,
    route_label: impl AsRef<str>,
) -> ForgeQueryLowerRuntimeRouteSubjectIdentity {
    let evidence_identity = fixture_retained_evidence_identity(route_family.as_ref(), route_label);
    ForgeQueryLowerRuntimeRouteSubjectIdentity::from_evidence_identity(
        route_family,
        &evidence_identity,
    )
}

fn admitted_fixture_eligibility(
    request: ForgeQueryLowerRuntimeCapabilityRequest,
    detail_family: impl AsRef<str>,
    detail_label: impl AsRef<str>,
) -> ForgeQueryLowerRuntimeCapabilityEligibility {
    let evidence_identity = fixture_retained_evidence_identity(detail_family, detail_label);
    admitted_fixture_eligibility_from_evidence(request, &evidence_identity)
}

fn admitted_fixture_eligibility_from_evidence(
    request: ForgeQueryLowerRuntimeCapabilityRequest,
    evidence_identity: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryLowerRuntimeCapabilityEligibility {
    ForgeQueryLowerRuntimeCapabilityEligibility::admitted_with_evidence_identity(
        request,
        &evidence_identity,
    )
}

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
        entity_identity: representative_entity_identity("task-7"),
    };
    let mutation_receipt = ForgeQueryMutationReceipt::from_authoritative_parts(
        representative_commit_identity("commit-route-write-7"),
        representative_snapshot_identity("snapshot-route-write-7"),
        vec![ForgeQueryMutationDelta::new(
            "Task",
            representative_entity_identity("task-7"),
            ForgeQueryMutationKind::Deleted,
            vec!["status.value".to_string()],
        )],
    );
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
    let representative_task_identity = representative_entity_identity("task-9");
    let command = ForgeQueryWriteCommand::UpdateAspects {
        entity_identity: representative_task_identity.clone(),
        aspects: vec![
            ForgeQueryAspectValue::new_set("status.value", "ready")
                .expect("representative signal status aspect should build"),
            ForgeQueryAspectValue::new_set("priority.value", "high")
                .expect("representative signal priority aspect should build"),
        ],
        metadata: Default::default(),
        naming_intent: None,
        continuity_intent: None,
    };
    let bridge = representative_bridge_authority_runtime();
    let snapshot_identity = ForgeQuerySnapshotIdentity::from_relational_snapshot(
        RelationalBridgeSnapshotIdentityParts::new(1, 1),
    );
    let bridge_authority = build_bridge_authority_bundle(
        &bridge,
        &snapshot_identity,
        &command,
        "Task",
        &representative_task_identity,
        ForgeQueryMutationKind::Updated,
    )
    .expect("representative signal authority should build");
    let mutation_receipt = ForgeQueryMutationReceipt::from_bridge_authoritative_parts(
        ForgeQueryCommitIdentity::from_relational_commit_id(1),
        snapshot_identity,
        vec![
            ForgeQueryMutationDelta::new(
                "Task",
                representative_task_identity,
                ForgeQueryMutationKind::Updated,
                vec!["status.value".to_string()],
            ),
            ForgeQueryMutationDelta::new(
                "Task",
                representative_entity_identity("task-10"),
                ForgeQueryMutationKind::Updated,
                vec!["priority.value".to_string()],
            ),
        ],
        bridge_authority,
    );
    let routing = SignalInvalidationRoutingReceipt::from_mutation_receipt(&mutation_receipt)
        .expect("representative signal routing fixture must carry bridge authority");
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

fn representative_commit_identity(label: impl AsRef<str>) -> ForgeQueryCommitIdentity {
    ForgeQueryCommitIdentity::preview(
        ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::WriteReceiptCommitIdentity)
            .field_value(ForgeQueryEvidenceTag::new("representative_commit"), label)
            .seal(),
    )
}

fn representative_snapshot_identity(label: impl AsRef<str>) -> ForgeQuerySnapshotIdentity {
    ForgeQuerySnapshotIdentity::preview(
        ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::WriteReceiptSnapshotIdentity)
            .field_value(ForgeQueryEvidenceTag::new("representative_snapshot"), label)
            .seal(),
    )
}

fn representative_entity_identity(label: impl AsRef<str>) -> ForgeQueryEntityIdentity {
    crate::memory_workspace::admit_authored_entity_label(label)
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
        ForgeQueryLowerRuntimeSubjectIdentity::compose("live-view-source-route-subject")
            .field_value(ForgeQueryEvidenceTag::new("view"), handle.name())
            .field_value(ForgeQueryEvidenceTag::new("target"), request.target())
            .field_shape(
                ForgeQueryEvidenceTag::new("shape"),
                request.view_shape().as_str(),
            )
            .field_value(
                ForgeQueryEvidenceTag::new("schema_basis"),
                schema_view.basis().as_str(),
            )
            .seal(),
    );
    let eligibility = admitted_fixture_eligibility(
        request.clone(),
        "live-view-source-eligibility",
        handle.name().to_string(),
    );
    let route_plan = ForgeQueryLowerRuntimeRoutePlan::new(
        eligibility.clone(),
        fixture_route_subject_identity("live-view-source-route", handle.name()),
    );
    let retained_evidence =
        crate::lower_runtime_routing::forge_query_lower_runtime_retained_evidence_identity(
            "representative-live-view-source",
            &fixture_retained_evidence_identity("representative-live-view-source", handle.name()),
        );
    let boundary_receipt = ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan(
        &route_plan,
        &retained_evidence,
    );
    let envelope = ForgeQueryLowerRuntimeBoundaryEnvelope::from_route_plan(
        ForgeQueryLowerRuntimeSeamKey::LiveViewSourceDeclaration,
        &route_plan,
        &boundary_receipt,
        &retained_evidence,
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
        ForgeQueryBasisAdmissionEvidenceRow::rows_from_values([
            "preview-basis-evidence-a",
            "preview-basis-evidence-b",
        ]),
    );
    let request = ForgeQueryLowerRuntimeCapabilityRequest::new(
        ForgeQueryLowerRuntimeSeamKey::PreviewBasisAdmission,
        ForgeQueryLowerRuntimeRouteKind::ReadmissionHandoff,
        ForgeQueryLowerRuntimeAuthorityOwner::Query,
        "Preview basis admission",
        ForgeQueryLowerRuntimeSubjectIdentity::compose("preview-basis-route-subject")
            .field_value(ForgeQueryEvidenceTag::new("label"), admission.label())
            .field_shape(
                ForgeQueryEvidenceTag::new("policy"),
                admission.effect_policy().as_str(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("lane"),
                admission.authority_lane().as_str(),
            )
            .field_usize(
                ForgeQueryEvidenceTag::new("evidence_count"),
                admission.evidence().len(),
            )
            .seal(),
    );
    let admission_evidence_identity =
        ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
            .field_value(ForgeQueryEvidenceTag::new("label"), admission.label())
            .field_shape(
                ForgeQueryEvidenceTag::new("policy"),
                admission.effect_policy().as_str(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("lane"),
                admission.authority_lane().as_str(),
            )
            .field_value_sequence(ForgeQueryEvidenceTag::new("evidence"), admission.evidence())
            .seal();
    let eligibility =
        admitted_fixture_eligibility_from_evidence(request.clone(), &admission_evidence_identity);
    let retained_evidence =
        crate::lower_runtime_routing::forge_query_lower_runtime_retained_evidence_identity(
            "representative-preview-basis-admission",
            &fixture_retained_evidence_identity(
                "representative-preview-basis-admission",
                admission.authority_lane().as_str(),
            ),
        );
    let handoff =
        ForgeQueryLowerRuntimeReadmissionReceipt::new(eligibility.clone(), &retained_evidence);
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
        fixture_subject_identity(
            "synthetic-inventory-route-subject",
            format!("{}-subject", row.seam_key().as_str()),
        ),
    );
    let eligibility = admitted_fixture_eligibility(
        request.clone(),
        "synthetic-inventory-eligibility",
        format!("{}-eligibility-detail", row.seam_key().as_str()),
    );
    match row.route_kind() {
        ForgeQueryLowerRuntimeRouteKind::RoutePlanning => {
            let plan = ForgeQueryLowerRuntimeRoutePlan::new(
                eligibility.clone(),
                fixture_route_subject_identity(
                    "synthetic-inventory-route",
                    format!("{}-route", row.seam_key().as_str()),
                ),
            );
            let retained_evidence =
                crate::lower_runtime_routing::forge_query_lower_runtime_retained_evidence_identity(
                    "synthetic-inventory-route-plan",
                    &fixture_retained_evidence_identity(
                        "synthetic-inventory-route-plan",
                        format!("{}-evidence", row.seam_key().as_str()),
                    ),
                );
            let boundary_receipt = ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan(
                &plan,
                &retained_evidence,
            );
            let envelope = ForgeQueryLowerRuntimeBoundaryEnvelope::from_route_plan(
                row.seam_key(),
                &plan,
                &boundary_receipt,
                &retained_evidence,
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
            let retained_evidence =
                crate::lower_runtime_routing::forge_query_lower_runtime_retained_evidence_identity(
                    "synthetic-inventory-readmission",
                    &fixture_retained_evidence_identity(
                        "synthetic-inventory-readmission",
                        format!("{}-evidence", row.seam_key().as_str()),
                    ),
                );
            let handoff = ForgeQueryLowerRuntimeReadmissionReceipt::new(
                eligibility.clone(),
                &retained_evidence,
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
    let row_identities = selected
        .iter()
        .map(|envelope| {
            ForgeQueryEvidenceIdentity::compose(
                ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("owner"),
                envelope.authority_owner().as_str(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("route_kind"),
                envelope.route_kind().as_str(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("support"),
                envelope.support_posture().as_str(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("cost"),
                envelope.route_cost_posture().as_str(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("failure"),
                envelope.route_failure_topology().as_str(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("strength"),
                envelope.artifact_strength().as_str(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("classification"),
                envelope.crossing_classification().as_str(),
            )
            .seal()
        })
        .collect::<Vec<_>>();
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(ForgeQueryEvidenceTag::new("parity_label"), label)
        .field_evidence_identity_sequence(ForgeQueryEvidenceTag::new("rows"), &row_identities)
        .seal()
        .as_str()
        .to_string()
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

    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("parity_label"),
            "hostile-route-divergence",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("readmission_owner"),
            readmission.authority_owner().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("readmission_route_kind"),
            readmission.route_kind().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("readmission_strength"),
            readmission.artifact_strength().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("readmission_failure"),
            readmission.route_failure_topology().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("routing_owner"),
            routing.authority_owner().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("routing_route_kind"),
            routing.route_kind().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("routing_strength"),
            routing.artifact_strength().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("routing_failure"),
            routing.route_failure_topology().as_str(),
        )
        .seal()
        .as_str()
        .to_string()
}
