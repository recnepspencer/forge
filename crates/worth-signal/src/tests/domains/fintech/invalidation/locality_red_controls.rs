use super::super::certification::invalidation::{
    FinancialLocalityExpectationManifest, FreshFinancialLocalityRecompute,
};
use super::super::world::{
    compile_financial_locality_world, CompiledFinancialWorld, FinancialLocalityRedObservation,
    FinancialWorldDefinition, SparseFanoutAxis,
};

#[test]
fn locality_compiler_returns_the_existing_compiled_financial_world_authority() {
    let definition =
        FinancialWorldDefinition::sparse_book_fanout(41, 64, SparseFanoutAxis::IndexDisjoint);
    let compiled: CompiledFinancialWorld = compile_financial_locality_world(definition)
        .expect("locality world must compile through the shared financial authority");

    assert!(compiled.definition().locality().is_some());
    assert_eq!(compiled.locality_definition().outputs().len(), 64);
}

fn sparse(total_outputs: u32, axis: SparseFanoutAxis) -> FinancialLocalityRedObservation {
    let definition = FinancialWorldDefinition::sparse_book_fanout(41, total_outputs, axis);
    let mut compiled = compile_financial_locality_world(definition)
        .expect("sparse financial locality world must compile and seal its baseline");
    let manifest = FinancialLocalityExpectationManifest::derive(compiled.locality_definition());
    let fresh = FreshFinancialLocalityRecompute::run(compiled.locality_definition());
    assert_eq!(
        compiled
            .committed_locality_financial_values()
            .expect("compiled locality baseline must expose committed financial values"),
        *fresh.baseline_values()
    );
    let mut expected_evaluations = fresh.changed_outputs();
    expected_evaluations.extend(manifest.unchanged_output_stops());
    assert_eq!(&expected_evaluations, manifest.necessary_evaluations());
    let expected_direct_candidates = match axis {
        SparseFanoutAxis::IndexDisjoint => 1,
        SparseFanoutAxis::QueriedRejecting => total_outputs as usize - 15,
        SparseFanoutAxis::RejectedDescendants => 2,
    };
    assert_eq!(
        manifest.candidate_dependencies().len(),
        expected_direct_candidates
    );
    let expected_causes = match axis {
        SparseFanoutAxis::RejectedDescendants => 2,
        SparseFanoutAxis::IndexDisjoint | SparseFanoutAxis::QueriedRejecting => 1,
    };
    assert_eq!(manifest.canonical_causes().len(), expected_causes);
    assert_eq!(
        manifest.canonical_work().len(),
        manifest.necessary_evaluations().len()
    );
    assert!(manifest.peak_ready_width() > 0);
    let observation = compiled
        .run_inherited_breadth_red_control()
        .expect("sparse financial locality mutation must execute");
    assert_eq!(
        compiled
            .committed_locality_financial_values()
            .expect("settled sparse world must expose committed financial values"),
        *fresh.shocked_values()
    );
    assert_eq!(
        observation.independent_necessary_evaluations,
        manifest.necessary_evaluations().len() as u64
    );
    assert_eq!(
        &observation.evaluated_outputs,
        manifest.necessary_evaluations()
    );
    assert_eq!(
        observation.unchanged_output_stops,
        manifest.unchanged_output_stops().len() as u64
    );
    observation
}

fn partitioned(
    regions: u16,
    matching_memberships: u16,
    instruments_per_matching_region: u16,
) -> FinancialLocalityRedObservation {
    let definition = FinancialWorldDefinition::partitioned_curve_universe(
        41,
        regions,
        matching_memberships,
        instruments_per_matching_region,
    );
    let mut compiled = compile_financial_locality_world(definition)
        .expect("partitioned financial locality world must compile and seal its baseline");
    let manifest = FinancialLocalityExpectationManifest::derive(compiled.locality_definition());
    let fresh = FreshFinancialLocalityRecompute::run(compiled.locality_definition());
    assert_eq!(
        compiled
            .committed_locality_financial_values()
            .expect("compiled locality baseline must expose committed financial values"),
        *fresh.baseline_values()
    );
    let mut expected_evaluations = fresh.changed_outputs();
    expected_evaluations.extend(manifest.unchanged_output_stops());
    assert_eq!(&expected_evaluations, manifest.necessary_evaluations());
    assert_eq!(manifest.queried_bucket_keys().len(), 3);
    assert_eq!(
        manifest.candidate_dependencies().len(),
        usize::from(matching_memberships)
    );
    assert_eq!(manifest.canonical_causes().len(), 1);
    assert_eq!(
        manifest.canonical_work().len(),
        manifest.necessary_evaluations().len()
    );
    assert!(manifest.peak_ready_width() > 0);
    let observation = compiled
        .run_inherited_breadth_red_control()
        .expect("partitioned financial locality mutation must execute");
    assert_eq!(
        compiled
            .committed_locality_financial_values()
            .expect("settled partition world must expose committed financial values"),
        *fresh.shocked_values()
    );
    assert_eq!(
        observation.independent_necessary_evaluations,
        manifest.necessary_evaluations().len() as u64
    );
    assert_eq!(
        &observation.evaluated_outputs,
        manifest.necessary_evaluations()
    );
    observation
}

#[test]
fn sparse_book_fanout_index_disjoint_red_control_separates_scan_from_semantic_work() {
    let small = sparse(64, SparseFanoutAxis::IndexDisjoint);
    let medium = sparse(512, SparseFanoutAxis::IndexDisjoint);

    assert_eq!(small.direct_candidates_examined, 64 - 15);
    assert_eq!(medium.direct_candidates_examined, 512 - 15);
    assert_eq!(
        medium.direct_candidates_examined - small.direct_candidates_examined,
        448
    );
    assert_eq!(small.contract_rejections, 64 - 16);
    assert_eq!(medium.contract_rejections, 512 - 16);
    assert_eq!(small.nodes_visited, medium.nodes_visited);
    assert_eq!(
        small.transitive_frontier_width,
        medium.transitive_frontier_width
    );
    assert_eq!(small.independent_necessary_evaluations, 16);
    assert_eq!(medium.independent_necessary_evaluations, 16);
}

#[test]
fn sparse_book_fanout_queried_rejection_slope_is_exact_and_semantic_work_is_flat() {
    let small = sparse(64, SparseFanoutAxis::QueriedRejecting);
    let medium = sparse(512, SparseFanoutAxis::QueriedRejecting);

    assert_eq!(
        medium.direct_candidates_examined - small.direct_candidates_examined,
        448
    );
    assert_eq!(medium.contract_rejections - small.contract_rejections, 448);
    assert_eq!(small.causality_rejections, 0);
    assert_eq!(medium.causality_rejections, 0);
    assert_eq!(small.nodes_visited, medium.nodes_visited);
    assert_eq!(
        small.independent_necessary_evaluations,
        medium.independent_necessary_evaluations
    );
}

#[test]
fn sparse_book_fanout_rejected_descendants_exposes_inherited_transitive_breadth() {
    let small = sparse(64, SparseFanoutAxis::RejectedDescendants);
    let medium = sparse(512, SparseFanoutAxis::RejectedDescendants);

    assert_eq!(small.direct_candidates_examined, 2);
    assert_eq!(medium.direct_candidates_examined, 2);
    assert_eq!(small.independent_necessary_evaluations, 17);
    assert_eq!(medium.independent_necessary_evaluations, 17);
    assert_eq!(small.unchanged_output_stops, 1);
    assert_eq!(medium.unchanged_output_stops, 1);
    assert_eq!(
        medium.transitive_frontier_width - small.transitive_frontier_width,
        448
    );
    assert_eq!(medium.nodes_visited - small.nodes_visited, 448);
}

#[test]
fn partitioned_curve_universe_red_control_exposes_producer_wide_region_scan() {
    let small = partitioned(16, 1, 1);
    let medium = partitioned(256, 1, 1);

    assert_eq!(small.direct_candidates_examined, 16);
    assert_eq!(medium.direct_candidates_examined, 256);
    assert_eq!(small.contract_rejections, 15);
    assert_eq!(medium.contract_rejections, 255);
    assert_eq!(
        medium.direct_candidates_examined - small.direct_candidates_examined,
        240
    );
    assert_eq!(small.nodes_visited, 2);
    assert_eq!(medium.nodes_visited, 2);
    assert_eq!(small.independent_necessary_evaluations, 3);
    assert_eq!(medium.independent_necessary_evaluations, 3);
}

#[test]
fn partitioned_curve_queried_membership_slope_is_exact_and_semantic_work_is_flat() {
    let one_member = partitioned(16, 1, 1);
    let four_members = partitioned(16, 4, 1);

    assert_eq!(one_member.direct_candidates_examined, 16);
    assert_eq!(four_members.direct_candidates_examined, 19);
    assert_eq!(
        four_members.direct_candidates_examined - one_member.direct_candidates_examined,
        3
    );
    assert_eq!(
        four_members.contract_rejections - one_member.contract_rejections,
        3
    );
    assert_eq!(one_member.independent_necessary_evaluations, 3);
    assert_eq!(four_members.independent_necessary_evaluations, 3);
    assert_eq!(one_member.nodes_visited, four_members.nodes_visited);
}

#[test]
fn partitioned_curve_semantic_frontier_slope_tracks_only_necessary_instruments() {
    let one_instrument = partitioned(16, 1, 1);
    let eight_instruments = partitioned(16, 1, 8);

    assert_eq!(one_instrument.direct_candidates_examined, 16);
    assert_eq!(eight_instruments.direct_candidates_examined, 16);
    assert_eq!(one_instrument.contract_rejections, 15);
    assert_eq!(eight_instruments.contract_rejections, 15);
    assert_eq!(
        eight_instruments.independent_necessary_evaluations
            - one_instrument.independent_necessary_evaluations,
        7
    );
    assert_eq!(
        eight_instruments.nodes_visited - one_instrument.nodes_visited,
        7
    );
    assert_eq!(
        eight_instruments.transitive_frontier_width - one_instrument.transitive_frontier_width,
        7
    );
}
