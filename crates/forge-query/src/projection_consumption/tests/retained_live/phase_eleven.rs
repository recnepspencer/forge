use crate::projection_consumption::{
    evaluate_projection_consumption_eligibility, ProjectMaterializedFacts,
    ProjectionConsumptionDenialReason, ProjectionConsumptionEligibility,
    ProjectionConsumptionSupportPosture, ProjectionFactConsumptionAttempt, ProjectionFactKind,
    ProjectionSourceFamily,
};

use super::support::{
    authorized_projection, live_binding, request_for_kind, retained_binding,
    visible_fields_for_kind,
};

fn assert_support_and_eligibility_sync_for_retained_binding() {
    let support = retained_binding().discover_projection_fact_consumption_support();

    for fact_kind in ProjectionFactKind::all().iter().copied() {
        let support_row = support
            .rows()
            .iter()
            .find(|row| row.fact_kind() == fact_kind)
            .expect("support row should exist");
        let declaration = retained_binding()
            .declare_projection_fact_consumption(
                "result-shape:test",
                &authorized_projection(
                    "query:test",
                    "result-shape:test",
                    &visible_fields_for_kind(fact_kind),
                ),
                request_for_kind(fact_kind),
            )
            .expect("retained declaration should remain structurally valid");
        let eligibility = evaluate_projection_consumption_eligibility(&declaration);

        match (support_row.posture(), eligibility) {
            (
                ProjectionConsumptionSupportPosture::Admitted,
                ProjectionConsumptionEligibility::Admitted(_),
            ) => {}
            (
                ProjectionConsumptionSupportPosture::SourceMismatch,
                ProjectionConsumptionEligibility::SourceMismatch(mismatch),
            ) => {
                assert_eq!(
                    mismatch.source_family(),
                    ProjectionSourceFamily::RetainedDerivedArtifactBinding
                );
                assert_eq!(mismatch.requested_fact_kind(), fact_kind);
            }
            (posture, other) => {
                panic!(
                    "retained support posture and eligibility diverged for fact {fact_kind:?}: posture {posture:?}, eligibility {other:?}"
                );
            }
        }
    }
}

fn assert_support_and_eligibility_sync_for_live_binding() {
    let support = live_binding().discover_projection_fact_consumption_support();

    for fact_kind in ProjectionFactKind::all().iter().copied() {
        let support_row = support
            .rows()
            .iter()
            .find(|row| row.fact_kind() == fact_kind)
            .expect("support row should exist");
        let declaration = live_binding()
            .declare_projection_fact_consumption(
                "result-shape:test",
                &authorized_projection(
                    "query:test",
                    "result-shape:test",
                    &visible_fields_for_kind(fact_kind),
                ),
                request_for_kind(fact_kind),
            )
            .expect("live declaration should remain structurally valid");
        let eligibility = evaluate_projection_consumption_eligibility(&declaration);

        match (support_row.posture(), eligibility) {
            (
                ProjectionConsumptionSupportPosture::Admitted,
                ProjectionConsumptionEligibility::Admitted(_),
            ) => {}
            (
                ProjectionConsumptionSupportPosture::SourceMismatch,
                ProjectionConsumptionEligibility::SourceMismatch(mismatch),
            ) => {
                assert_eq!(
                    mismatch.source_family(),
                    ProjectionSourceFamily::LiveArtifactBinding
                );
                assert_eq!(mismatch.requested_fact_kind(), fact_kind);
            }
            (posture, other) => {
                panic!(
                    "live support posture and eligibility diverged for fact {fact_kind:?}: posture {posture:?}, eligibility {other:?}"
                );
            }
        }
    }
}

#[test]
fn retained_binding_common_path_consumes_admitted_field_and_source_reference_facts() {
    let binding = retained_binding();

    let attempt = binding
        .consume_projection_facts(
            "result-shape:test",
            &authorized_projection("query:test", "result-shape:test", &["profile.display_name"]),
            ProjectMaterializedFacts::declare()
                .view_local_identities()
                .display_field("profile.display_name")
                .source_references(),
        )
        .expect("retained binding consumption should succeed");

    match attempt {
        ProjectionFactConsumptionAttempt::Admitted(completed) => {
            assert_eq!(
                completed.source_family(),
                ProjectionSourceFamily::RetainedDerivedArtifactBinding
            );
            assert_eq!(completed.facts().view_local_identities().len(), 3);
            assert_eq!(completed.facts().display_fields().len(), 3);
            assert_eq!(completed.facts().source_references().len(), 2);
            assert_eq!(
                completed.projection_consumption_envelope().source_family(),
                ProjectionSourceFamily::RetainedDerivedArtifactBinding
            );
        }
        other => panic!("expected admitted retained binding consumption, got {other:?}"),
    }
}

#[test]
fn live_binding_common_path_consumes_entity_identity_field_and_source_reference_facts() {
    let binding = live_binding();

    let attempt = binding
        .consume_projection_facts(
            "result-shape:test",
            &authorized_projection("query:test", "result-shape:test", &["profile.display_name"]),
            ProjectMaterializedFacts::declare()
                .entity_identities()
                .view_local_identities()
                .display_field("profile.display_name")
                .source_references(),
        )
        .expect("live binding consumption should succeed");

    match attempt {
        ProjectionFactConsumptionAttempt::Admitted(completed) => {
            assert_eq!(
                completed.source_family(),
                ProjectionSourceFamily::LiveArtifactBinding
            );
            assert_eq!(completed.facts().entity_identities().len(), 3);
            assert_eq!(completed.facts().view_local_identities().len(), 3);
            assert_eq!(completed.facts().display_fields().len(), 3);
            assert_eq!(completed.facts().source_references().len(), 2);
            assert_eq!(completed.receipt().extracted_fact_count(), 11);
        }
        other => panic!("expected admitted live binding consumption, got {other:?}"),
    }
}

#[test]
fn retained_binding_missing_declared_field_evidence_fails_extraction_honestly() {
    let binding = retained_binding();

    let error = binding
        .consume_projection_facts(
            "result-shape:test",
            &authorized_projection("query:test", "result-shape:test", &["metrics.priority"]),
            ProjectMaterializedFacts::declare().display_field("metrics.priority"),
        )
        .expect_err("retained binding should reject missing field evidence");

    let message = format!("{error:?}");
    assert!(message.contains("MissingDeclaredFieldEvidence"));
    assert!(message.contains("metrics.priority"));
}

#[test]
fn live_binding_missing_declared_field_evidence_fails_extraction_honestly() {
    let binding = live_binding();

    let error = binding
        .consume_projection_facts(
            "result-shape:test",
            &authorized_projection("query:test", "result-shape:test", &["metrics.priority"]),
            ProjectMaterializedFacts::declare().display_field("metrics.priority"),
        )
        .expect_err("live binding should reject missing field evidence");

    let message = format!("{error:?}");
    assert!(message.contains("MissingDeclaredFieldEvidence"));
    assert!(message.contains("metrics.priority"));
}

#[test]
fn retained_and_live_support_reports_match_phase_eleven_family_boundaries() {
    let retained = retained_binding().discover_projection_fact_consumption_support();
    let live = live_binding().discover_projection_fact_consumption_support();

    assert!(matches!(
        retained
            .rows()
            .iter()
            .find(|row| row.fact_kind() == ProjectionFactKind::SourceReference)
            .expect("retained source reference row should exist")
            .posture(),
        ProjectionConsumptionSupportPosture::Admitted
    ));
    assert!(matches!(
        retained
            .rows()
            .iter()
            .find(|row| row.fact_kind() == ProjectionFactKind::EntityIdentity)
            .expect("retained entity identity row should exist")
            .posture(),
        ProjectionConsumptionSupportPosture::SourceMismatch
    ));
    assert!(matches!(
        live.rows()
            .iter()
            .find(|row| row.fact_kind() == ProjectionFactKind::EntityIdentity)
            .expect("live entity identity row should exist")
            .posture(),
        ProjectionConsumptionSupportPosture::Admitted
    ));
}

#[test]
fn retained_and_live_support_and_eligibility_stay_in_sync_for_all_fact_kinds() {
    assert_support_and_eligibility_sync_for_retained_binding();
    assert_support_and_eligibility_sync_for_live_binding();
}

#[test]
fn retained_and_live_common_path_preserve_receipt_and_envelope_identity() {
    let retained_attempt = retained_binding()
        .consume_projection_facts(
            "result-shape:test",
            &authorized_projection("query:test", "result-shape:test", &["profile.display_name"]),
            ProjectMaterializedFacts::declare()
                .view_local_identities()
                .display_field("profile.display_name")
                .source_references(),
        )
        .expect("retained binding consumption should succeed");
    let live_attempt = live_binding()
        .consume_projection_facts(
            "result-shape:test",
            &authorized_projection("query:test", "result-shape:test", &["profile.display_name"]),
            ProjectMaterializedFacts::declare()
                .entity_identities()
                .display_field("profile.display_name")
                .source_references(),
        )
        .expect("live binding consumption should succeed");

    for (expected_family, expected_extracted_count, attempt) in [
        (
            ProjectionSourceFamily::RetainedDerivedArtifactBinding,
            8usize,
            retained_attempt,
        ),
        (
            ProjectionSourceFamily::LiveArtifactBinding,
            8usize,
            live_attempt,
        ),
    ] {
        let completed = attempt.completed().expect("attempt should be admitted");
        let receipt = completed.receipt();
        let envelope = completed.projection_consumption_envelope();

        assert_eq!(receipt.source_family(), expected_family);
        assert_eq!(envelope.source_family(), expected_family);
        assert_eq!(envelope.source_identity(), receipt.source_identity());
        assert_eq!(receipt.extracted_fact_count(), expected_extracted_count);
        assert_eq!(envelope.extracted_fact_count(), expected_extracted_count);
        assert_eq!(
            envelope.sources().receipt_digest(),
            receipt.receipt_digest()
        );
        assert_eq!(
            envelope.sources().fact_set_digest(),
            receipt.fact_set_digest()
        );
        assert!(!receipt.integrity_digest().is_empty());
        assert!(!envelope.envelope_digest().is_empty());
    }
}

#[test]
fn retained_and_live_common_path_keep_visibility_denial_on_hidden_fields() {
    let retained_attempt = retained_binding()
        .consume_projection_facts(
            "result-shape:test",
            &authorized_projection("query:test", "result-shape:test", &[]),
            ProjectMaterializedFacts::declare().display_field("profile.display_name"),
        )
        .expect("retained declaration path should succeed");
    let live_attempt = live_binding()
        .consume_projection_facts(
            "result-shape:test",
            &authorized_projection("query:test", "result-shape:test", &[]),
            ProjectMaterializedFacts::declare().display_field("profile.display_name"),
        )
        .expect("live declaration path should succeed");

    for attempt in [retained_attempt, live_attempt] {
        match attempt {
            ProjectionFactConsumptionAttempt::Denied(denied) => {
                assert_eq!(
                    denied.reason(),
                    &ProjectionConsumptionDenialReason::FactFamilyNotVisible {
                        field_key: "profile.display_name".to_string(),
                    }
                );
            }
            other => panic!("expected visibility denial, got {other:?}"),
        }
    }
}
