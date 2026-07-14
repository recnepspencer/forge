use crate::declarative_live::{
    DeclarativeLiveQueryRequest, DeclarativeLiveViewShape, DeclarativeProjectionField,
};
use crate::evidence_identity::{
    WorthQueryEvidenceIdentity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};
use crate::lower_runtime_routing::{
    LiveViewDeclarationAdmissionBoundaryReceipt, SignalInvalidationBoundaryReceipt,
    WorthQueryLowerRuntimeAuthorityOwner, WorthQueryLowerRuntimeBoundaryEnvelope,
    WorthQueryLowerRuntimeBoundaryExecutionReceipt, WorthQueryLowerRuntimeCapabilityEligibility,
    WorthQueryLowerRuntimeCapabilityRequest, WorthQueryLowerRuntimeCrossingRow,
    WorthQueryLowerRuntimeReadmissionReceipt, WorthQueryLowerRuntimeRouteKind,
    WorthQueryLowerRuntimeRoutePlan, WorthQueryLowerRuntimeRouteSubjectIdentity,
    WorthQueryLowerRuntimeSeamKey, WorthQueryLowerRuntimeSubjectIdentity,
    WriteAuthorityExecutionReceipt,
};
use crate::memory_workspace::{
    WorthQueryCommitIdentity, WorthQueryEntityIdentity, WorthQueryMutationDelta,
    WorthQueryMutationKind, WorthQueryMutationReceipt, WorthQuerySnapshotIdentity,
};
use crate::runtime::{
    build_bridge_authority_bundle, LiveViewDeclarationAdmissionReceipt,
    SignalInvalidationRoutingReceipt, WorthQueryAdmittedAspectValue,
    WorthQueryBackendAdmissibleMutation, WorthQueryBasisAdmissionEvidenceRow,
    WorthQueryEffectPolicy, WorthQueryPreviewBasisAdmission, WorthQueryRuntimeEvidenceAuthority,
    WorthQueryRuntimeSourceAdapter, WorthQueryWriteCommand,
};
use crate::schema_view::{QuerySchemaView, SchemaFieldKind, SchemaFieldView};
use crate::session_label::WorthQuerySessionLabel;
use worth_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts;

use super::{
    priority_value_touch, representative_bridge_authority_runtime, status_value_touch,
    RepresentativeArtifacts, RepresentativeSourceAdapter,
    WorthQueryLowerRuntimeRepresentativeEvidenceSource,
};

fn fixture_retained_evidence_identity(
    fixture_family: impl AsRef<str>,
    retained_label: impl AsRef<str>,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(WorthQueryEvidenceTag::new("fixture_family"), fixture_family)
        .field_value(
            WorthQueryEvidenceTag::new("fixture_retained_label"),
            retained_label,
        )
        .seal()
}

fn fixture_subject_identity(
    subject_family: impl AsRef<str>,
    subject_label: impl AsRef<str>,
) -> WorthQueryLowerRuntimeSubjectIdentity {
    let evidence_identity =
        fixture_retained_evidence_identity(subject_family.as_ref(), subject_label);
    WorthQueryLowerRuntimeSubjectIdentity::compose(subject_family)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("fixture_subject"),
            &evidence_identity,
        )
        .seal()
}

fn fixture_route_subject_identity(
    route_family: impl AsRef<str>,
    route_label: impl AsRef<str>,
) -> WorthQueryLowerRuntimeRouteSubjectIdentity {
    let evidence_identity = fixture_retained_evidence_identity(route_family.as_ref(), route_label);
    WorthQueryLowerRuntimeRouteSubjectIdentity::from_evidence_identity(
        route_family,
        &evidence_identity,
    )
}

fn admitted_fixture_eligibility(
    request: WorthQueryLowerRuntimeCapabilityRequest,
    detail_family: impl AsRef<str>,
    detail_label: impl AsRef<str>,
) -> WorthQueryLowerRuntimeCapabilityEligibility {
    let evidence_identity = fixture_retained_evidence_identity(detail_family, detail_label);
    admitted_fixture_eligibility_from_evidence(request, &evidence_identity)
}

fn admitted_fixture_eligibility_from_evidence(
    request: WorthQueryLowerRuntimeCapabilityRequest,
    evidence_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryLowerRuntimeCapabilityEligibility {
    WorthQueryLowerRuntimeCapabilityEligibility::admitted_with_evidence_identity(
        request,
        &evidence_identity,
    )
}

pub(crate) fn representative_live_view_schema_row() -> RepresentativeArtifacts {
    let request = DeclarativeLiveQueryRequest::new("Task", DeclarativeLiveViewShape::table())
        .project(DeclarativeProjectionField::from_authoring_parts(
            "title", "value",
        ))
        .project(DeclarativeProjectionField::from_authoring_parts(
            "status", "value",
        ));
    let admission_receipt =
        LiveViewDeclarationAdmissionReceipt::from_request("tasks.table", &request);
    let boundary_receipt = LiveViewDeclarationAdmissionBoundaryReceipt::from_request(
        "tasks.table",
        &request,
        admission_receipt,
    );
    RepresentativeArtifacts {
        seam_key: WorthQueryLowerRuntimeSeamKey::LiveViewSchemaAdmission,
        request: boundary_receipt
            .readmission_receipt()
            .eligibility()
            .request()
            .clone(),
        eligibility: boundary_receipt.readmission_receipt().eligibility().clone(),
        route_plan: None,
        boundary_receipt: boundary_receipt.boundary_execution_receipt().clone(),
        envelope: boundary_receipt.boundary_envelope().clone(),
        evidence_source: WorthQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture,
    }
}

pub(crate) fn representative_write_authority_row() -> RepresentativeArtifacts {
    let command = WorthQueryWriteCommand::Delete {
        entity_identity: representative_entity_identity("task-7"),
    };
    let mutation_receipt = WorthQueryMutationReceipt::from_authoritative_parts(
        representative_commit_identity("commit-route-write-7"),
        representative_snapshot_identity("snapshot-route-write-7"),
        vec![WorthQueryMutationDelta::from_touched_aspects(
            "Task",
            representative_entity_identity("task-7"),
            WorthQueryMutationKind::Deleted,
            vec![status_value_touch()],
        )],
    );
    let execution = WriteAuthorityExecutionReceipt::from_command(&command, mutation_receipt);
    RepresentativeArtifacts {
        seam_key: WorthQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
        request: execution.route_plan().eligibility().request().clone(),
        eligibility: execution.route_plan().eligibility().clone(),
        route_plan: Some(execution.route_plan().clone()),
        boundary_receipt: execution.boundary_execution_receipt().clone(),
        envelope: execution.boundary_envelope().clone(),
        evidence_source: WorthQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture,
    }
}

pub(crate) fn representative_signal_invalidation_row() -> RepresentativeArtifacts {
    let representative_task_identity = representative_entity_identity("task-9");
    let command = WorthQueryWriteCommand::UpdateAspects {
        entity_identity: representative_task_identity.clone(),
        aspects: vec![
            WorthQueryAdmittedAspectValue::new_set(
                status_value_touch(),
                crate::runtime::WorthQueryAdmittedAspectValue::native_string_value("ready"),
            )
            .expect("representative signal status aspect should build"),
            WorthQueryAdmittedAspectValue::new_set(
                priority_value_touch(),
                crate::runtime::WorthQueryAdmittedAspectValue::native_string_value("high"),
            )
            .expect("representative signal priority aspect should build"),
        ],
        metadata: Default::default(),
        naming_intent: None,
        continuity_intent: None,
    };
    let mutation = WorthQueryBackendAdmissibleMutation::from_admitted_command(command);
    let bridge = representative_bridge_authority_runtime();
    let snapshot_identity = WorthQuerySnapshotIdentity::from_relational_snapshot(
        RelationalBridgeSnapshotIdentityParts::new(1, 1),
    );
    let bridge_authority = build_bridge_authority_bundle(
        &bridge,
        &snapshot_identity,
        &mutation,
        "Task",
        &representative_task_identity,
        WorthQueryMutationKind::Updated,
    )
    .expect("representative signal authority should build");
    let mutation_receipt = WorthQueryMutationReceipt::from_bridge_authoritative_parts(
        WorthQueryCommitIdentity::from_relational_commit_id(1),
        snapshot_identity,
        vec![
            WorthQueryMutationDelta::from_touched_aspects(
                "Task",
                representative_task_identity,
                WorthQueryMutationKind::Updated,
                vec![status_value_touch()],
            ),
            WorthQueryMutationDelta::from_touched_aspects(
                "Task",
                representative_entity_identity("task-10"),
                WorthQueryMutationKind::Updated,
                vec![priority_value_touch()],
            ),
        ],
        bridge_authority,
    );
    let routing = SignalInvalidationRoutingReceipt::from_mutation_receipt(&mutation_receipt)
        .expect("representative signal routing fixture must carry bridge authority");
    let boundary_receipt =
        SignalInvalidationBoundaryReceipt::from_mutation_receipt(&mutation_receipt, routing);
    RepresentativeArtifacts {
        seam_key: WorthQueryLowerRuntimeSeamKey::SignalInvalidationRouting,
        request: boundary_receipt
            .route_plan()
            .eligibility()
            .request()
            .clone(),
        eligibility: boundary_receipt.route_plan().eligibility().clone(),
        route_plan: Some(boundary_receipt.route_plan().clone()),
        boundary_receipt: boundary_receipt.boundary_execution_receipt().clone(),
        envelope: boundary_receipt.boundary_envelope().clone(),
        evidence_source: WorthQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture,
    }
}

fn representative_commit_identity(label: impl AsRef<str>) -> WorthQueryCommitIdentity {
    WorthQueryCommitIdentity::preview(
        WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WriteReceiptCommitIdentity)
            .field_value(WorthQueryEvidenceTag::new("representative_commit"), label)
            .seal(),
    )
}

fn representative_snapshot_identity(label: impl AsRef<str>) -> WorthQuerySnapshotIdentity {
    WorthQuerySnapshotIdentity::preview(
        WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WriteReceiptSnapshotIdentity)
            .field_value(WorthQueryEvidenceTag::new("representative_snapshot"), label)
            .seal(),
    )
}

fn representative_entity_identity(label: impl AsRef<str>) -> WorthQueryEntityIdentity {
    crate::memory_workspace::admit_authored_entity_label(label)
}

pub(crate) fn representative_live_view_source_row() -> RepresentativeArtifacts {
    let request = DeclarativeLiveQueryRequest::new("Task", DeclarativeLiveViewShape::table())
        .project(DeclarativeProjectionField::from_authoring_parts(
            "identity", "id",
        ))
        .project(DeclarativeProjectionField::from_authoring_parts(
            "title", "value",
        ));
    let schema_view = QuerySchemaView::new(
        "certification-live-source",
        [
            SchemaFieldView::new(
                crate::authoring::AspectName::new("identity")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("id").expect("schema field literal must be valid"),
                SchemaFieldKind::String,
            ),
            SchemaFieldView::new(
                crate::authoring::AspectName::new("title")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("value")
                    .expect("schema field literal must be valid"),
                SchemaFieldKind::String,
            ),
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
    let request = WorthQueryLowerRuntimeCapabilityRequest::new(
        WorthQueryLowerRuntimeSeamKey::LiveViewSourceDeclaration,
        WorthQueryLowerRuntimeRouteKind::RoutePlanning,
        WorthQueryLowerRuntimeAuthorityOwner::Query,
        "Live view source declaration",
        WorthQueryLowerRuntimeSubjectIdentity::compose("live-view-source-route-subject")
            .field_value(WorthQueryEvidenceTag::new("view"), handle.name())
            .field_value(WorthQueryEvidenceTag::new("target"), request.target())
            .field_shape(
                WorthQueryEvidenceTag::new("shape"),
                request.view_shape().as_str(),
            )
            .field_value(
                WorthQueryEvidenceTag::new("schema_basis"),
                schema_view.basis().as_str(),
            )
            .seal(),
    );
    let eligibility = admitted_fixture_eligibility(
        request.clone(),
        "live-view-source-eligibility",
        handle.name().to_string(),
    );
    let route_plan = WorthQueryLowerRuntimeRoutePlan::new(
        eligibility.clone(),
        fixture_route_subject_identity("live-view-source-route", handle.name()),
    );
    let retained_evidence =
        crate::lower_runtime_routing::worth_query_lower_runtime_retained_evidence_identity(
            "representative-live-view-source",
            &fixture_retained_evidence_identity("representative-live-view-source", handle.name()),
        );
    let boundary_receipt = WorthQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan(
        &route_plan,
        &retained_evidence,
    );
    let envelope = WorthQueryLowerRuntimeBoundaryEnvelope::from_route_plan(
        WorthQueryLowerRuntimeSeamKey::LiveViewSourceDeclaration,
        &route_plan,
        &boundary_receipt,
        &retained_evidence,
    );
    RepresentativeArtifacts {
        seam_key: WorthQueryLowerRuntimeSeamKey::LiveViewSourceDeclaration,
        request,
        eligibility,
        route_plan: Some(route_plan),
        boundary_receipt,
        envelope,
        evidence_source: WorthQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture,
    }
}

pub(crate) fn representative_preview_basis_row() -> RepresentativeArtifacts {
    let authority = WorthQueryRuntimeEvidenceAuthority::new();
    let admission = WorthQueryPreviewBasisAdmission::new(
        &authority,
        WorthQuerySessionLabel::scoped_strs("lower-runtime-routing", ["preview.tasks.certified"])
            .expect("fixture label should build"),
        WorthQueryEffectPolicy::DeriveOnly,
        WorthQueryBasisAdmissionEvidenceRow::rows_from_values([
            "preview-basis-evidence-a",
            "preview-basis-evidence-b",
        ]),
    );
    let request = WorthQueryLowerRuntimeCapabilityRequest::new(
        WorthQueryLowerRuntimeSeamKey::PreviewBasisAdmission,
        WorthQueryLowerRuntimeRouteKind::ReadmissionHandoff,
        WorthQueryLowerRuntimeAuthorityOwner::Query,
        "Preview basis admission",
        WorthQueryLowerRuntimeSubjectIdentity::compose("preview-basis-route-subject")
            .field_value(WorthQueryEvidenceTag::new("label"), admission.label())
            .field_shape(
                WorthQueryEvidenceTag::new("policy"),
                admission.effect_policy().as_str(),
            )
            .field_shape(
                WorthQueryEvidenceTag::new("lane"),
                admission.authority_lane().as_str(),
            )
            .field_usize(
                WorthQueryEvidenceTag::new("evidence_count"),
                admission.evidence().len(),
            )
            .seal(),
    );
    let admission_evidence_identity =
        WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
            .field_value(WorthQueryEvidenceTag::new("label"), admission.label())
            .field_shape(
                WorthQueryEvidenceTag::new("policy"),
                admission.effect_policy().as_str(),
            )
            .field_shape(
                WorthQueryEvidenceTag::new("lane"),
                admission.authority_lane().as_str(),
            )
            .field_value_sequence(WorthQueryEvidenceTag::new("evidence"), admission.evidence())
            .seal();
    let eligibility =
        admitted_fixture_eligibility_from_evidence(request.clone(), &admission_evidence_identity);
    let retained_evidence =
        crate::lower_runtime_routing::worth_query_lower_runtime_retained_evidence_identity(
            "representative-preview-basis-admission",
            &fixture_retained_evidence_identity(
                "representative-preview-basis-admission",
                admission.authority_lane().as_str(),
            ),
        );
    let handoff =
        WorthQueryLowerRuntimeReadmissionReceipt::new(eligibility.clone(), &retained_evidence);
    let boundary_receipt =
        WorthQueryLowerRuntimeBoundaryExecutionReceipt::from_readmission_receipt(&handoff);
    let envelope = WorthQueryLowerRuntimeBoundaryEnvelope::from_readmission_receipt(
        WorthQueryLowerRuntimeSeamKey::PreviewBasisAdmission,
        &handoff,
        &boundary_receipt,
    );
    RepresentativeArtifacts {
        seam_key: WorthQueryLowerRuntimeSeamKey::PreviewBasisAdmission,
        request,
        eligibility,
        route_plan: None,
        boundary_receipt,
        envelope,
        evidence_source: WorthQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture,
    }
}

pub(crate) fn synthetic_inventory_row(
    row: &WorthQueryLowerRuntimeCrossingRow,
) -> RepresentativeArtifacts {
    let request = WorthQueryLowerRuntimeCapabilityRequest::new(
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
        WorthQueryLowerRuntimeRouteKind::RoutePlanning => {
            let plan = WorthQueryLowerRuntimeRoutePlan::new(
                eligibility.clone(),
                fixture_route_subject_identity(
                    "synthetic-inventory-route",
                    format!("{}-route", row.seam_key().as_str()),
                ),
            );
            let retained_evidence =
                crate::lower_runtime_routing::worth_query_lower_runtime_retained_evidence_identity(
                    "synthetic-inventory-route-plan",
                    &fixture_retained_evidence_identity(
                        "synthetic-inventory-route-plan",
                        format!("{}-evidence", row.seam_key().as_str()),
                    ),
                );
            let boundary_receipt = WorthQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan(
                &plan,
                &retained_evidence,
            );
            let envelope = WorthQueryLowerRuntimeBoundaryEnvelope::from_route_plan(
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
                    WorthQueryLowerRuntimeRepresentativeEvidenceSource::InventorySynthesized,
            }
        }
        WorthQueryLowerRuntimeRouteKind::ReadmissionHandoff => {
            let retained_evidence =
                crate::lower_runtime_routing::worth_query_lower_runtime_retained_evidence_identity(
                    "synthetic-inventory-readmission",
                    &fixture_retained_evidence_identity(
                        "synthetic-inventory-readmission",
                        format!("{}-evidence", row.seam_key().as_str()),
                    ),
                );
            let handoff = WorthQueryLowerRuntimeReadmissionReceipt::new(
                eligibility.clone(),
                &retained_evidence,
            );
            let boundary_receipt =
                WorthQueryLowerRuntimeBoundaryExecutionReceipt::from_readmission_receipt(&handoff);
            let envelope = WorthQueryLowerRuntimeBoundaryEnvelope::from_readmission_receipt(
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
                    WorthQueryLowerRuntimeRepresentativeEvidenceSource::InventorySynthesized,
            }
        }
    }
}

pub(crate) fn normalized_parity_digest(
    label: &str,
    envelopes: &[WorthQueryLowerRuntimeBoundaryEnvelope],
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
            WorthQueryEvidenceIdentity::compose(
                WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
            )
            .field_shape(
                WorthQueryEvidenceTag::new("owner"),
                envelope.authority_owner().as_str(),
            )
            .field_shape(
                WorthQueryEvidenceTag::new("route_kind"),
                envelope.route_kind().as_str(),
            )
            .field_shape(
                WorthQueryEvidenceTag::new("support"),
                envelope.support_posture().as_str(),
            )
            .field_shape(
                WorthQueryEvidenceTag::new("cost"),
                envelope.route_cost_posture().as_str(),
            )
            .field_shape(
                WorthQueryEvidenceTag::new("failure"),
                envelope.route_failure_topology().as_str(),
            )
            .field_shape(
                WorthQueryEvidenceTag::new("strength"),
                envelope.artifact_strength().as_str(),
            )
            .field_shape(
                WorthQueryEvidenceTag::new("classification"),
                envelope.crossing_classification().as_str(),
            )
            .seal()
        })
        .collect::<Vec<_>>();
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(WorthQueryEvidenceTag::new("parity_label"), label)
        .field_evidence_identity_sequence(WorthQueryEvidenceTag::new("rows"), &row_identities)
        .seal()
        .as_str()
        .to_string()
}

pub(crate) fn hostile_parity_divergence_digest(
    envelopes: &[WorthQueryLowerRuntimeBoundaryEnvelope],
) -> String {
    let readmission = envelopes
        .iter()
        .find(|row| row.seam_key().as_str() == "live-view-schema-admission")
        .expect("live-view schema admission seam should be present");
    let routing = envelopes
        .iter()
        .find(|row| row.seam_key().as_str() == "signal-invalidation-routing")
        .expect("signal invalidation routing seam should be present");

    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            WorthQueryEvidenceTag::new("parity_label"),
            "hostile-route-divergence",
        )
        .field_shape(
            WorthQueryEvidenceTag::new("readmission_owner"),
            readmission.authority_owner().as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("readmission_route_kind"),
            readmission.route_kind().as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("readmission_strength"),
            readmission.artifact_strength().as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("readmission_failure"),
            readmission.route_failure_topology().as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("routing_owner"),
            routing.authority_owner().as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("routing_route_kind"),
            routing.route_kind().as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("routing_strength"),
            routing.artifact_strength().as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("routing_failure"),
            routing.route_failure_topology().as_str(),
        )
        .seal()
        .as_str()
        .to_string()
}
