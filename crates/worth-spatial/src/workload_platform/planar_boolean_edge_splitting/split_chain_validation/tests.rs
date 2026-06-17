use super::tests_support::{
    chain_set_with_conflicting_source_interval_basis, chain_set_with_first_member_fragment,
    chain_set_with_first_member_fragment_and_range, chain_set_with_foreign_interval_schedule,
    chain_set_with_malformed_source_interval_basis,
    fragment_set_with_duplicate_identity_across_schedules, fragment_set_with_ranges,
    prepared_split_products,
};
use super::*;

#[test]
fn split_chain_validation_proves_each_source_edge_domain_is_covered_once() {
    let (fragments, chains) = prepared_split_products();

    let receipt = fragments
        .validate_split_edge_chains(&chains)
        .expect("prepared split products should validate");

    assert!(receipt.certifies_split_chain_integrity());
    assert_eq!(
        receipt.split_edge_fragment_set_identity(),
        fragments.fragment_set_identity()
    );
    assert_eq!(
        receipt.overlap_edge_chain_set_identity(),
        chains.chain_set_identity()
    );
    assert_eq!(receipt.counters().source_edges_checked(), 1);
    assert_eq!(receipt.counters().fragment_schedules_checked(), 1);
    assert_eq!(receipt.counters().fragments_checked(), 3);
    assert_eq!(receipt.counters().overlap_chains_checked(), 1);
    assert_eq!(receipt.counters().overlap_members_checked(), 1);
    assert_eq!(receipt.fragment_coverage_rows().len(), 1);
    assert_eq!(receipt.overlap_coverage_rows().len(), 1);
}

#[test]
fn split_chain_validation_rejects_gap_overlap_or_dangling_fragment_reference() {
    let (fragments, chains) = prepared_split_products();
    let gap_fragments = fragment_set_with_ranges(&fragments, &[[0.0, 0.25], [0.5, 1.0]]);
    let gap = gap_fragments
        .validate_split_edge_chains(&chains)
        .expect_err("fragment domain gaps must deny");
    assert_eq!(
        gap.kind(),
        PlanarBooleanSplitChainValidationDenialKind::FragmentGap
    );
    assert_eq!(gap.counters().denied_chains(), 1);

    let overlap_fragments = fragment_set_with_ranges(&fragments, &[[0.0, 0.6], [0.5, 1.0]]);
    let overlap = overlap_fragments
        .validate_split_edge_chains(&chains)
        .expect_err("fragment domain overlaps must deny");
    assert_eq!(
        overlap.kind(),
        PlanarBooleanSplitChainValidationDenialKind::FragmentOverlap
    );
    assert_eq!(overlap.counters().denied_chains(), 1);

    let dangling = chain_set_with_first_member_fragment(&chains, "missing-fragment");
    let dangling_denial = fragments
        .validate_split_edge_chains(&dangling)
        .expect_err("dangling chain member fragment reference must deny");
    assert_eq!(
        dangling_denial.kind(),
        PlanarBooleanSplitChainValidationDenialKind::DanglingOverlapFragmentReference
    );
    assert_eq!(dangling_denial.counters().denied_chains(), 1);
}

#[test]
fn overlap_chain_validation_rejects_fragment_reference_outside_source_interval() {
    let (fragments, chains) = prepared_split_products();
    let outside_interval_fragment = fragments
        .fragments()
        .find(|fragment| fragment.parameter_range()[1] <= 0.25)
        .expect("prepared fixture should include a pre-overlap fragment");
    let malformed = chain_set_with_first_member_fragment_and_range(
        &chains,
        outside_interval_fragment.fragment_identity(),
        outside_interval_fragment.parameter_range(),
    );

    let denial = fragments
        .validate_split_edge_chains(&malformed)
        .expect_err("overlap member outside its source interval must deny");

    assert_eq!(
        denial.kind(),
        PlanarBooleanSplitChainValidationDenialKind::OverlapFragmentOutsideSourceInterval
    );
    assert_eq!(denial.counters().out_of_interval_references_rejected(), 1);
}

#[test]
fn split_chain_validation_rejects_non_finite_fragment_domain_as_malformed() {
    let (fragments, chains) = prepared_split_products();
    let malformed = fragment_set_with_ranges(&fragments, &[[f64::NAN, 0.25], [0.25, 1.0]]);

    let denial = malformed
        .validate_split_edge_chains(&chains)
        .expect_err("non-finite fragment boundary must deny as malformed range");

    assert_eq!(
        denial.kind(),
        PlanarBooleanSplitChainValidationDenialKind::MalformedFragmentRange
    );
    assert_eq!(denial.counters().denied_chains(), 1);
}

#[test]
fn overlap_chain_validation_rejects_conflicting_source_interval_basis() {
    let (fragments, chains) = prepared_split_products();
    let malformed = chain_set_with_conflicting_source_interval_basis(&chains);

    let denial = fragments
        .validate_split_edge_chains(&malformed)
        .expect_err("same source interval identity with conflicting range basis must deny");

    assert_eq!(
        denial.kind(),
        PlanarBooleanSplitChainValidationDenialKind::MismatchedOverlapIntervalBasis
    );
    assert_eq!(denial.counters().mismatched_interval_basis_rejected(), 1);
    assert_eq!(denial.counters().denied_chains(), 1);
}

#[test]
fn split_chain_validation_rejects_duplicate_fragment_identity_across_schedules() {
    let (fragments, chains) = prepared_split_products();
    let duplicate = fragment_set_with_duplicate_identity_across_schedules(&fragments);

    let denial = duplicate
        .validate_split_edge_chains(&chains)
        .expect_err("duplicate fragment identity across schedules must deny before indexed lookup");

    assert_eq!(
        denial.kind(),
        PlanarBooleanSplitChainValidationDenialKind::DuplicateFragmentIdentity
    );
    assert_eq!(denial.counters().dangling_references_rejected(), 1);
    assert_eq!(denial.counters().denied_chains(), 1);
}

#[test]
fn split_chain_validation_rejects_foreign_interval_subdivision_chain_authority() {
    let (fragments, chains) = prepared_split_products();
    let foreign = chain_set_with_foreign_interval_schedule(&chains);

    let denial = fragments
        .validate_split_edge_chains(&foreign)
        .expect_err("matching fragment set identity cannot hide foreign interval authority");

    assert_eq!(
        denial.kind(),
        PlanarBooleanSplitChainValidationDenialKind::ForeignOverlapChainSet
    );
    assert_eq!(denial.counters().foreign_chain_sets_rejected(), 1);
    assert_eq!(denial.counters().denied_chains(), 1);
}

#[test]
fn overlap_chain_validation_rejects_malformed_source_interval_basis() {
    let (fragments, chains) = prepared_split_products();
    let malformed = chain_set_with_malformed_source_interval_basis(&chains);

    let denial = fragments
        .validate_split_edge_chains(&malformed)
        .expect_err("malformed overlap source interval basis must deny");

    assert_eq!(
        denial.kind(),
        PlanarBooleanSplitChainValidationDenialKind::MalformedOverlapIntervalBasis
    );
    assert_eq!(denial.counters().mismatched_interval_basis_rejected(), 1);
    assert_eq!(denial.counters().denied_chains(), 1);
}
