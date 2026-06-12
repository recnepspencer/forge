use super::*;

#[test]
fn route_plan_drift_rejects_foreign_boundary_receipt() {
    let request = ForgeQueryLowerRuntimeCapabilityRequest::new(
        ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
        ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
        ForgeQueryLowerRuntimeAuthorityOwner::Query,
        WRITE_AUTHORITY_CAPABILITY_LABEL,
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeSubjectIdentity::compose(
            "test-subject",
        )
        .field_identity(
            crate::evidence_identity::ForgeQueryEvidenceTag::new("test_subject"),
            "subject-a",
        )
        .seal(),
    );
    let detail_a = crate::evidence_identity::ForgeQueryEvidenceIdentity::compose(
        crate::evidence_identity::ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
    )
    .field_identity(
        crate::evidence_identity::ForgeQueryEvidenceTag::new("test_detail"),
        "detail-a",
    )
    .seal();
    let eligibility = ForgeQueryLowerRuntimeCapabilityEligibility::admitted_with_evidence_identity(
        request, &detail_a,
    );
    let plan = ForgeQueryLowerRuntimeRoutePlan::new(
        eligibility,
        ForgeQueryLowerRuntimeRouteSubjectIdentity::from_evidence_identity("test-route", &detail_a),
    );

    let foreign_request = ForgeQueryLowerRuntimeCapabilityRequest::new(
        ForgeQueryLowerRuntimeSeamKey::SignalInvalidationRouting,
        ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
        ForgeQueryLowerRuntimeAuthorityOwner::Query,
        SIGNAL_INVALIDATION_CAPABILITY_LABEL,
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeSubjectIdentity::compose(
            "test-subject",
        )
        .field_identity(
            crate::evidence_identity::ForgeQueryEvidenceTag::new("test_subject"),
            "subject-b",
        )
        .seal(),
    );
    let detail_b = crate::evidence_identity::ForgeQueryEvidenceIdentity::compose(
        crate::evidence_identity::ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
    )
    .field_identity(
        crate::evidence_identity::ForgeQueryEvidenceTag::new("test_detail"),
        "detail-b",
    )
    .seal();
    let foreign_eligibility =
        ForgeQueryLowerRuntimeCapabilityEligibility::admitted_with_evidence_identity(
            foreign_request,
            &detail_b,
        );
    let foreign_plan = ForgeQueryLowerRuntimeRoutePlan::new(
        foreign_eligibility,
        ForgeQueryLowerRuntimeRouteSubjectIdentity::from_evidence_identity("test-route", &detail_b),
    );
    let retained_a =
        crate::lower_runtime_routing::forge_query_lower_runtime_retained_evidence_identity(
            "runtime-backend-test",
            &crate::evidence_identity::ForgeQueryEvidenceIdentity::compose(
                crate::evidence_identity::ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
            )
            .field_identity(
                crate::evidence_identity::ForgeQueryEvidenceTag::new("test_retained"),
                "detail-a",
            )
            .seal(),
        );
    let retained_b =
        crate::lower_runtime_routing::forge_query_lower_runtime_retained_evidence_identity(
            "runtime-backend-test",
            &crate::evidence_identity::ForgeQueryEvidenceIdentity::compose(
                crate::evidence_identity::ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
            )
            .field_identity(
                crate::evidence_identity::ForgeQueryEvidenceTag::new("test_retained"),
                "detail-b",
            )
            .seal(),
        );
    let foreign_receipt =
        ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan(&foreign_plan, &retained_b);

    let drift = foreign_receipt
        .drift_from_route_plan(&plan, &retained_a)
        .expect("foreign route receipt must drift");

    assert!(drift.contains("boundary execution request digest"));
}

#[test]
fn write_authority_boundary_receipt_carries_boundary_envelope() {
    let command = ForgeQueryWriteCommand::Delete {
        entity_identity: crate::memory_workspace::ForgeQueryEntityIdentity::authored_command(
            "task-1",
        ),
    };
    let mutation_receipt = ForgeQueryMutationReceipt {
        commit_identity:
            crate::memory_workspace::ForgeQueryCommitIdentity::from_external_authority_label(
                "commit-1",
            ),
        snapshot_identity:
            crate::memory_workspace::ForgeQuerySnapshotIdentity::from_external_authority_label(
                "snapshot-1",
            ),
        deltas: Vec::new(),
        bridge_authority: None,
    };
    let receipt = WriteAuthorityExecutionReceipt::from_command(&command, mutation_receipt);

    assert_eq!(
        receipt.boundary_envelope().seam_key(),
        ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution
    );
    assert_eq!(
        receipt.boundary_envelope().boundary_execution_identity(),
        receipt
            .boundary_execution_receipt()
            .boundary_execution_identity()
    );
}
