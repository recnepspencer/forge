use crate::projection_consumption::{
    ProjectMaterializedFacts, ProjectionFactConsumptionAttempt, ProjectionSourceFamily,
};

use super::support::{
    authorized_projection, live_struct_binding, profile_struct_value, retained_struct_binding,
    shared_test_result_shape, test_result_shape_artifact, test_result_shape_canonical_digest,
};

#[test]
fn retained_derived_consumption_preserves_complete_struct_values() {
    let binding = retained_struct_binding();
    let attempt = binding
        .consume_projection_facts(
            &test_result_shape_artifact("result-shape:test"),
            &authorized_projection(
                "query:test",
                &test_result_shape_canonical_digest("result-shape:test"),
                &["profile"],
            ),
            ProjectMaterializedFacts::declare().derived_field_path(profile_path()),
        )
        .unwrap();
    let ProjectionFactConsumptionAttempt::Admitted(completed) = attempt else {
        panic!("retained struct consumption must admit")
    };

    assert_eq!(
        completed.facts().derived_fields()[0].as_struct().unwrap(),
        &profile_struct_value()
    );
    assert_eq!(
        completed.source_family(),
        ProjectionSourceFamily::RetainedDerivedArtifactBinding
    );
}

#[test]
fn live_artifact_consumption_preserves_complete_struct_values() {
    let binding = live_struct_binding();
    let attempt = binding
        .consume_projection_facts(
            &shared_test_result_shape().identity,
            &authorized_projection(
                "query:test",
                &shared_test_result_shape().digest,
                &["profile"],
            ),
            ProjectMaterializedFacts::declare().derived_field_path(profile_path()),
        )
        .unwrap();
    let ProjectionFactConsumptionAttempt::Admitted(completed) = attempt else {
        panic!("live struct consumption must admit")
    };

    assert_eq!(
        completed.facts().derived_fields()[0].as_struct().unwrap(),
        &profile_struct_value()
    );
    assert_eq!(
        completed.source_family(),
        ProjectionSourceFamily::LiveArtifactBinding
    );
}

fn profile_path() -> crate::projection_consumption::ProjectionFactFieldPath {
    crate::projection_consumption::projection_fact_field_path_from_segments([
        worth_foundational::facade::FieldKey::new("profile").unwrap(),
    ])
}
