use super::super::certification::invalidation::{
    FinancialLocalityExpectationManifest, FreshFinancialLocalityRecompute,
};
use super::super::world::{
    compile_financial_locality_world, CompiledFinancialWorld, FinancialLocalityRedObservation,
    FinancialWorldDefinition, LocalitySemanticOutputId, SparseFanoutAxis,
};
use std::collections::BTreeSet;
use std::ops::Deref;

struct RedControlEvidence {
    observation: FinancialLocalityRedObservation,
    expected_evaluations: BTreeSet<LocalitySemanticOutputId>,
    expected_stops: BTreeSet<LocalitySemanticOutputId>,
}

impl Deref for RedControlEvidence {
    type Target = FinancialLocalityRedObservation;

    fn deref(&self) -> &Self::Target {
        &self.observation
    }
}

impl RedControlEvidence {
    fn extra_evaluations(&self) -> usize {
        self.evaluated_outputs.len() - self.expected_evaluations.len()
    }
}

#[test]
fn locality_compiler_returns_the_existing_compiled_financial_world_authority() {
    let definition =
        FinancialWorldDefinition::sparse_book_fanout(41, 64, SparseFanoutAxis::IndexDisjoint);
    let compiled: CompiledFinancialWorld = compile_financial_locality_world(definition)
        .expect("locality world must compile through the shared financial authority");

    assert!(compiled.definition().locality().is_some());
    assert_eq!(compiled.locality_definition().outputs().len(), 64);
}

fn sparse(total_outputs: u32, axis: SparseFanoutAxis) -> RedControlEvidence {
    let definition = FinancialWorldDefinition::sparse_book_fanout(41, total_outputs, axis);
    let (compiled, manifest, fresh) = prepare_red_control(definition);
    let expected_trace_candidates = match axis {
        SparseFanoutAxis::IndexDisjoint => 15,
        SparseFanoutAxis::QueriedRejecting => total_outputs as usize - 1,
        SparseFanoutAxis::RejectedDescendants => 16,
    };
    assert_eq!(
        manifest.candidate_dependencies().len(),
        expected_trace_candidates
    );
    let expected_causes = match axis {
        SparseFanoutAxis::RejectedDescendants => 16,
        SparseFanoutAxis::IndexDisjoint | SparseFanoutAxis::QueriedRejecting => 15,
    };
    assert_eq!(manifest.canonical_causes().len(), expected_causes);
    assert_eq!(
        manifest.queried_bucket_keys().len(),
        if matches!(axis, SparseFanoutAxis::QueriedRejecting) {
            18
        } else {
            16
        }
    );
    assert_eq!(
        manifest.unchanged_output_stops().len(),
        usize::from(matches!(axis, SparseFanoutAxis::RejectedDescendants))
    );
    assert_eq!(
        manifest.canonical_work().len(),
        manifest.necessary_evaluations().len()
    );
    assert!(manifest.peak_ready_width() > 0);
    execute_red_control(compiled, manifest, fresh)
}

fn partitioned(
    regions: u16,
    matching_memberships: u16,
    instruments_per_matching_region: u16,
) -> RedControlEvidence {
    let definition = FinancialWorldDefinition::partitioned_curve_universe(
        41,
        regions,
        matching_memberships,
        instruments_per_matching_region,
    );
    let (compiled, manifest, fresh) = prepare_red_control(definition);
    assert_eq!(
        manifest.queried_bucket_keys().len(),
        usize::from(instruments_per_matching_region) + 4
    );
    assert_eq!(
        manifest.candidate_dependencies().len(),
        usize::from(matching_memberships + instruments_per_matching_region)
    );
    assert_eq!(
        manifest.canonical_causes().len(),
        usize::from(instruments_per_matching_region) + 1
    );
    assert_eq!(
        manifest.canonical_work().len(),
        manifest.necessary_evaluations().len()
    );
    assert!(manifest.peak_ready_width() > 0);
    execute_red_control(compiled, manifest, fresh)
}

fn prepare_red_control(
    definition: FinancialWorldDefinition,
) -> (
    CompiledFinancialWorld,
    FinancialLocalityExpectationManifest,
    FreshFinancialLocalityRecompute,
) {
    let compiled = compile_financial_locality_world(definition)
        .expect("financial locality world must compile and seal its baseline");
    let manifest = FinancialLocalityExpectationManifest::derive(
        compiled.locality_definition(),
        compiled.locality_graph_instance(),
    );
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
    (compiled, manifest, fresh)
}

fn execute_red_control(
    mut compiled: CompiledFinancialWorld,
    manifest: FinancialLocalityExpectationManifest,
    fresh: FreshFinancialLocalityRecompute,
) -> RedControlEvidence {
    let observation = compiled
        .run_inherited_breadth_red_control()
        .expect("financial locality mutation must execute");
    assert_eq!(
        compiled
            .committed_locality_financial_values()
            .expect("settled locality world must expose committed financial values"),
        *fresh.shocked_values()
    );
    assert!(manifest
        .necessary_evaluations()
        .is_subset(&observation.evaluated_outputs));
    assert!(manifest
        .unchanged_output_stops()
        .is_subset(&observation.baseline_retained_outputs));
    assert!(
        observation.comparator_suppressed_count >= manifest.unchanged_output_stops().len() as u64
    );
    assert_eq!(
        observation.evaluated_outputs,
        *manifest.necessary_evaluations()
    );
    assert_eq!(
        observation.reverse_candidates_returned,
        observation.direct_candidates_examined
    );
    RedControlEvidence {
        observation,
        expected_evaluations: manifest.necessary_evaluations().clone(),
        expected_stops: manifest.unchanged_output_stops().clone(),
    }
}

#[test]
fn sparse_book_fanout_index_disjoint_candidates_remain_flat() {
    let small = sparse(64, SparseFanoutAxis::IndexDisjoint);
    let medium = sparse(512, SparseFanoutAxis::IndexDisjoint);

    assert_eq!(small.direct_candidates_examined, 15);
    assert_eq!(medium.direct_candidates_examined, 15);
    assert_eq!(small.contract_rejections, 0);
    assert_eq!(medium.contract_rejections, 0);
    assert_eq!(small.causality_rejections, 0);
    assert_eq!(medium.causality_rejections, 0);
    assert_eq!(small.nodes_visited, 0);
    assert_eq!(medium.nodes_visited, 0);
    assert_eq!(small.transitive_frontier_width, 0);
    assert_eq!(medium.transitive_frontier_width, 0);
    assert_eq!(small.reverse_bucket_probes, medium.reverse_bucket_probes);
    assert_eq!(small.evaluated_outputs.len(), 16);
    assert_eq!(medium.evaluated_outputs.len(), 16);
    assert_eq!(small.extra_evaluations(), 0);
    assert_eq!(medium.extra_evaluations(), 0);
}

#[test]
fn sparse_book_fanout_queried_rejections_are_counted_without_semantic_work() {
    let small = sparse(64, SparseFanoutAxis::QueriedRejecting);
    let medium = sparse(512, SparseFanoutAxis::QueriedRejecting);

    assert_eq!(
        medium.direct_candidates_examined - small.direct_candidates_examined,
        448
    );
    assert_eq!(medium.contract_rejections - small.contract_rejections, 448);
    assert_eq!(small.causality_rejections, 0);
    assert_eq!(medium.causality_rejections, 0);
    assert_eq!(small.nodes_visited, 0);
    assert_eq!(medium.nodes_visited, 0);
    assert_eq!(small.expected_evaluations.len(), 16);
    assert_eq!(medium.expected_evaluations.len(), 16);
    assert_eq!(small.extra_evaluations(), 0);
    assert_eq!(medium.extra_evaluations(), 0);
    assert_eq!(small.transitive_frontier_width, 0);
    assert_eq!(medium.transitive_frontier_width, 0);
}

#[test]
fn sparse_book_fanout_rejected_descendants_add_no_routing_breadth() {
    let small = sparse(64, SparseFanoutAxis::RejectedDescendants);
    let medium = sparse(512, SparseFanoutAxis::RejectedDescendants);

    assert_eq!(small.direct_candidates_examined, 16);
    assert_eq!(medium.direct_candidates_examined, 16);
    assert_eq!(small.expected_evaluations.len(), 17);
    assert_eq!(medium.expected_evaluations.len(), 17);
    assert_eq!(small.expected_stops.len(), 1);
    assert_eq!(medium.expected_stops.len(), 1);
    assert_eq!(small.evaluated_outputs.len(), 17);
    assert_eq!(medium.evaluated_outputs.len(), 17);
    assert_eq!(small.extra_evaluations(), 0);
    assert_eq!(medium.extra_evaluations(), 0);
    assert!(small.comparator_suppressed_count >= 1);
    assert!(medium.comparator_suppressed_count >= 1);
    assert_eq!(small.baseline_retained_outputs, small.expected_stops);
    assert_eq!(medium.baseline_retained_outputs, medium.expected_stops);
    assert_eq!(small.transitive_frontier_width, 0);
    assert_eq!(medium.transitive_frontier_width, 0);
    assert_eq!(small.nodes_visited, 0);
    assert_eq!(medium.nodes_visited, 0);
}

#[test]
fn partitioned_curve_universe_disjoint_regions_do_not_widen_queries() {
    let small = partitioned(16, 1, 1);
    let medium = partitioned(256, 1, 1);

    assert_eq!(small.direct_candidates_examined, 2);
    assert_eq!(medium.direct_candidates_examined, 2);
    assert_eq!(small.contract_rejections, 0);
    assert_eq!(medium.contract_rejections, 0);
    assert_eq!(small.nodes_visited, 0);
    assert_eq!(medium.nodes_visited, 0);
    assert_eq!(small.evaluated_outputs.len(), 3);
    assert_eq!(medium.evaluated_outputs.len(), 3);
    assert_eq!(small.extra_evaluations(), 0);
    assert_eq!(medium.extra_evaluations(), 0);
}

#[test]
fn partitioned_curve_queried_membership_rejections_are_exact() {
    let one_member = partitioned(16, 1, 1);
    let four_members = partitioned(16, 4, 1);

    assert_eq!(one_member.direct_candidates_examined, 2);
    assert_eq!(four_members.direct_candidates_examined, 5);
    assert_eq!(
        four_members.direct_candidates_examined - one_member.direct_candidates_examined,
        3
    );
    assert_eq!(
        four_members.contract_rejections - one_member.contract_rejections,
        3
    );
    assert_eq!(one_member.expected_evaluations.len(), 3);
    assert_eq!(four_members.expected_evaluations.len(), 3);
    assert_eq!(one_member.extra_evaluations(), 0);
    assert_eq!(four_members.extra_evaluations(), 0);
    assert_eq!(one_member.nodes_visited, 0);
    assert_eq!(four_members.nodes_visited, 0);
}

#[test]
fn partitioned_curve_semantic_frontier_slope_tracks_only_necessary_instruments() {
    let one_instrument = partitioned(16, 1, 1);
    let eight_instruments = partitioned(16, 1, 8);

    assert_eq!(one_instrument.direct_candidates_examined, 2);
    assert_eq!(eight_instruments.direct_candidates_examined, 9);
    assert_eq!(one_instrument.contract_rejections, 0);
    assert_eq!(eight_instruments.contract_rejections, 0);
    assert_eq!(
        eight_instruments.evaluated_outputs.len() - one_instrument.evaluated_outputs.len(),
        7
    );
    assert_eq!(one_instrument.extra_evaluations(), 0);
    assert_eq!(eight_instruments.extra_evaluations(), 0);
    assert_eq!(one_instrument.nodes_visited, 0);
    assert_eq!(eight_instruments.nodes_visited, 0);
    assert_eq!(one_instrument.transitive_frontier_width, 0);
    assert_eq!(eight_instruments.transitive_frontier_width, 0);
}
