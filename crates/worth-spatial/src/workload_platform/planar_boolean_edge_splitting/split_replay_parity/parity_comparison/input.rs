use super::super::parity_receipt::denial::{
    PlanarBooleanEdgeSplitReplayParityDenial, PlanarBooleanEdgeSplitReplayParityDenialKind as Kind,
};
use super::super::replay_execution::PlanarBooleanEdgeSplitReplayProduct;
use crate::workload_platform::planar_boolean_edge_splitting::{
    PlanarBooleanEdgeSplitRequest, PlanarBooleanOverlapEdgeChainSet,
    PlanarBooleanSplitDecisionLogQueryResult, PlanarBooleanSplitEdgeChainLedgerQueryResult,
    PlanarBooleanSplitEdgeFragmentSet, PlanarBooleanSplitOperationalTruthDigest,
    PlanarBooleanSplitPersistentNamingReceipt,
};
use crate::workload_platform::retained_replay_workload::ReplayReceiptSet;

pub struct PlanarBooleanEdgeSplitReplayParityInput<'a> {
    replay_product: Option<&'a PlanarBooleanEdgeSplitReplayProduct<'a>>,
    original_ledger: &'a PlanarBooleanSplitEdgeChainLedgerQueryResult,
    replayed_ledger: &'a PlanarBooleanSplitEdgeChainLedgerQueryResult,
    original_request: &'a PlanarBooleanEdgeSplitRequest,
    replayed_request: &'a PlanarBooleanEdgeSplitRequest,
    original_decision_log: &'a PlanarBooleanSplitDecisionLogQueryResult,
    replayed_decision_log: &'a PlanarBooleanSplitDecisionLogQueryResult,
    original_operational_truth: &'a PlanarBooleanSplitOperationalTruthDigest,
    replayed_operational_truth: &'a PlanarBooleanSplitOperationalTruthDigest,
    original_fragments: &'a PlanarBooleanSplitEdgeFragmentSet,
    replayed_fragments: &'a PlanarBooleanSplitEdgeFragmentSet,
    original_overlap_chains: &'a PlanarBooleanOverlapEdgeChainSet,
    replayed_overlap_chains: &'a PlanarBooleanOverlapEdgeChainSet,
    original_persistent_naming: &'a PlanarBooleanSplitPersistentNamingReceipt,
    replayed_persistent_naming: &'a PlanarBooleanSplitPersistentNamingReceipt,
    replay_receipts: &'a ReplayReceiptSet,
}

impl<'a> PlanarBooleanEdgeSplitReplayParityInput<'a> {
    pub fn from_replay_product(
        replay_product: &'a PlanarBooleanEdgeSplitReplayProduct<'a>,
    ) -> Result<Self, PlanarBooleanEdgeSplitReplayParityDenial> {
        let original = replay_product.original();
        let replayed = replay_product.replayed();
        if !replay_product.certifies_query_owned_replay_product() {
            return Err(PlanarBooleanEdgeSplitReplayParityDenial::new(
                Kind::ReplayProductNotQueryOwned,
                replay_product.product_identity(),
                "query-owned replay product with complete closure manifest",
                "incomplete replay product",
                "edge-split replay parity must consume a query-owned replay product",
            ));
        }
        reject_missing(
            replay_product
                .replay_receipts()
                .replay_checkpoint_identity(),
            Kind::MissingRetainedReplayReceipt,
            "retained replay checkpoint identity is required for edge-split replay parity",
        )?;
        reject_split_request_lineage(
            original.request(),
            replayed.request(),
            replay_product.replay_receipts(),
        )?;
        reject_ledger_request_mismatch(original.ledger(), original.request())?;
        reject_ledger_request_mismatch(replayed.ledger(), replayed.request())?;
        Ok(Self {
            replay_product: Some(replay_product),
            original_ledger: original.ledger(),
            replayed_ledger: replayed.ledger(),
            original_request: original.request(),
            replayed_request: replayed.request(),
            original_decision_log: original.decision_log(),
            replayed_decision_log: replayed.decision_log(),
            original_operational_truth: original.operational_truth(),
            replayed_operational_truth: replayed.operational_truth(),
            original_fragments: original.fragments(),
            replayed_fragments: replayed.fragments(),
            original_overlap_chains: original.overlap_chains(),
            replayed_overlap_chains: replayed.overlap_chains(),
            original_persistent_naming: original.naming(),
            replayed_persistent_naming: replayed.naming(),
            replay_receipts: replay_product.replay_receipts(),
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn from_query_products(
        original_ledger: &'a PlanarBooleanSplitEdgeChainLedgerQueryResult,
        replayed_ledger: &'a PlanarBooleanSplitEdgeChainLedgerQueryResult,
        original_request: &'a PlanarBooleanEdgeSplitRequest,
        replayed_request: &'a PlanarBooleanEdgeSplitRequest,
        original_decision_log: &'a PlanarBooleanSplitDecisionLogQueryResult,
        replayed_decision_log: &'a PlanarBooleanSplitDecisionLogQueryResult,
        original_operational_truth: &'a PlanarBooleanSplitOperationalTruthDigest,
        replayed_operational_truth: &'a PlanarBooleanSplitOperationalTruthDigest,
        original_fragments: &'a PlanarBooleanSplitEdgeFragmentSet,
        replayed_fragments: &'a PlanarBooleanSplitEdgeFragmentSet,
        original_overlap_chains: &'a PlanarBooleanOverlapEdgeChainSet,
        replayed_overlap_chains: &'a PlanarBooleanOverlapEdgeChainSet,
        original_persistent_naming: &'a PlanarBooleanSplitPersistentNamingReceipt,
        replayed_persistent_naming: &'a PlanarBooleanSplitPersistentNamingReceipt,
        replay_receipts: &'a ReplayReceiptSet,
    ) -> Result<Self, PlanarBooleanEdgeSplitReplayParityDenial> {
        reject_missing(
            replay_receipts.replay_checkpoint_identity(),
            Kind::MissingRetainedReplayReceipt,
            "retained replay checkpoint identity is required for edge-split replay parity",
        )?;
        reject_split_request_lineage(original_request, replayed_request, replay_receipts)?;
        reject_ledger_request_mismatch(original_ledger, original_request)?;
        reject_ledger_request_mismatch(replayed_ledger, replayed_request)?;
        reject_missing(
            original_ledger.receipt().receipt_identity(),
            Kind::MissingSplitLedgerReceipt,
            "original split ledger receipt identity is required for replay parity",
        )?;
        reject_missing(
            replayed_ledger.receipt().receipt_identity(),
            Kind::MissingSplitLedgerReceipt,
            "replayed split ledger receipt identity is required for replay parity",
        )?;
        reject_missing(
            original_decision_log.receipt().receipt_identity(),
            Kind::MissingDecisionLogReceipt,
            "original decision-log receipt identity is required for replay parity",
        )?;
        reject_missing(
            replayed_decision_log.receipt().receipt_identity(),
            Kind::MissingDecisionLogReceipt,
            "replayed decision-log receipt identity is required for replay parity",
        )?;
        reject_missing(
            original_operational_truth.digest_identity(),
            Kind::MissingOperationalTruthDigest,
            "original operational truth digest is required for replay parity",
        )?;
        reject_missing(
            replayed_operational_truth.digest_identity(),
            Kind::MissingOperationalTruthDigest,
            "replayed operational truth digest is required for replay parity",
        )?;
        Ok(Self {
            replay_product: None,
            original_ledger,
            replayed_ledger,
            original_request,
            replayed_request,
            original_decision_log,
            replayed_decision_log,
            original_operational_truth,
            replayed_operational_truth,
            original_fragments,
            replayed_fragments,
            original_overlap_chains,
            replayed_overlap_chains,
            original_persistent_naming,
            replayed_persistent_naming,
            replay_receipts,
        })
    }

    pub(crate) fn original_ledger(&self) -> &'a PlanarBooleanSplitEdgeChainLedgerQueryResult {
        self.original_ledger
    }

    pub(crate) fn replay_product(&self) -> Option<&'a PlanarBooleanEdgeSplitReplayProduct<'a>> {
        self.replay_product
    }

    pub(crate) fn replayed_ledger(&self) -> &'a PlanarBooleanSplitEdgeChainLedgerQueryResult {
        self.replayed_ledger
    }

    pub(crate) fn original_request(&self) -> &'a PlanarBooleanEdgeSplitRequest {
        self.original_request
    }

    pub(crate) fn replayed_request(&self) -> &'a PlanarBooleanEdgeSplitRequest {
        self.replayed_request
    }

    pub(crate) fn original_decision_log(&self) -> &'a PlanarBooleanSplitDecisionLogQueryResult {
        self.original_decision_log
    }

    pub(crate) fn replayed_decision_log(&self) -> &'a PlanarBooleanSplitDecisionLogQueryResult {
        self.replayed_decision_log
    }

    pub(crate) fn original_operational_truth(
        &self,
    ) -> &'a PlanarBooleanSplitOperationalTruthDigest {
        self.original_operational_truth
    }

    pub(crate) fn replayed_operational_truth(
        &self,
    ) -> &'a PlanarBooleanSplitOperationalTruthDigest {
        self.replayed_operational_truth
    }

    pub(crate) fn original_fragments(&self) -> &'a PlanarBooleanSplitEdgeFragmentSet {
        self.original_fragments
    }

    pub(crate) fn replayed_fragments(&self) -> &'a PlanarBooleanSplitEdgeFragmentSet {
        self.replayed_fragments
    }

    pub(crate) fn original_overlap_chains(&self) -> &'a PlanarBooleanOverlapEdgeChainSet {
        self.original_overlap_chains
    }

    pub(crate) fn replayed_overlap_chains(&self) -> &'a PlanarBooleanOverlapEdgeChainSet {
        self.replayed_overlap_chains
    }

    pub(crate) fn original_persistent_naming(
        &self,
    ) -> &'a PlanarBooleanSplitPersistentNamingReceipt {
        self.original_persistent_naming
    }

    pub(crate) fn replayed_persistent_naming(
        &self,
    ) -> &'a PlanarBooleanSplitPersistentNamingReceipt {
        self.replayed_persistent_naming
    }

    pub(crate) fn replay_receipts(&self) -> &'a ReplayReceiptSet {
        self.replay_receipts
    }
}

fn reject_split_request_lineage(
    original_request: &PlanarBooleanEdgeSplitRequest,
    replayed_request: &PlanarBooleanEdgeSplitRequest,
    replay_receipts: &ReplayReceiptSet,
) -> Result<(), PlanarBooleanEdgeSplitReplayParityDenial> {
    if original_request.split_request_identity() != replayed_request.split_request_identity() {
        return Err(PlanarBooleanEdgeSplitReplayParityDenial::new(
            Kind::ReplaySplitRequestMismatch,
            "split-request-identity",
            original_request.split_request_identity(),
            replayed_request.split_request_identity(),
            "edge-split replay parity requires the original and replayed ledgers to consume the same split request identity",
        ));
    }
    let Some(retained_replay_stage_identity) = original_request.retained_replay_stage_identity()
    else {
        return Err(PlanarBooleanEdgeSplitReplayParityDenial::new(
            Kind::MissingSplitRequestRetainedReplay,
            original_request.split_request_identity(),
            "retained replay evidence row",
            "none",
            "edge-split replay parity requires retained replay evidence in the split request evidence index",
        ));
    };
    if replayed_request.retained_replay_stage_identity() != Some(retained_replay_stage_identity) {
        return Err(PlanarBooleanEdgeSplitReplayParityDenial::new(
            Kind::ReplaySplitRequestMismatch,
            "retained-replay-stage-identity",
            retained_replay_stage_identity,
            replayed_request
                .retained_replay_stage_identity()
                .unwrap_or("none"),
            "original and replayed split requests must carry the same retained replay stage identity",
        ));
    }
    if replay_receipts.stage_identity().receipt_identity() != retained_replay_stage_identity {
        return Err(PlanarBooleanEdgeSplitReplayParityDenial::new(
            Kind::ForeignRetainedReplayReceipt,
            "retained-replay-stage-identity",
            retained_replay_stage_identity,
            replay_receipts.stage_identity().receipt_identity(),
            "retained replay receipt must be the receipt admitted into the split request evidence index",
        ));
    }
    Ok(())
}

fn reject_ledger_request_mismatch(
    ledger: &PlanarBooleanSplitEdgeChainLedgerQueryResult,
    request: &PlanarBooleanEdgeSplitRequest,
) -> Result<(), PlanarBooleanEdgeSplitReplayParityDenial> {
    if ledger.receipt().split_request_identity() != request.split_request_identity() {
        return Err(PlanarBooleanEdgeSplitReplayParityDenial::new(
            Kind::ReplaySplitRequestLedgerMismatch,
            "split-ledger-request-identity",
            request.split_request_identity(),
            ledger.receipt().split_request_identity(),
            "split replay parity requires each ledger to be assembled from the request being replayed",
        ));
    }
    Ok(())
}

fn reject_missing(
    identity: &str,
    kind: Kind,
    human_reason: &'static str,
) -> Result<(), PlanarBooleanEdgeSplitReplayParityDenial> {
    if identity.is_empty() {
        Err(PlanarBooleanEdgeSplitReplayParityDenial::new(
            kind,
            "edge-split-replay-parity-input",
            "non-empty identity",
            identity,
            human_reason,
        ))
    } else {
        Ok(())
    }
}
