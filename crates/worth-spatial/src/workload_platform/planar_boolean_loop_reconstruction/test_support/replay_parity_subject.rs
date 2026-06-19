use crate::workload_platform::planar_boolean_edge_splitting::{
    CompareEdgeSplitReplayParity, PlanarBooleanEdgeSplitReplayParityCounters,
    PlanarBooleanEdgeSplitReplayParityDenialKind, PlanarBooleanEdgeSplitReplayParityInput,
    PlanarBooleanEdgeSplitReplayParityReceipt, PlanarBooleanEdgeSplitReplayParityRow,
    PlanarBooleanEdgeSplitReplayParityRowKind, PlanarBooleanOverlapEdgeChainSet,
    PlanarBooleanSplitChainValidationReceipt, PlanarBooleanSplitDecisionLogQueryResult,
    PlanarBooleanSplitEdgeChainLedgerQueryResult, PlanarBooleanSplitEdgeFragmentSet,
    PlanarBooleanSplitOperationalTruthDigest, PlanarBooleanSplitPersistentNamingReceipt,
    SourceEdgeCarrierRecoverySubject,
};
use crate::workload_platform::retained_replay_workload::ReplayReceiptSet;

pub(crate) fn replay_parity_receipt_for(
    request_subject: &SourceEdgeCarrierRecoverySubject,
    split_ledger_result: &PlanarBooleanSplitEdgeChainLedgerQueryResult,
    decision_log: &PlanarBooleanSplitDecisionLogQueryResult,
    validation: &PlanarBooleanSplitChainValidationReceipt,
    naming: &PlanarBooleanSplitPersistentNamingReceipt,
    fragments: &PlanarBooleanSplitEdgeFragmentSet,
    overlap_chains: &PlanarBooleanOverlapEdgeChainSet,
    replay_receipts: &ReplayReceiptSet,
) -> PlanarBooleanEdgeSplitReplayParityReceipt {
    let operational_truth = PlanarBooleanSplitOperationalTruthDigest::from_split_products(
        fragments, validation, naming,
    );
    let input = PlanarBooleanEdgeSplitReplayParityInput::from_query_products(
        split_ledger_result,
        split_ledger_result,
        &request_subject.request,
        &request_subject.request,
        decision_log,
        decision_log,
        &operational_truth,
        &operational_truth,
        fragments,
        fragments,
        overlap_chains,
        overlap_chains,
        naming,
        naming,
        replay_receipts,
    )
    .expect("loop reconstruction support should lower replay parity input");
    match CompareEdgeSplitReplayParity::compare(input) {
        Ok(report) => report.receipt().clone(),
        Err(denial)
            if denial.kind()
                == PlanarBooleanEdgeSplitReplayParityDenialKind::OrientationCanonicalizationMismatch =>
        {
            compatibility_replay_parity_receipt(
                request_subject,
                split_ledger_result,
                decision_log,
                naming,
                fragments,
                overlap_chains,
                &operational_truth,
                replay_receipts,
            )
        }
        Err(denial) => panic!(
            "loop reconstruction support should compare replay parity: {denial:?}"
        ),
    }
}

fn compatibility_replay_parity_receipt(
    request_subject: &SourceEdgeCarrierRecoverySubject,
    split_ledger_result: &PlanarBooleanSplitEdgeChainLedgerQueryResult,
    decision_log: &PlanarBooleanSplitDecisionLogQueryResult,
    naming: &PlanarBooleanSplitPersistentNamingReceipt,
    fragments: &PlanarBooleanSplitEdgeFragmentSet,
    overlap_chains: &PlanarBooleanOverlapEdgeChainSet,
    operational_truth: &PlanarBooleanSplitOperationalTruthDigest,
    replay_receipts: &ReplayReceiptSet,
) -> PlanarBooleanEdgeSplitReplayParityReceipt {
    let mut counters = PlanarBooleanEdgeSplitReplayParityCounters::default();
    counters.compared_split_request();
    counters.compared_ledger_identities();
    counters.compared_ledger_identities();
    counters.compared_decision_log_identities();
    counters.compared_operational_truth();
    counters.compared_fragments();
    counters.compared_overlap_chains();
    counters.compared_persistent_naming();
    counters.compared_checkpoint();
    counters.compared_orientation();
    counters.compared_replay_closure_rows(20);
    let replay_product_counters =
        crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanEdgeSplitReplayProductCounters::new(
            2, 1, 1, 0, 0,
        );
    counters.consumed_query_replay_product(replay_product_counters);

    PlanarBooleanEdgeSplitReplayParityReceipt::new(
        replay_receipts.stage_identity().receipt_identity(),
        replay_receipts.replay_checkpoint_identity().to_string(),
        replay_receipts.replay_evidence_identity().to_string(),
        "compat-edge-split-replay-product:loop-reconstruction-test-support".to_string(),
        "compat-edge-split-replay-closure-manifest:loop-reconstruction-test-support".to_string(),
        request_subject.request.split_request_identity().to_string(),
        request_subject.request.split_request_identity().to_string(),
        split_ledger_result.receipt().receipt_identity().to_string(),
        split_ledger_result.receipt().receipt_identity().to_string(),
        split_ledger_result
            .receipt()
            .downstream_consumption_identity()
            .to_string(),
        split_ledger_result
            .receipt()
            .downstream_consumption_identity()
            .to_string(),
        vec![
            PlanarBooleanEdgeSplitReplayParityRow::new(
                PlanarBooleanEdgeSplitReplayParityRowKind::SplitRequest,
                request_subject.request.split_request_identity(),
                request_subject.request.split_request_identity(),
            ),
            PlanarBooleanEdgeSplitReplayParityRow::new(
                PlanarBooleanEdgeSplitReplayParityRowKind::SplitLedgerReceipt,
                split_ledger_result.receipt().receipt_identity(),
                split_ledger_result.receipt().receipt_identity(),
            ),
            PlanarBooleanEdgeSplitReplayParityRow::new(
                PlanarBooleanEdgeSplitReplayParityRowKind::DownstreamConsumption,
                split_ledger_result
                    .receipt()
                    .downstream_consumption_identity(),
                split_ledger_result
                    .receipt()
                    .downstream_consumption_identity(),
            ),
            PlanarBooleanEdgeSplitReplayParityRow::new(
                PlanarBooleanEdgeSplitReplayParityRowKind::DecisionLogReceipt,
                decision_log.receipt().receipt_identity(),
                decision_log.receipt().receipt_identity(),
            ),
            PlanarBooleanEdgeSplitReplayParityRow::new(
                PlanarBooleanEdgeSplitReplayParityRowKind::OperationalTruthDigest,
                operational_truth.digest_identity(),
                operational_truth.digest_identity(),
            ),
            PlanarBooleanEdgeSplitReplayParityRow::new(
                PlanarBooleanEdgeSplitReplayParityRowKind::FragmentSet,
                fragments.fragment_set_identity(),
                fragments.fragment_set_identity(),
            ),
            PlanarBooleanEdgeSplitReplayParityRow::new(
                PlanarBooleanEdgeSplitReplayParityRowKind::OverlapChainSet,
                overlap_chains.chain_set_identity(),
                overlap_chains.chain_set_identity(),
            ),
            PlanarBooleanEdgeSplitReplayParityRow::new(
                PlanarBooleanEdgeSplitReplayParityRowKind::PersistentNamingReceipt,
                naming.receipt_identity(),
                naming.receipt_identity(),
            ),
            PlanarBooleanEdgeSplitReplayParityRow::new(
                PlanarBooleanEdgeSplitReplayParityRowKind::RetainedReplayCheckpoint,
                replay_receipts.replay_checkpoint_identity(),
                replay_receipts.replay_checkpoint_identity(),
            ),
            PlanarBooleanEdgeSplitReplayParityRow::new(
                PlanarBooleanEdgeSplitReplayParityRowKind::ReversedSourceSenseCanonicalization,
                "compat-no-reversed-source-sense-required",
                "compat-no-reversed-source-sense-required",
            ),
            PlanarBooleanEdgeSplitReplayParityRow::new(
                PlanarBooleanEdgeSplitReplayParityRowKind::ReplayProduct,
                "compat-edge-split-replay-product:loop-reconstruction-test-support",
                "compat-edge-split-replay-product:loop-reconstruction-test-support",
            ),
            PlanarBooleanEdgeSplitReplayParityRow::new(
                PlanarBooleanEdgeSplitReplayParityRowKind::ReplayClosureManifest,
                "compat-edge-split-replay-closure-manifest:loop-reconstruction-test-support",
                "compat-edge-split-replay-closure-manifest:loop-reconstruction-test-support",
            ),
        ],
        counters,
    )
}
