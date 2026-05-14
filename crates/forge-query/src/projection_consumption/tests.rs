use super::{
    declare_projection_consumption, discover_projection_consumption_support,
    evaluate_projection_consumption_eligibility, ProjectMaterializedFacts,
    ProjectionConsumptionBindingContext, ProjectionConsumptionDeclarationError,
    ProjectionConsumptionDenialReason, ProjectionConsumptionEligibility, ProjectionConsumptionSource,
    ProjectionConsumptionSupportPosture, ProjectionConsumptionWarnings,
    ProjectionConsumptionWarningKind, ProjectionFactKind, ProjectionSourceFamily,
};

fn test_binding(visible_fields: &[&str]) -> ProjectionConsumptionBindingContext {
    ProjectionConsumptionBindingContext::test_only(
        "result-shape:test",
        "authorized-projection:test",
        visible_fields
            .iter()
            .map(|field| field.to_string())
            .collect(),
    )
}

fn test_source(family: ProjectionSourceFamily) -> ProjectionConsumptionSource {
    match family {
        ProjectionSourceFamily::QueryReadReceipt => ProjectionConsumptionSource::test_only(
            family,
            Some("query:test"),
            Some("basis:test"),
            Some("result:test"),
            Some("result-shape:test"),
            "read-graph:test",
        ),
        ProjectionSourceFamily::QueryWriteReceipt => ProjectionConsumptionSource::test_only(
            family,
            None,
            Some("snapshot:test"),
            None,
            None,
            "commit:test",
        ),
        ProjectionSourceFamily::QueryContextExecution => ProjectionConsumptionSource::test_only(
            family,
            Some("query:test"),
            Some("basis:test"),
            Some("result:test"),
            Some("result-shape:test"),
            "query-context:test",
        ),
    }
}

fn all_source_families() -> [ProjectionSourceFamily; 3] {
    [
        ProjectionSourceFamily::QueryReadReceipt,
        ProjectionSourceFamily::QueryWriteReceipt,
        ProjectionSourceFamily::QueryContextExecution,
    ]
}

fn request_for_kind(kind: ProjectionFactKind) -> ProjectMaterializedFacts {
    match kind {
        ProjectionFactKind::EntityIdentity => ProjectMaterializedFacts::declare().entity_identities(),
        ProjectionFactKind::ViewLocalIdentity => {
            ProjectMaterializedFacts::declare().view_local_identities()
        }
        ProjectionFactKind::TargetIdentity => ProjectMaterializedFacts::declare().target_identity(),
        ProjectionFactKind::SourceReference => {
            ProjectMaterializedFacts::declare().source_references()
        }
        ProjectionFactKind::EffectContinuity => {
            ProjectMaterializedFacts::declare().effect_continuity_facts()
        }
        ProjectionFactKind::Membership => ProjectMaterializedFacts::declare().memberships(),
        ProjectionFactKind::RelationEndpoint => {
            ProjectMaterializedFacts::declare().relation_endpoints()
        }
        ProjectionFactKind::DisplayField => {
            ProjectMaterializedFacts::declare().display_field("profile.display_name")
        }
        ProjectionFactKind::DerivedScalarField => {
            ProjectMaterializedFacts::declare().derived_scalar_field("profile.display_name")
        }
    }
}

fn visible_fields_for_kind(kind: ProjectionFactKind) -> Vec<&'static str> {
    match kind {
        ProjectionFactKind::DisplayField | ProjectionFactKind::DerivedScalarField => {
            vec!["profile.display_name"]
        }
        _ => vec!["identity.id"],
    }
}

fn assert_warning_matches_posture(
    warnings: &ProjectionConsumptionWarnings,
    expected: ProjectionConsumptionWarningKind,
) {
    assert_eq!(warnings.warning_kinds(), [expected]);
}

#[test]
fn equivalent_declarations_share_digest() {
    let requested = ProjectMaterializedFacts::declare()
        .entity_identities()
        .display_field("profile.display_name");

    let left = declare_projection_consumption(
        test_source(ProjectionSourceFamily::QueryReadReceipt),
        test_binding(&["identity.id", "profile.display_name"]),
        requested.clone(),
    )
    .expect("equivalent declaration should be valid");
    let right = declare_projection_consumption(
        test_source(ProjectionSourceFamily::QueryReadReceipt),
        test_binding(&["identity.id", "profile.display_name"]),
        requested,
    )
    .expect("equivalent declaration should be valid");

    assert_eq!(left.declaration_digest(), right.declaration_digest());
}

#[test]
fn read_receipt_admits_visible_identity_and_display_requests() {
    let declaration = declare_projection_consumption(
        test_source(ProjectionSourceFamily::QueryReadReceipt),
        test_binding(&["identity.id", "profile.display_name"]),
        ProjectMaterializedFacts::declare()
            .entity_identities()
            .display_field("profile.display_name"),
    )
    .expect("read-backed declaration should be valid");

    let eligibility = evaluate_projection_consumption_eligibility(&declaration);
    match eligibility {
        ProjectionConsumptionEligibility::Admitted(admitted) => {
            assert!(!admitted.eligibility_digest().is_empty());
        }
        other => panic!("unexpected eligibility posture: {other:?}"),
    }
}

#[test]
fn masked_display_field_denies_before_admission() {
    let declaration = declare_projection_consumption(
        test_source(ProjectionSourceFamily::QueryReadReceipt),
        test_binding(&["identity.id"]),
        ProjectMaterializedFacts::declare()
            .entity_identities()
            .display_field("profile.display_name"),
    )
    .expect("masked field declaration should still be structurally valid");

    let eligibility = evaluate_projection_consumption_eligibility(&declaration);
    match eligibility {
        ProjectionConsumptionEligibility::Denied(denied) => {
            assert_eq!(
                denied.reason(),
                &ProjectionConsumptionDenialReason::FactFamilyNotVisible {
                    field_key: "profile.display_name".to_string(),
                }
            );
            assert_eq!(denied.counters().denied_count(), 1);
        }
        other => panic!("unexpected eligibility posture: {other:?}"),
    }
}

#[test]
fn read_receipt_membership_request_returns_source_mismatch() {
    let declaration = declare_projection_consumption(
        test_source(ProjectionSourceFamily::QueryReadReceipt),
        test_binding(&["identity.id"]),
        ProjectMaterializedFacts::declare().memberships(),
    )
    .expect("membership declaration should be structurally valid");

    let eligibility = evaluate_projection_consumption_eligibility(&declaration);
    match eligibility {
        ProjectionConsumptionEligibility::SourceMismatch(mismatch) => {
            assert_eq!(
                mismatch.requested_fact_kind(),
                ProjectionFactKind::Membership
            );
            assert_eq!(
                mismatch.source_family(),
                ProjectionSourceFamily::QueryReadReceipt
            );
        }
        other => panic!("unexpected eligibility posture: {other:?}"),
    }
}

#[test]
fn write_receipt_target_identity_is_deferred_for_now() {
    let declaration = declare_projection_consumption(
        test_source(ProjectionSourceFamily::QueryWriteReceipt),
        test_binding(&[]),
        ProjectMaterializedFacts::declare().target_identity(),
    )
    .expect("write receipt declaration should be structurally valid");

    let eligibility = evaluate_projection_consumption_eligibility(&declaration);
    assert!(matches!(
        eligibility,
        ProjectionConsumptionEligibility::Deferred(_)
    ));
}

#[test]
fn query_context_support_and_admission_share_warning_posture() {
    let source = test_source(ProjectionSourceFamily::QueryContextExecution);
    let report = discover_projection_consumption_support(&source);
    let row = report
        .rows()
        .iter()
        .find(|row| row.fact_kind() == ProjectionFactKind::DerivedScalarField)
        .expect("derived scalar support row should exist");
    assert!(matches!(
        row.posture(),
        ProjectionConsumptionSupportPosture::AdmittedWithWarnings(
            ProjectionConsumptionWarningKind::QueryContextPayloadBound
        )
    ));

    let declaration = declare_projection_consumption(
        source,
        test_binding(&["profile.display_name"]),
        ProjectMaterializedFacts::declare().derived_scalar_field("profile.display_name"),
    )
    .expect("query-context declaration should be valid");
    let eligibility = evaluate_projection_consumption_eligibility(&declaration);
    match eligibility {
        ProjectionConsumptionEligibility::AdmittedWithWarnings(_, warnings) => {
            assert_eq!(
                warnings.warning_kinds(),
                [ProjectionConsumptionWarningKind::QueryContextPayloadBound]
            );
        }
        other => panic!("unexpected eligibility posture: {other:?}"),
    }
}

#[test]
fn empty_projection_fact_request_is_rejected_before_declaration_exists() {
    let declaration = declare_projection_consumption(
        test_source(ProjectionSourceFamily::QueryReadReceipt),
        test_binding(&["identity.id"]),
        ProjectMaterializedFacts::declare(),
    );

    assert_eq!(
        declaration,
        Err(ProjectionConsumptionDeclarationError::NoRequestedFacts)
    );
}

#[test]
fn source_and_binding_result_shapes_must_match() {
    let declaration = declare_projection_consumption(
        test_source(ProjectionSourceFamily::QueryReadReceipt),
        ProjectionConsumptionBindingContext::test_only(
            "result-shape:other",
            "authorized-projection:test",
            vec!["identity.id".to_string()],
        ),
        ProjectMaterializedFacts::declare().entity_identities(),
    );

    assert_eq!(
        declaration,
        Err(
            ProjectionConsumptionDeclarationError::SourceBindingResultShapeMismatch {
                source_result_shape_digest: "result-shape:test".to_string(),
                binding_result_shape_digest: "result-shape:other".to_string(),
            }
        )
    );
}

#[test]
fn support_discovery_and_eligibility_stay_in_sync_for_all_phase_one_two_lanes() {
    for family in all_source_families() {
        let source = test_source(family);
        let report = discover_projection_consumption_support(&source);
        for fact_kind in ProjectionFactKind::all().iter().copied() {
            let row = report
                .rows()
                .iter()
                .find(|row| row.fact_kind() == fact_kind)
                .expect("support row should exist for every fact kind");
            let declaration = declare_projection_consumption(
                source.clone(),
                test_binding(&visible_fields_for_kind(fact_kind)),
                request_for_kind(fact_kind),
            )
            .expect("matrix declaration should be structurally valid");
            let eligibility = evaluate_projection_consumption_eligibility(&declaration);
            match (row.posture(), eligibility) {
                (
                    ProjectionConsumptionSupportPosture::Admitted,
                    ProjectionConsumptionEligibility::Admitted(_),
                ) => {}
                (
                    ProjectionConsumptionSupportPosture::AdmittedWithWarnings(expected),
                    ProjectionConsumptionEligibility::AdmittedWithWarnings(_, warnings),
                ) => assert_warning_matches_posture(&warnings, expected.clone()),
                (
                    ProjectionConsumptionSupportPosture::Deferred(expected_reason),
                    ProjectionConsumptionEligibility::Deferred(deferred),
                ) => assert_eq!(deferred.reason(), expected_reason),
                (
                    ProjectionConsumptionSupportPosture::SourceMismatch,
                    ProjectionConsumptionEligibility::SourceMismatch(mismatch),
                ) => {
                    assert_eq!(mismatch.source_family(), family);
                    assert_eq!(mismatch.requested_fact_kind(), fact_kind);
                }
                (posture, other) => {
                    panic!(
                        "support posture and eligibility diverged for family {family:?} fact {fact_kind:?}: posture {posture:?}, eligibility {other:?}"
                    );
                }
            }
        }
    }
}
