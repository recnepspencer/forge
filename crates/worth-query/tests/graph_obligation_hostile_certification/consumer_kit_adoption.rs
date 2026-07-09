use worth_query::facade::consumer_kit::{
    graph_obligation_consumer_kit, WorthQueryBoundaryAuditSourceSet,
    WorthQueryGraphObligationConsumerRegistrationDeclaration,
    WorthQueryGraphObligationLocalCeremonyAudit, WorthQueryGraphObligationResidueManifest,
    WorthQueryGraphObligationSelectorCoverageDeclaration, WorthQueryGraphObligationSupportPin,
};
use worth_query::facade::runtime::{
    WorthQueryGraphObligationKind, WorthQueryGraphObligationSupportLane,
    WorthQueryGraphTouchSelector,
};

use super::support::{
    authority_matrix, committed_world, graph_mutation_touch, registration_for_kind,
};

#[test]
fn consumer_kit_proves_adoption_without_local_ceremony() {
    let authority_budget = authority_matrix()
        .rows_for_lane(WorthQueryGraphObligationSupportLane::GraphComposition)
        .find(|row| row.obligation_kind() == WorthQueryGraphObligationKind::BlockingInvariant)
        .expect("blocking graph-composition row")
        .execution_budget()
        .clone();
    let registration = registration_for_kind(
        WorthQueryGraphObligationKind::BlockingInvariant,
        WorthQueryGraphTouchSelector::collection("topology.edge").unwrap(),
        WorthQueryGraphObligationSupportLane::GraphComposition,
    )
    .with_execution_budget(authority_budget.clone());

    let proof = graph_obligation_consumer_kit("phase-20-reference-consumer")
        .register_obligations(
            WorthQueryGraphObligationConsumerRegistrationDeclaration::for_runtime_family(
                "phase-20-reference-family",
                [registration],
            )
            .unwrap(),
        )
        .declare_selector_coverage(
            WorthQueryGraphObligationSelectorCoverageDeclaration::required([(
                "graph mutation selector",
                WorthQueryGraphTouchSelector::collection("topology.edge").unwrap(),
            )]),
        )
        .pin_support(WorthQueryGraphObligationSupportPin::supported_with_budget(
            [(
                WorthQueryGraphObligationKind::BlockingInvariant,
                WorthQueryGraphObligationSupportLane::GraphComposition,
                authority_budget,
            )],
        ))
        .against_support_matrix(authority_matrix())
        .audit_local_ceremony(WorthQueryGraphObligationLocalCeremonyAudit::evaluate(
            &WorthQueryBoundaryAuditSourceSet::new("phase-20-reference-consumer").source_file(
                "consumer.rs",
                "crates/reference/src/consumer.rs",
                "pub fn consumer_uses_query_graph_obligation_kit() {}",
            ),
        ))
        .account_for_residue(WorthQueryGraphObligationResidueManifest::empty())
        .prove_execution_with(&graph_mutation_touch(), &committed_world())
        .unwrap()
        .prove_adoption_with_execution()
        .unwrap();

    assert_eq!(
        proof.manifest().consumer_name(),
        "phase-20-reference-consumer"
    );
    assert!(proof.local_ceremony_audit().is_clean());
    assert_eq!(proof.residue_manifest().rows().len(), 0);
    assert_eq!(proof.in_memory_proof().selected_obligation_count(), 1);
    assert!(proof.execution_proof().has_real_executor_rows());
    assert!(proof.manifest().execution_proof_digest().is_some());
}
