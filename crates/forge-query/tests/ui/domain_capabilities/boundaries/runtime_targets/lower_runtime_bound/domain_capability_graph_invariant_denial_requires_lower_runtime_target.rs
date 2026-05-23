use forge_query::facade::{
    admit_eligible_domain_capability_contribution, evaluate_requested_domain_capability_contribution,
    materialize_graph_composition_domain_invariant_denial,
    prepare_admitted_domain_capability_contribution_for_materialization,
    ForgeQueryIntentDeclaration, ForgeQueryInvariantCapabilityContributionAuthoring,
};

fn main() {
    let declaration = ForgeQueryIntentDeclaration::strategy_commit(
        "test.intent",
        "test.strategy",
        "1",
        "test.contract",
        serde_json::json!({ "entity": "edge:42" }),
    );
    let requested = ForgeQueryInvariantCapabilityContributionAuthoring::graph_invariant_denial(
        "spatial.non_manifold_edge_split",
        ["edges"],
        ["edge:42"],
        ["mixed_existing_and_symbolic_entity_identity_edges"],
        ["mixed_existing_target_followup_mutation"],
        "program-graph-1",
        "breadth-graph-1",
        "components=1;symbolic_entities=1;symbolic_relations=0;declared_collections=1;declared_symbols=1;target_combinations=1;lifecycle_families=1",
        "spatial.non_manifold_edge_split",
        "result would introduce non-manifold topology",
    )
    .for_intent_declaration(&declaration);
    let eligible = match evaluate_requested_domain_capability_contribution(requested) {
        forge_proof::TransitionOutcome::Success(value) => value,
        _ => unreachable!(),
    };
    let admitted = match admit_eligible_domain_capability_contribution(eligible) {
        forge_proof::TransitionOutcome::Success(value) => value,
        _ => unreachable!(),
    };
    let target = admitted.payload().target().clone();
    let ready = match prepare_admitted_domain_capability_contribution_for_materialization(
        admitted,
        target,
    ) {
        forge_proof::TransitionOutcome::Success(value) => value,
        _ => unreachable!(),
    };

    let _ = materialize_graph_composition_domain_invariant_denial(ready);
}
