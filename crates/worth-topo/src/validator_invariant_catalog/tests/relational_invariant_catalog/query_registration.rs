use forge_query::facade::{
    ForgeQueryGraphObligationRegistrationCatalog, ForgeQueryRuntime, ForgeQueryRuntimeError,
};
use forge_relational::facade::runtime::RelationalRuntimeApi;

use super::execution_inputs::relational_invariant_closeout;

#[test]
fn query_registration_artifact_projection_uses_query_owned_public_lane() {
    let closeout = relational_invariant_closeout();
    let artifact = closeout.query_registration_artifact();

    assert_eq!(artifact.lane(), "query_invariant_catalog_registration");
    assert!(artifact
        .semantic_code()
        .contains("relational_invariant_catalog"));
    assert!(artifact.detail().contains("Query artifact"));
    assert_eq!(
        artifact.query_graph_obligation_registration_count(),
        closeout
            .counters()
            .query_graph_obligation_registration_count()
    );
    assert_eq!(
        artifact.relational_invariant_family_count(),
        closeout.counters().invariant_family_count()
    );
    assert_eq!(
        artifact.graph_scoped_custom_invariant_count(),
        closeout.counters().invariant_family_count()
    );
    assert!(artifact
        .graph_scoped_custom_invariant_rows()
        .iter()
        .all(|row| !row.graph_obligation_registration_digest().is_empty()
            && !row.custom_rule_id().is_empty()
            && !row.execution_point().is_empty()));
    assert!(!artifact.query_materialization_digest().is_empty());
}

#[test]
fn query_registration_bundle_materializes_query_graph_obligation_catalog() {
    let closeout = relational_invariant_closeout();
    let bundle = closeout.query_registration_bundle();
    let catalog = ForgeQueryGraphObligationRegistrationCatalog::from_registrations(
        bundle.graph_obligation_registrations().to_vec(),
    )
    .expect("Worth relational invariant bundle should be valid Query graph authority");

    assert_eq!(
        catalog.registration_count(),
        closeout.counters().invariant_family_count()
    );
    assert_eq!(
        catalog.registration_count(),
        bundle.graph_scoped_custom_invariant_count()
    );
    assert!(catalog.registrations().iter().all(|registration| {
        bundle
            .graph_obligation_registrations()
            .iter()
            .any(|expected| expected.registration_digest() == registration.registration_digest())
    }));
}

#[test]
fn query_builder_rejects_mixed_query_owned_invariants_and_relational_runtime_authority() {
    let closeout = relational_invariant_closeout();
    let bundle = closeout.query_registration_bundle();
    let mut builder = ForgeQueryRuntime::builder()
        .invariant_registration_artifact(bundle.artifact().clone())
        .relational_runtime(RelationalRuntimeApi::builder().build());
    for registration in bundle.graph_scoped_custom_invariants() {
        builder = builder.graph_scoped_custom_invariant(registration.clone());
    }
    let error = match builder.build_backend_from_parts().build() {
        Ok(_) => panic!(
            "mixed Query-owned invariant registrations and explicit relational runtime should fail"
        ),
        Err(error) => error,
    };

    match error {
        ForgeQueryRuntimeError::InvariantRegistration { stage, message } => {
            assert_eq!(stage, "relational_runtime_authority_selection");
            assert!(message.contains("explicitly supplied relational runtime"));
            assert!(message.contains("choose one authority path"));
        }
        other => panic!("unexpected runtime error: {other:?}"),
    }
}
