use super::edge_splitting_split_vertex_identity_support::build_interval_subdivision_schedule_for_metaboss;
use super::metaboss_support::MetabossEventExtractionSubject;
use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanOverlapEdgeChainSet, PlanarBooleanSplitChainValidationReceipt,
    PlanarBooleanSplitEdgeFragmentSet,
};

pub(crate) fn assert_split_chain_validation_matches_metaboss(
    subject: &MetabossEventExtractionSubject,
) {
    let interval_normalized = build_interval_subdivision_schedule_for_metaboss(subject);
    let split_vertices = interval_normalized
        .mint_split_vertex_identities()
        .expect("metaboss split vertices should mint before split-chain validation");
    let fragments = interval_normalized
        .build_split_edge_fragments(&split_vertices)
        .expect("metaboss split fragments should build before split-chain validation");
    let chains = interval_normalized
        .build_overlap_edge_chains(&fragments)
        .expect("metaboss overlap chains should build before split-chain validation");
    let receipt = fragments
        .validate_split_edge_chains(&chains)
        .expect("metaboss split chain validation should certify prepared products");

    assert_split_chain_validation_receipt_reconciles(&fragments, &chains, &receipt);
}

fn assert_split_chain_validation_receipt_reconciles(
    fragments: &PlanarBooleanSplitEdgeFragmentSet,
    chains: &PlanarBooleanOverlapEdgeChainSet,
    receipt: &PlanarBooleanSplitChainValidationReceipt,
) {
    assert!(receipt.certifies_split_chain_integrity());
    assert_eq!(
        receipt.split_edge_fragment_set_identity(),
        fragments.fragment_set_identity()
    );
    assert_eq!(
        receipt.overlap_edge_chain_set_identity(),
        chains.chain_set_identity()
    );
    assert_eq!(
        receipt.interval_subdivision_schedule_set_identity(),
        fragments.interval_subdivision_schedule_set_identity()
    );
    assert_eq!(
        receipt.counters().fragment_schedules_checked(),
        fragments.schedules().len()
    );
    assert_eq!(
        receipt.counters().fragments_checked(),
        fragments.fragments().count()
    );
    assert_eq!(
        receipt.counters().overlap_chains_checked(),
        chains.chains().len()
    );
    assert_eq!(
        receipt.counters().overlap_members_checked(),
        chains
            .chains()
            .iter()
            .map(|chain| chain.members().len())
            .sum::<usize>()
    );
    assert_eq!(receipt.counters().gaps_rejected(), 0);
    assert_eq!(receipt.counters().overlaps_rejected(), 0);
    assert_eq!(receipt.counters().dangling_references_rejected(), 0);
    assert_eq!(receipt.counters().mismatched_interval_basis_rejected(), 0);
    assert_eq!(receipt.counters().out_of_interval_references_rejected(), 0);
    assert_eq!(
        receipt.fragment_coverage_rows().len(),
        fragments.schedules().len()
    );
    assert!(!receipt.overlap_coverage_rows().is_empty());
}
