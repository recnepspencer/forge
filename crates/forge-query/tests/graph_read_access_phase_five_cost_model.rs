use forge_query::facade::runtime::ForgeQueryGraphReadAccessRequirementKind;
use forge_query::facade::runtime::{
    derive_graph_read_cost_evidence, estimate_graph_read_access_cost,
    estimate_graph_read_access_cost_with_planning_observation,
    explain_graph_read_access_requirements_for_family, ForgeQueryGraphReadBudget,
    ForgeQueryGraphReadBudgetClassKind, ForgeQueryGraphReadComplexityContractKind,
    ForgeQueryGraphReadCostEstimateStatusKind, ForgeQueryGraphReadInlineEphemeralAllowanceKind,
};

#[allow(dead_code)]
mod graph_read_access_cost_model_support;
mod support;

use graph_read_access_cost_model_support::{
    assert_exact_bucket_contribution, bucket_sum, dense_traversal_family, frontier_search_family,
    intermediate_pressure_family, reordered_simple_traversal_family, simple_traversal_family,
    workspace,
};

#[test]
fn dense_boolean_traversal_exceeds_inline_ephemeral_budget_conservatively() {
    let mut workspace = workspace("graph-read-access.phase-five.dense-budget");
    let family = dense_traversal_family(&mut workspace, "phase-five-dense-budget");
    let requirements = explain_graph_read_access_requirements_for_family(&family)
        .expect("requirements should derive");

    let evidence = derive_graph_read_cost_evidence(&requirements);
    let estimate = estimate_graph_read_access_cost(&requirements, evidence);
    let budget = ForgeQueryGraphReadBudget::inline_ephemeral_default();
    let budget_check = budget.check_supported_cost(&estimate);

    assert_eq!(
        estimate.status().kind(),
        &ForgeQueryGraphReadCostEstimateStatusKind::UnknownConservative
    );
    assert_eq!(
        estimate.complexity_contract().kind(),
        &ForgeQueryGraphReadComplexityContractKind::BroadTraversalCandidate
    );
    assert_eq!(
        budget_check.class().kind(),
        &ForgeQueryGraphReadBudgetClassKind::ExceedsInlineEphemeralBudget
    );
    assert_eq!(
        budget_check.inline_ephemeral_allowance().kind(),
        &ForgeQueryGraphReadInlineEphemeralAllowanceKind::Rejected
    );
    assert_eq!(
        budget_check.cost_estimate_digest(),
        estimate.digest().as_str()
    );
}

#[test]
fn equivalent_access_requirements_produce_identical_cost_estimate_digests() {
    let mut workspace = workspace("graph-read-access.phase-five.equivalence");
    let first = simple_traversal_family(&mut workspace, "phase-five-equivalence-a");
    let second = reordered_simple_traversal_family(&mut workspace, "phase-five-equivalence-b");
    let first_requirements = explain_graph_read_access_requirements_for_family(&first)
        .expect("first requirements should derive");
    let second_requirements = explain_graph_read_access_requirements_for_family(&second)
        .expect("second requirements should derive");

    let first_evidence = derive_graph_read_cost_evidence(&first_requirements);
    let second_evidence = derive_graph_read_cost_evidence(&second_requirements);
    let first_estimate = estimate_graph_read_access_cost(&first_requirements, first_evidence);
    let second_estimate = estimate_graph_read_access_cost(&second_requirements, second_evidence);

    assert_eq!(first_requirements.digest(), second_requirements.digest());
    assert_eq!(
        first_estimate.attribution_rows(),
        second_estimate.attribution_rows()
    );
    assert_eq!(
        first_estimate.digest().as_str(),
        second_estimate.digest().as_str()
    );
}

#[test]
fn memory_estimate_names_each_relevant_access_structure_bucket() {
    let mut workspace = workspace("graph-read-access.phase-five.memory-buckets");
    let family = frontier_search_family(&mut workspace, "phase-five-memory-buckets");
    let requirements = explain_graph_read_access_requirements_for_family(&family)
        .expect("requirements should derive");
    let estimate = estimate_graph_read_access_cost(
        &requirements,
        derive_graph_read_cost_evidence(&requirements),
    );
    let memory = estimate.supported().memory();

    assert_eq!(
        memory.adjacency_bytes(),
        bucket_sum(&estimate, |memory| memory.adjacency_bytes())
    );
    assert_eq!(
        memory.reverse_adjacency_bytes(),
        bucket_sum(&estimate, |memory| memory.reverse_adjacency_bytes())
    );
    assert_eq!(
        memory.frontier_bytes(),
        bucket_sum(&estimate, |memory| memory.frontier_bytes())
    );
    assert_eq!(
        memory.visited_bytes(),
        bucket_sum(&estimate, |memory| memory.visited_bytes())
    );
    assert_eq!(
        memory.dedup_bytes(),
        bucket_sum(&estimate, |memory| memory.dedup_bytes())
    );
    assert_eq!(
        memory.predicate_bytes(),
        bucket_sum(&estimate, |memory| memory.predicate_bytes())
    );
    assert_eq!(
        memory.ordering_bytes(),
        bucket_sum(&estimate, |memory| memory.ordering_bytes())
    );
    assert_eq!(
        memory.proof_bytes(),
        bucket_sum(&estimate, |memory| memory.proof_bytes())
    );
    assert_eq!(
        memory.result_bytes(),
        bucket_sum(&estimate, |memory| memory.result_bytes())
    );

    assert_exact_bucket_contribution(
        &estimate,
        ForgeQueryGraphReadAccessRequirementKind::DirectionalAdjacency,
        |memory| memory.adjacency_bytes(),
        512,
    );
    assert_exact_bucket_contribution(
        &estimate,
        ForgeQueryGraphReadAccessRequirementKind::ReverseAdjacency,
        |memory| memory.reverse_adjacency_bytes(),
        512,
    );
    assert_exact_bucket_contribution(
        &estimate,
        ForgeQueryGraphReadAccessRequirementKind::TraversalWorkset,
        |memory| memory.frontier_bytes(),
        512,
    );
    assert_exact_bucket_contribution(
        &estimate,
        ForgeQueryGraphReadAccessRequirementKind::VisitedSet,
        |memory| memory.visited_bytes(),
        384,
    );
    assert_exact_bucket_contribution(
        &estimate,
        ForgeQueryGraphReadAccessRequirementKind::DedupSet,
        |memory| memory.dedup_bytes(),
        384,
    );
    assert_exact_bucket_contribution(
        &estimate,
        ForgeQueryGraphReadAccessRequirementKind::PredicateSupport,
        |memory| memory.predicate_bytes(),
        512,
    );
    assert_exact_bucket_contribution(
        &estimate,
        ForgeQueryGraphReadAccessRequirementKind::OrderingSupport,
        |memory| memory.ordering_bytes(),
        512,
    );
    assert_exact_bucket_contribution(
        &estimate,
        ForgeQueryGraphReadAccessRequirementKind::ProofSupport,
        |memory| memory.proof_bytes(),
        256,
    );
    assert_exact_bucket_contribution(
        &estimate,
        ForgeQueryGraphReadAccessRequirementKind::ResultBuffer,
        |memory| memory.result_bytes(),
        1024,
    );
}

#[test]
fn intermediate_set_pressure_marks_broad_even_when_index_bytes_fit() {
    let mut workspace = workspace("graph-read-access.phase-five.intermediate-broadness");
    let family = intermediate_pressure_family(&mut workspace, "phase-five-intermediate-broadness");
    let requirements = explain_graph_read_access_requirements_for_family(&family)
        .expect("requirements should derive");
    let estimate = estimate_graph_read_access_cost(
        &requirements,
        derive_graph_read_cost_evidence(&requirements),
    );

    assert!(
        estimate.supported().index_bytes()
            <= ForgeQueryGraphReadBudget::inline_ephemeral_default().max_inline_index_bytes()
    );
    assert!(
        estimate.intrinsic().intermediate_set_size()
            > ForgeQueryGraphReadBudget::inline_ephemeral_default()
                .max_inline_intermediate_set_size()
    );
    assert_eq!(
        estimate.complexity_contract().kind(),
        &ForgeQueryGraphReadComplexityContractKind::BroadTraversalCandidate
    );
}

#[test]
fn cost_estimation_is_planning_pure_not_execution_observation() {
    let mut workspace = workspace("graph-read-access.phase-five.planning-purity");
    let family = frontier_search_family(&mut workspace, "phase-five-planning-purity");
    let requirements = explain_graph_read_access_requirements_for_family(&family)
        .expect("requirements should derive");
    let observed = estimate_graph_read_access_cost_with_planning_observation(
        &requirements,
        derive_graph_read_cost_evidence(&requirements),
    );
    let estimate = observed.estimate();

    assert_eq!(estimate.counters().edge_scan_count(), 0);
    assert_eq!(estimate.counters().access_buffer_allocation_count(), 0);
    assert_eq!(observed.planning_observation().edge_read_count(), 0);
    assert_eq!(
        observed
            .planning_observation()
            .access_buffer_allocation_count(),
        0
    );
    assert_eq!(
        estimate.counters().requirement_row_count(),
        requirements.rows().len()
    );
    assert!(estimate.intrinsic().edge_touches() > 0);
    assert!(estimate.supported().index_bytes() > 0);
}
