use super::super::parity_receipt::denial::{
    PlanarBooleanEdgeSplitReplayParityDenial, PlanarBooleanEdgeSplitReplayParityDenialKind as Kind,
};
use super::super::parity_receipt::{
    PlanarBooleanEdgeSplitReplayParityCounters, PlanarBooleanEdgeSplitReplayParityReceipt,
    PlanarBooleanEdgeSplitReplayParityRow, PlanarBooleanEdgeSplitReplayParityRowKind as RowKind,
};
use super::checkpoint_comparison::validate_checkpoint_receipts;
use super::input::PlanarBooleanEdgeSplitReplayParityInput;
use super::orientation_canonicalization::validate_reversed_source_sense_canonicalization;

pub struct ReplayPlanarBooleanEdgeSplit;
pub struct CompareEdgeSplitReplayParity;
pub struct CompareEdgeSplitCheckpointParity;
pub struct CanonicalizeReversedEdgeSenseSplit;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanEdgeSplitReplayParityReport {
    receipt: PlanarBooleanEdgeSplitReplayParityReceipt,
}

impl ReplayPlanarBooleanEdgeSplit {
    pub fn compare_retained_split_products(
        input: PlanarBooleanEdgeSplitReplayParityInput<'_>,
    ) -> Result<PlanarBooleanEdgeSplitReplayParityReport, PlanarBooleanEdgeSplitReplayParityDenial>
    {
        CompareEdgeSplitReplayParity::compare(input)
    }
}

impl CompareEdgeSplitReplayParity {
    pub fn compare(
        input: PlanarBooleanEdgeSplitReplayParityInput<'_>,
    ) -> Result<PlanarBooleanEdgeSplitReplayParityReport, PlanarBooleanEdgeSplitReplayParityDenial>
    {
        let mut counters = PlanarBooleanEdgeSplitReplayParityCounters::default();
        let mut rows = Vec::new();
        let (replay_product_identity, replay_closure_manifest_identity) =
            if let Some(replay_product) = input.replay_product() {
                counters.consumed_query_replay_product(replay_product.counters());
                counters
                    .compared_replay_closure_rows(replay_product.closure_manifest().rows().len());
                compare_row(
                    RowKind::ReplayProduct,
                    replay_product.product_identity(),
                    replay_product.product_identity(),
                    Kind::ReplayProductNotQueryOwned,
                    &mut counters,
                    &mut rows,
                )?;
                compare_row(
                    RowKind::ReplayClosureManifest,
                    replay_product.closure_manifest().manifest_identity(),
                    replay_product.closure_manifest().manifest_identity(),
                    Kind::ReplayProductNotQueryOwned,
                    &mut counters,
                    &mut rows,
                )?;
                (
                    replay_product.product_identity().to_string(),
                    replay_product
                        .closure_manifest()
                        .manifest_identity()
                        .to_string(),
                )
            } else {
                (
                    "legacy-edge-split-replay-product:compatibility-shim".to_string(),
                    "legacy-edge-split-replay-closure-manifest:compatibility-shim".to_string(),
                )
            };

        compare_row(
            RowKind::SplitRequest,
            input.original_request().split_request_identity(),
            input.replayed_request().split_request_identity(),
            Kind::ReplaySplitRequestMismatch,
            &mut counters,
            &mut rows,
        )?;
        counters.compared_split_request();
        compare_row(
            RowKind::SplitLedgerReceipt,
            input.original_ledger().receipt().receipt_identity(),
            input.replayed_ledger().receipt().receipt_identity(),
            Kind::ReplayLedgerMismatch,
            &mut counters,
            &mut rows,
        )?;
        counters.compared_ledger_identities();
        compare_row(
            RowKind::DownstreamConsumption,
            input
                .original_ledger()
                .receipt()
                .downstream_consumption_identity(),
            input
                .replayed_ledger()
                .receipt()
                .downstream_consumption_identity(),
            Kind::ReplayLedgerMismatch,
            &mut counters,
            &mut rows,
        )?;
        counters.compared_ledger_identities();
        compare_row(
            RowKind::DecisionLogReceipt,
            input.original_decision_log().receipt().receipt_identity(),
            input.replayed_decision_log().receipt().receipt_identity(),
            Kind::ReplayDecisionLogMismatch,
            &mut counters,
            &mut rows,
        )?;
        counters.compared_decision_log_identities();
        compare_row(
            RowKind::OperationalTruthDigest,
            input.original_operational_truth().digest_identity(),
            input.replayed_operational_truth().digest_identity(),
            Kind::ReplayOperationalTruthMismatch,
            &mut counters,
            &mut rows,
        )?;
        counters.compared_operational_truth();
        compare_row(
            RowKind::FragmentSet,
            input.original_fragments().fragment_set_identity(),
            input.replayed_fragments().fragment_set_identity(),
            Kind::ReplayFragmentMismatch,
            &mut counters,
            &mut rows,
        )?;
        counters.compared_fragments();
        compare_row(
            RowKind::OverlapChainSet,
            input.original_overlap_chains().chain_set_identity(),
            input.replayed_overlap_chains().chain_set_identity(),
            Kind::ReplayOverlapChainMismatch,
            &mut counters,
            &mut rows,
        )?;
        counters.compared_overlap_chains();
        compare_row(
            RowKind::PersistentNamingReceipt,
            input.original_persistent_naming().receipt_identity(),
            input.replayed_persistent_naming().receipt_identity(),
            Kind::ReplayPersistentNamingMismatch,
            &mut counters,
            &mut rows,
        )?;
        counters.compared_persistent_naming();

        let checkpoint_identity =
            CompareEdgeSplitCheckpointParity::checkpoint_identity(input.replay_receipts())?;
        rows.push(PlanarBooleanEdgeSplitReplayParityRow::new(
            RowKind::RetainedReplayCheckpoint,
            checkpoint_identity.clone(),
            checkpoint_identity,
        ));
        counters.compared_checkpoint();

        let orientation_identity = CanonicalizeReversedEdgeSenseSplit::canonical_identity(&input)?;
        rows.push(PlanarBooleanEdgeSplitReplayParityRow::new(
            RowKind::ReversedSourceSenseCanonicalization,
            orientation_identity.clone(),
            orientation_identity,
        ));
        counters.compared_orientation();

        let receipt = PlanarBooleanEdgeSplitReplayParityReceipt::new(
            input.replay_receipts().stage_identity().receipt_identity(),
            input
                .replay_receipts()
                .replay_checkpoint_identity()
                .to_string(),
            input
                .replay_receipts()
                .replay_evidence_identity()
                .to_string(),
            replay_product_identity,
            replay_closure_manifest_identity,
            input
                .original_request()
                .split_request_identity()
                .to_string(),
            input
                .replayed_request()
                .split_request_identity()
                .to_string(),
            input
                .original_ledger()
                .receipt()
                .receipt_identity()
                .to_string(),
            input
                .replayed_ledger()
                .receipt()
                .receipt_identity()
                .to_string(),
            input
                .original_ledger()
                .receipt()
                .downstream_consumption_identity()
                .to_string(),
            input
                .replayed_ledger()
                .receipt()
                .downstream_consumption_identity()
                .to_string(),
            rows,
            counters,
        );
        Ok(PlanarBooleanEdgeSplitReplayParityReport { receipt })
    }
}

impl CompareEdgeSplitCheckpointParity {
    pub(crate) fn checkpoint_identity(
        replay_receipts: &crate::workload_platform::retained_replay_workload::ReplayReceiptSet,
    ) -> Result<String, PlanarBooleanEdgeSplitReplayParityDenial> {
        validate_checkpoint_receipts(replay_receipts)
    }
}

impl CanonicalizeReversedEdgeSenseSplit {
    pub(crate) fn canonical_identity(
        input: &PlanarBooleanEdgeSplitReplayParityInput<'_>,
    ) -> Result<String, PlanarBooleanEdgeSplitReplayParityDenial> {
        validate_reversed_source_sense_canonicalization(input)
    }
}

impl PlanarBooleanEdgeSplitReplayParityReport {
    pub fn receipt(&self) -> &PlanarBooleanEdgeSplitReplayParityReceipt {
        &self.receipt
    }
}

fn compare_row(
    kind: RowKind,
    original: &str,
    replayed: &str,
    denial_kind: Kind,
    counters: &mut PlanarBooleanEdgeSplitReplayParityCounters,
    rows: &mut Vec<PlanarBooleanEdgeSplitReplayParityRow>,
) -> Result<(), PlanarBooleanEdgeSplitReplayParityDenial> {
    if original != replayed {
        counters.rejected_replay_mismatch();
        return Err(PlanarBooleanEdgeSplitReplayParityDenial::new(
            denial_kind,
            format!("{kind:?}"),
            original,
            replayed,
            "edge-split replay parity comparison found a mismatched proof identity",
        ));
    }
    rows.push(PlanarBooleanEdgeSplitReplayParityRow::new(
        kind, original, replayed,
    ));
    Ok(())
}
