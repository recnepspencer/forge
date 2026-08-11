use crate::projection_consumption::{
    ProjectMaterializedFacts, ProjectionConsumptionDenialReason, ProjectionFactConsumptionAttempt,
};

use super::super::support::{
    authorized_projection, live_binding, retained_binding, shared_test_result_shape,
    test_result_shape_artifact, test_result_shape_canonical_digest,
};

#[test]
fn retained_binding_missing_declared_field_evidence_fails_extraction_honestly() {
    let binding = retained_binding();

    let error = binding
        .consume_projection_facts(
            &test_result_shape_artifact("result-shape:test"),
            &authorized_projection(
                "query:test",
                &test_result_shape_canonical_digest("result-shape:test"),
                &["metrics.priority"],
            ),
            ProjectMaterializedFacts::declare().display_field_path(
                crate::projection_consumption::projection_fact_field_path_from_segments([
                    worth_foundational::facade::FieldKey::new("metrics")
                        .expect("projection fact field segment should admit"),
                    worth_foundational::facade::FieldKey::new("priority")
                        .expect("projection fact field segment should admit"),
                ]),
            ),
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
            &shared_test_result_shape().identity,
            &authorized_projection(
                "query:test",
                &shared_test_result_shape().digest,
                &["metrics.priority"],
            ),
            ProjectMaterializedFacts::declare().display_field_path(
                crate::projection_consumption::projection_fact_field_path_from_segments([
                    worth_foundational::facade::FieldKey::new("metrics")
                        .expect("projection fact field segment should admit"),
                    worth_foundational::facade::FieldKey::new("priority")
                        .expect("projection fact field segment should admit"),
                ]),
            ),
        )
        .expect_err("live binding should reject missing field evidence");

    let message = format!("{error:?}");
    assert!(message.contains("MissingDeclaredFieldEvidence"));
    assert!(message.contains("metrics.priority"));
}

#[test]
fn retained_and_live_common_path_keep_visibility_denial_on_hidden_fields() {
    let retained_attempt = retained_binding()
        .consume_projection_facts(
            &test_result_shape_artifact("result-shape:test"),
            &authorized_projection(
                "query:test",
                &test_result_shape_canonical_digest("result-shape:test"),
                &[],
            ),
            ProjectMaterializedFacts::declare().display_field_path(
                crate::projection_consumption::projection_fact_field_path_from_segments([
                    worth_foundational::facade::FieldKey::new("profile")
                        .expect("projection fact field segment should admit"),
                    worth_foundational::facade::FieldKey::new("display_name")
                        .expect("projection fact field segment should admit"),
                ]),
            ),
        )
        .expect("retained declaration path should succeed");
    let live_attempt = live_binding()
        .consume_projection_facts(
            &shared_test_result_shape().identity,
            &authorized_projection("query:test", &shared_test_result_shape().digest, &[]),
            ProjectMaterializedFacts::declare().display_field_path(
                crate::projection_consumption::projection_fact_field_path_from_segments([
                    worth_foundational::facade::FieldKey::new("profile")
                        .expect("projection fact field segment should admit"),
                    worth_foundational::facade::FieldKey::new("display_name")
                        .expect("projection fact field segment should admit"),
                ]),
            ),
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
