use super::{
    advisory_plan, blocking_plan, capability_gap_block_plan, domain_invariant_summary,
    operating_context_block_plan, preflight_block_plan, schema_contract_block_plan,
};

#[test]
fn block_verdict_lowers_to_graph_composition_domain_invariant_denial() {
    let plan = blocking_plan();
    let denial = plan
        .graph_composition_domain_invariant_denial(domain_invariant_summary())
        .expect("block verdict lowers into graph-composition denial");

    assert_eq!(denial.invariant_family(), "topology:loop-wiring:v1");
    assert_eq!(
        denial.message(),
        "loop successor would break closed-loop continuity"
    );
    assert_eq!(
        denial.domain_invariant_summary().summary_digest(),
        domain_invariant_summary().summary_digest()
    );
}

#[test]
fn advisory_verdict_does_not_lower_to_blocking_domain_invariant_denial() {
    assert!(advisory_plan()
        .graph_composition_domain_invariant_denial(domain_invariant_summary())
        .is_none());
}

#[test]
fn schema_contract_validator_block_lowers_to_graph_composition_domain_invariant_denial() {
    assert!(schema_contract_block_plan()
        .graph_composition_domain_invariant_denial(domain_invariant_summary())
        .is_some());
}

#[test]
fn non_domain_invariant_block_kinds_do_not_lower_to_domain_invariant_denial() {
    for plan in [
        preflight_block_plan(),
        capability_gap_block_plan(),
        operating_context_block_plan(),
    ] {
        assert!(
            plan.graph_composition_domain_invariant_denial(domain_invariant_summary())
                .is_none(),
            "{} must not collapse into graph-composition domain invariant denial",
            plan.kind().as_str()
        );
    }
}
