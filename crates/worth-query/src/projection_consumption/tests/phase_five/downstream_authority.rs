use super::super::super::{
    downstream_authority_closure_contract, load_projection_authority_contract_document,
    DownstreamAuthorityClosureRole, ExternalProjectionAuthorityContractDocument,
    ProjectMaterializedFacts, ProjectionAuthorityContract,
    ProjectionAuthorityContractDocumentErrorKind, ProjectionAuthorityOutcome,
};
use super::super::phase_four::support::{
    authorized_projection, read_result, read_result_shape, write_receipt,
};

#[test]
fn closure_contract_freezes_authority_and_deletion_roles() {
    let contract = downstream_authority_closure_contract();
    assert_eq!(contract.authoritative_width(), 7);
    assert_eq!(
        contract.deletion_obligations().collect::<Vec<_>>(),
        [
            "independently_pairable_completed_parts",
            "consumer_basis_compatibility_scan",
            "digest_to_authority_promotion",
            "raw_source_identity_reentry",
        ]
    );
    assert_eq!(
        contract
            .rows()
            .iter()
            .find(|row| row.component() == "evidence_identity")
            .expect("evidence row")
            .role(),
        DownstreamAuthorityClosureRole::DerivedEvidence
    );
}

#[test]
fn admitted_consumption_seals_one_structurally_replayable_authority() {
    let result = read_result();
    let result_shape = read_result_shape();
    let projection = authorized_projection(
        "query:test",
        result_shape.digest().as_str(),
        &["profile.display_name", "metrics.priority"],
    );
    let requested = || {
        ProjectMaterializedFacts::declare()
            .entity_identities()
            .display_field_path(
                crate::projection_consumption::projection_fact_field_path_from_segments([
                    worth_foundational::facade::FieldKey::new("profile").unwrap(),
                    worth_foundational::facade::FieldKey::new("display_name").unwrap(),
                ]),
            )
    };
    let left = result
        .consume_projection_facts(&result_shape, &projection, requested())
        .unwrap()
        .into_authority();
    let right = result
        .consume_projection_facts(&result_shape, &projection, requested())
        .unwrap()
        .into_authority();
    let left = left.authority().expect("authority should seal");
    let right = right.authority().expect("replayed authority should seal");

    assert!(left.structurally_equivalent(right));
    assert_eq!(left.counters().relationship_checks(), 10);
    assert_eq!(left.counters().requirement_checks(), 2);
    assert_eq!(left.counters().consumed_fact_visits(), 4);
    assert_eq!(left.counters().authority_constructions(), 1);
    assert_eq!(
        left.basis_authority(),
        right.basis_authority(),
        "replay must preserve exact nominal basis authority"
    );
}

#[test]
fn fluent_and_explicit_paths_share_one_transition() {
    let receipt = write_receipt();
    let projection = authorized_projection("query:test", "result-shape:test", &["identity.id"]);
    let contract = ProjectionAuthorityContract::declare()
        .require_settled_consumption()
        .require_source_authority()
        .require_target_identity()
        .require_source_references();
    let fluent = receipt
        .consume_projection_authority("result-shape:test", &projection, contract.clone())
        .unwrap();
    let explicit = receipt
        .consume_projection_facts(
            "result-shape:test",
            &projection,
            ProjectMaterializedFacts::declare()
                .target_identity()
                .source_references(),
        )
        .unwrap()
        .into_authority_with_contract(contract);
    let fluent = fluent.authority().expect("fluent authority");
    let explicit = explicit.authority().expect("explicit authority");

    assert!(fluent.structurally_equivalent(explicit));
    assert_eq!(fluent.consumer_contract().requirement_count(), 3);
    assert_eq!(fluent.consumer_contract().requested_fact_count(), 2);
    assert_eq!(fluent.counters().requirement_checks(), 3);
}

#[test]
fn serialized_contract_replays_through_the_same_authority_transition() {
    let receipt = write_receipt();
    let projection = authorized_projection("query:test", "result-shape:test", &["identity.id"]);
    let contract = ProjectionAuthorityContract::declare()
        .require_settled_consumption()
        .require_source_authority()
        .require_target_identity()
        .require_source_references();
    let document = contract
        .to_terminal_json_document()
        .expect("contract must serialize canonically");
    let replayed = load_projection_authority_contract_document(&document.to_external())
        .expect("canonical contract must load");
    assert_eq!(contract, replayed);

    let direct = receipt
        .consume_projection_authority("result-shape:test", &projection, contract)
        .unwrap();
    let replay = receipt
        .consume_projection_authority("result-shape:test", &projection, replayed)
        .unwrap();
    assert!(direct
        .authority()
        .expect("direct authority")
        .structurally_equivalent(replay.authority().expect("replayed authority")));
}

#[test]
fn external_contract_document_fails_closed_on_unknown_schema() {
    let error = load_projection_authority_contract_document(
        &ExternalProjectionAuthorityContractDocument::new(
            r#"{"schema":"foreign","requirements":[],"facts":[]}"#,
        ),
    )
    .expect_err("foreign schema must not become authority input");
    assert_eq!(
        error.kind(),
        ProjectionAuthorityContractDocumentErrorKind::SchemaMismatch
    );
}

#[test]
fn non_admitted_consumption_cannot_produce_partial_authority() {
    let result = read_result();
    let result_shape = read_result_shape();
    let projection = authorized_projection(
        "query:test",
        result_shape.digest().as_str(),
        &["metrics.priority"],
    );
    let outcome = result
        .consume_projection_facts(
            &result_shape,
            &projection,
            ProjectMaterializedFacts::declare().display_field_path(
                crate::projection_consumption::projection_fact_field_path_from_segments([
                    worth_foundational::facade::FieldKey::new("profile").unwrap(),
                    worth_foundational::facade::FieldKey::new("display_name").unwrap(),
                ]),
            ),
        )
        .unwrap()
        .into_authority();

    assert!(outcome.authority().is_none());
    assert!(matches!(
        outcome,
        ProjectionAuthorityOutcome::ConsumptionDenied(_)
    ));
}
