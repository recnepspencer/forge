use super::edge_splitting_decision_log_support::build_decision_log_products_for_metaboss;
use super::metaboss_support::MetabossEventExtractionSubject;
#[path = "public_api_planar_boolean_edge_splitting_ledger_manifest_support.rs"]
mod ledger_manifest_support;
#[path = "public_api_planar_boolean_edge_splitting_ledger_oracle_support.rs"]
mod ledger_oracle_support;
use ledger_manifest_support::{edge_key, ObservedSplitEdgeChainLedgerManifest};
use ledger_oracle_support::MetabossSplitLedgerOracle;
use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanSplitDecisionLogQueryDomain, PlanarBooleanSplitDecisionLogQueryInput,
    PlanarBooleanSplitEdgeChainLedgerQueryDomain, PlanarBooleanSplitEdgeChainLedgerQueryInput,
    PlanarBooleanSplitEdgeChainLedgerQueryResult,
};
use worth_spatial::facade::workload_vocabulary::{
    BooleanEvidenceReceipt, BooleanEvidenceStageKind, WorkloadEvidenceRow,
};

pub(crate) fn assert_split_edge_chain_ledger_matches_metaboss(
    subject: &MetabossEventExtractionSubject,
) {
    let (result, expected, decision_log, oracle) =
        build_split_edge_chain_ledger_with_manifest_for_metaboss(subject);
    oracle.assert_matches_observed_products(&expected);
    let ledger = result.ledger();
    let receipt = result.receipt();
    oracle.assert_matches_ledger_semantics(ledger, &decision_log);

    assert!(result.certifies_query_owned_split_edge_chain_ledger());
    assert_eq!(receipt.boolean_stage(), BooleanEvidenceStageKind::Split);
    assert_eq!(receipt.evidence_identity(), receipt.receipt_identity());
    assert_eq!(
        WorkloadEvidenceRow::from_boolean_evidence_receipt(receipt).stage(),
        BooleanEvidenceStageKind::Split.evidence_stage()
    );
    assert_eq!(ledger.chains().len(), receipt.chain_identities().len());
    assert_eq!(
        receipt.counters().ledger_chains_emitted(),
        ledger.chains().len()
    );
    assert_eq!(receipt.counters().validation_receipts_consumed(), 1);
    assert_eq!(receipt.counters().downstream_identities_emitted(), 1);
    assert_eq!(
        receipt.counters().fragment_rows_consumed(),
        expected.total_fragments()
    );
    assert_eq!(
        receipt.counters().persistent_name_rows_bound(),
        expected.persistent_name_rows_bound
    );
    assert_eq!(
        receipt.counters().decision_rows_bound(),
        expected.decision_rows_bound
    );
    assert_eq!(
        receipt.split_chain_validation_receipt_identity(),
        ledger.split_chain_validation_receipt_identity()
    );
    assert_eq!(
        receipt.split_persistent_naming_receipt_identity(),
        ledger.split_persistent_naming_receipt_identity()
    );
    assert_eq!(
        receipt.split_decision_log_receipt_identity(),
        ledger.split_decision_log_receipt_identity()
    );
    assert!(!receipt.downstream_consumption_identity().is_empty());
    assert_eq!(ledger.chains().len(), expected.chains.len());
    for chain in ledger.chains() {
        let expected_chain = expected
            .chains
            .get(&edge_key(
                chain.source_edge_identity(),
                chain.carrier_identity(),
            ))
            .expect("ledger chain must correspond to an expected production product key");
        assert_eq!(
            chain.endpoint_boundary_schedule_identity(),
            expected_chain.endpoint_boundary_schedule_identity
        );
        assert_eq!(
            chain.interval_subdivision_schedule_identity(),
            expected_chain.interval_subdivision_schedule_identity
        );
        assert_eq!(
            chain.split_vertex_schedule_identity(),
            expected_chain.split_vertex_schedule_identity
        );
        assert_eq!(
            chain.split_fragment_schedule_identity(),
            expected_chain.split_fragment_schedule_identity
        );
        assert_eq!(
            chain.fragment_identities(),
            expected_chain.fragment_identities
        );
        assert_eq!(
            chain.split_vertex_identities(),
            expected_chain.split_vertex_identities
        );
        assert_eq!(
            chain.overlap_chain_identities(),
            expected_chain.overlap_chain_identities
        );
        assert_eq!(
            chain.persistent_name_row_identities(),
            expected_chain.persistent_name_row_identities
        );
        assert_eq!(
            chain.decision_identities(),
            expected_chain.decision_identities
        );
        assert_eq!(
            chain.validation_fragment_coverage_identities(),
            expected_chain.validation_fragment_coverage_identities
        );
        assert_eq!(
            chain.validation_overlap_coverage_identities(),
            expected_chain.validation_overlap_coverage_identities
        );
    }
}

pub(crate) fn assert_split_edge_chain_ledger_orders_all_products_canonically_across_replay(
    subject: &MetabossEventExtractionSubject,
) {
    let (first, _, _, _) = build_split_edge_chain_ledger_with_manifest_for_metaboss(subject);
    let (replayed, _, _, _) = build_split_edge_chain_ledger_with_manifest_for_metaboss(subject);

    assert_eq!(
        first.ledger().ledger_identity(),
        replayed.ledger().ledger_identity()
    );
    assert_eq!(
        first.receipt().receipt_identity(),
        replayed.receipt().receipt_identity()
    );
    assert_eq!(
        first.receipt().chain_identities(),
        replayed.receipt().chain_identities()
    );
    let mut sorted_chain_identities = first.receipt().chain_identities().to_vec();
    sorted_chain_identities.sort();
    assert_eq!(first.receipt().chain_identities(), sorted_chain_identities);
    assert_eq!(
        first.receipt().downstream_consumption_identity(),
        replayed.receipt().downstream_consumption_identity()
    );
}

pub(crate) fn build_split_edge_chain_ledger_with_manifest_for_metaboss(
    subject: &MetabossEventExtractionSubject,
) -> (
    PlanarBooleanSplitEdgeChainLedgerQueryResult,
    ObservedSplitEdgeChainLedgerManifest,
    worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitDecisionLogQueryResult,
    MetabossSplitLedgerOracle,
) {
    let products = build_decision_log_products_for_metaboss(subject);
    let oracle = MetabossSplitLedgerOracle::from_products(&products);
    let decision_log = PlanarBooleanSplitDecisionLogQueryDomain::declare(
        PlanarBooleanSplitDecisionLogQueryInput::new(
            &products.request,
            &products.endpoint_boundary,
            &products.interval_subdivision,
            &products.vertices,
            &products.fragments,
            &products.validation,
            &products.naming,
        ),
    )
    .expect("metaboss decision-log Query declaration should lower")
    .execute()
    .expect("metaboss decision-log Query plan should execute");
    let expected = ObservedSplitEdgeChainLedgerManifest::from_products(&products, &decision_log);

    let ledger = PlanarBooleanSplitEdgeChainLedgerQueryDomain::declare(
        PlanarBooleanSplitEdgeChainLedgerQueryInput::new(
            &products.request,
            &products.endpoint_boundary,
            &products.interval_subdivision,
            &products.vertices,
            &products.fragments,
            &products.chains,
            &products.validation,
            &products.naming,
            &decision_log,
        ),
    )
    .expect("metaboss split ledger Query declaration should lower")
    .execute()
    .expect("metaboss split ledger Query plan should execute");
    (ledger, expected, decision_log, oracle)
}
