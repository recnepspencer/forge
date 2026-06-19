use forge_query::facade::consumer_kit::{
    graph_obligation_consumer_kit, ForgeQueryBoundaryAuditSourceSet,
    ForgeQueryGraphObligationConsumerRegistrationDeclaration,
    ForgeQueryGraphObligationLocalCeremonyAudit, ForgeQueryGraphObligationResidueManifest,
    ForgeQueryGraphObligationSelectorCoverageDeclaration, ForgeQueryGraphObligationSupportPin,
};
use forge_query::facade::runtime::{
    ForgeQueryGraphObligationExecutionStatus, ForgeQueryGraphObligationKind,
    ForgeQueryGraphObligationOperatingWorldDescriptor,
    ForgeQueryGraphObligationOperatingWorldSelector, ForgeQueryGraphObligationRegistration,
    ForgeQueryGraphObligationRuleIdentity, ForgeQueryGraphObligationSupportLane,
    ForgeQueryGraphObligationSupportMatrix, ForgeQueryGraphObligationSupportPosture,
    ForgeQueryGraphTouchDescriptor, ForgeQueryGraphTouchReadVerb, ForgeQueryGraphTouchSelector,
    ForgeQueryMutationFamily,
};

#[test]
fn graph_obligation_consumer_kit_is_available_from_public_facade() {
    let registration = ForgeQueryGraphObligationRegistration::blocking_invariant(
        ForgeQueryGraphObligationRuleIdentity::new("worth.topo", "operator-scope", "1.0.0")
            .unwrap(),
        ForgeQueryGraphTouchSelector::collection("worth_topo").unwrap(),
        ForgeQueryGraphObligationOperatingWorldSelector::any_operating_world(),
    )
    .with_support_posture(ForgeQueryGraphObligationSupportPosture::supported(
        ForgeQueryGraphObligationSupportLane::AssemblyIndexSelection,
    ));

    let proof = graph_obligation_consumer_kit("worth-topo")
        .register_obligations(
            ForgeQueryGraphObligationConsumerRegistrationDeclaration::for_runtime_family(
                "worth-topo-operators",
                [registration],
            )
            .unwrap(),
        )
        .declare_selector_coverage(
            ForgeQueryGraphObligationSelectorCoverageDeclaration::required([(
                "operator catalog read coverage",
                ForgeQueryGraphTouchSelector::collection("worth_topo").unwrap(),
            )]),
        )
        .pin_support(ForgeQueryGraphObligationSupportPin::supported([(
            ForgeQueryGraphObligationKind::BlockingInvariant,
            ForgeQueryGraphObligationSupportLane::AssemblyIndexSelection,
        )]))
        .audit_local_ceremony(ForgeQueryGraphObligationLocalCeremonyAudit::evaluate(
            &ForgeQueryBoundaryAuditSourceSet::new("worth-topo").source_file(
                "operator-consumer.rs",
                "crates/worth-topo/src/operator_consumer.rs",
                "pub fn operator_consumer_uses_graph_obligation_kit() {}",
            ),
        ))
        .account_for_residue(ForgeQueryGraphObligationResidueManifest::empty())
        .prove_in_memory_selection(
            &ForgeQueryGraphTouchDescriptor::read_family(
                "worth_topo",
                [ForgeQueryGraphTouchReadVerb::ObservesCollection],
            )
            .unwrap(),
            &ForgeQueryGraphObligationOperatingWorldDescriptor::any_committed_authority(),
        )
        .unwrap()
        .prove_adoption()
        .unwrap();

    assert_eq!(proof.manifest().consumer_name(), "worth-topo");
}

#[test]
fn graph_obligation_consumer_kit_public_facade_proves_execution_backed_adoption() {
    let authority_budget =
        ForgeQueryGraphObligationSupportMatrix::milestone_9_9_authority_surface()
            .rows_for_lane(ForgeQueryGraphObligationSupportLane::GraphComposition)
            .find(|row| row.obligation_kind() == ForgeQueryGraphObligationKind::BlockingInvariant)
            .expect("blocking graph-composition row")
            .execution_budget()
            .clone();
    let registration = ForgeQueryGraphObligationRegistration::blocking_invariant(
        ForgeQueryGraphObligationRuleIdentity::new(
            "worth.topo",
            "public-facade-operator-scope",
            "1.0.0",
        )
        .unwrap(),
        ForgeQueryGraphTouchSelector::collection("topology.edge").unwrap(),
        ForgeQueryGraphObligationOperatingWorldSelector::any_operating_world(),
    )
    .with_support_posture(ForgeQueryGraphObligationSupportPosture::supported(
        ForgeQueryGraphObligationSupportLane::GraphComposition,
    ))
    .with_execution_budget(authority_budget.clone());

    let proof = graph_obligation_consumer_kit("worth-topo-public-facade")
        .register_obligations(
            ForgeQueryGraphObligationConsumerRegistrationDeclaration::for_runtime_family(
                "worth-topo-public-operators",
                [registration],
            )
            .unwrap(),
        )
        .declare_selector_coverage(
            ForgeQueryGraphObligationSelectorCoverageDeclaration::required([(
                "operator graph mutation coverage",
                ForgeQueryGraphTouchSelector::collection("topology.edge").unwrap(),
            )]),
        )
        .pin_support(ForgeQueryGraphObligationSupportPin::supported_with_budget(
            [(
                ForgeQueryGraphObligationKind::BlockingInvariant,
                ForgeQueryGraphObligationSupportLane::GraphComposition,
                authority_budget,
            )],
        ))
        .against_support_matrix(
            ForgeQueryGraphObligationSupportMatrix::milestone_9_9_authority_surface(),
        )
        .audit_local_ceremony(ForgeQueryGraphObligationLocalCeremonyAudit::evaluate(
            &ForgeQueryBoundaryAuditSourceSet::new("worth-topo-public-facade").source_file(
                "operator-consumer.rs",
                "crates/worth-topo/src/operator_consumer.rs",
                "pub fn operator_consumer_uses_graph_obligation_kit() {}",
            ),
        ))
        .account_for_residue(ForgeQueryGraphObligationResidueManifest::empty())
        .prove_execution_with(
            &ForgeQueryGraphTouchDescriptor::declared_mutation_collection(
                "topology.edge",
                ForgeQueryMutationFamily::Update,
                None,
                ["set:capacity"],
                ["capacity"],
            )
            .unwrap(),
            &ForgeQueryGraphObligationOperatingWorldDescriptor::any_committed_authority(),
        )
        .unwrap()
        .prove_adoption_with_execution()
        .unwrap();

    let execution_proof = proof.execution_proof().expect("execution proof");
    let execution_proof_digest = proof
        .manifest()
        .execution_proof_digest()
        .expect("manifest execution proof digest");

    assert_eq!(proof.manifest().consumer_name(), "worth-topo-public-facade");
    assert_eq!(execution_proof_digest, execution_proof.proof_digest());
    assert!(execution_proof.has_real_executor_rows());
    assert_eq!(
        execution_proof.execution_statuses(),
        vec![ForgeQueryGraphObligationExecutionStatus::Executed]
    );
    assert_eq!(proof.in_memory_proof().selected_obligation_count(), 1);
    assert!(proof.local_ceremony_audit().is_clean());
    assert_eq!(proof.residue_manifest().rows().len(), 0);
}
