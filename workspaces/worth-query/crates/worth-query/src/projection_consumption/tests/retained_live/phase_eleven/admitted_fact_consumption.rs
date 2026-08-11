use crate::projection_consumption::{
    ProjectMaterializedFacts, ProjectionFactConsumptionAttempt, ProjectionSourceFamily,
};

use super::super::support::{
    authorized_projection, live_binding, retained_binding, shared_test_result_shape,
    test_result_shape_artifact, test_result_shape_canonical_digest,
};

#[test]
fn retained_binding_common_path_consumes_admitted_field_and_source_reference_facts() {
    let binding = retained_binding();

    let attempt = binding
        .consume_projection_facts(
            &test_result_shape_artifact("result-shape:test"),
            &authorized_projection(
                "query:test",
                &test_result_shape_canonical_digest("result-shape:test"),
                &["profile.display_name"],
            ),
            ProjectMaterializedFacts::declare()
                .view_local_identities()
                .display_field_path(
                    crate::projection_consumption::projection_fact_field_path_from_segments([
                        worth_foundational::facade::FieldKey::new("profile")
                            .expect("projection fact field segment should admit"),
                        worth_foundational::facade::FieldKey::new("display_name")
                            .expect("projection fact field segment should admit"),
                    ]),
                )
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
            &shared_test_result_shape().identity,
            &authorized_projection(
                "query:test",
                &shared_test_result_shape().digest,
                &["profile.display_name"],
            ),
            ProjectMaterializedFacts::declare()
                .entity_identities()
                .view_local_identities()
                .display_field_path(
                    crate::projection_consumption::projection_fact_field_path_from_segments([
                        worth_foundational::facade::FieldKey::new("profile")
                            .expect("projection fact field segment should admit"),
                        worth_foundational::facade::FieldKey::new("display_name")
                            .expect("projection fact field segment should admit"),
                    ]),
                )
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
fn retained_and_live_common_path_preserve_receipt_and_envelope_identity() {
    let retained_attempt = retained_binding()
        .consume_projection_facts(
            &test_result_shape_artifact("result-shape:test"),
            &authorized_projection(
                "query:test",
                &test_result_shape_canonical_digest("result-shape:test"),
                &["profile.display_name"],
            ),
            ProjectMaterializedFacts::declare()
                .view_local_identities()
                .display_field_path(
                    crate::projection_consumption::projection_fact_field_path_from_segments([
                        worth_foundational::facade::FieldKey::new("profile")
                            .expect("projection fact field segment should admit"),
                        worth_foundational::facade::FieldKey::new("display_name")
                            .expect("projection fact field segment should admit"),
                    ]),
                )
                .source_references(),
        )
        .expect("retained binding consumption should succeed");
    let live_attempt = live_binding()
        .consume_projection_facts(
            &shared_test_result_shape().identity,
            &authorized_projection(
                "query:test",
                &shared_test_result_shape().digest,
                &["profile.display_name"],
            ),
            ProjectMaterializedFacts::declare()
                .entity_identities()
                .display_field_path(
                    crate::projection_consumption::projection_fact_field_path_from_segments([
                        worth_foundational::facade::FieldKey::new("profile")
                            .expect("projection fact field segment should admit"),
                        worth_foundational::facade::FieldKey::new("display_name")
                            .expect("projection fact field segment should admit"),
                    ]),
                )
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
