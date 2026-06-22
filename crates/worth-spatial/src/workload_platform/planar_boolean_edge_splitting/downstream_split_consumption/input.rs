use crate::workload_platform::evidence_ledger::{
    SpatialEvidenceLookupProduct, SpatialGeometryEvidenceTouchAuthority,
};
use crate::workload_platform::planar_boolean_edge_splitting::{
    PlanarBooleanEdgeSplitReplayParityReceipt, PlanarBooleanSplitChainValidationReceipt,
    PlanarBooleanSplitDecisionLogReceipt, PlanarBooleanSplitEdgeChainLedgerReceipt,
    PlanarBooleanSplitPersistentNamingReceipt,
};

pub struct PlanarBooleanDownstreamSplitConsumptionInput<'a> {
    split_ledger_receipt: &'a PlanarBooleanSplitEdgeChainLedgerReceipt,
    decision_log_receipt: &'a PlanarBooleanSplitDecisionLogReceipt,
    validation_receipt: &'a PlanarBooleanSplitChainValidationReceipt,
    persistent_naming_receipt: &'a PlanarBooleanSplitPersistentNamingReceipt,
    replay_parity_receipt: &'a PlanarBooleanEdgeSplitReplayParityReceipt,
    spatial_touch_authority: &'a SpatialGeometryEvidenceTouchAuthority,
    spatial_lookup: &'a SpatialEvidenceLookupProduct,
}

impl<'a> PlanarBooleanDownstreamSplitConsumptionInput<'a> {
    pub fn from_split_ledger_receipt(
        split_ledger_receipt: &'a PlanarBooleanSplitEdgeChainLedgerReceipt,
        decision_log_receipt: &'a PlanarBooleanSplitDecisionLogReceipt,
        validation_receipt: &'a PlanarBooleanSplitChainValidationReceipt,
        persistent_naming_receipt: &'a PlanarBooleanSplitPersistentNamingReceipt,
        replay_parity_receipt: &'a PlanarBooleanEdgeSplitReplayParityReceipt,
        spatial_touch_authority: &'a SpatialGeometryEvidenceTouchAuthority,
        spatial_lookup: &'a SpatialEvidenceLookupProduct,
    ) -> Self {
        Self {
            split_ledger_receipt,
            decision_log_receipt,
            validation_receipt,
            persistent_naming_receipt,
            replay_parity_receipt,
            spatial_touch_authority,
            spatial_lookup,
        }
    }

    pub(crate) fn split_ledger_receipt(&self) -> &'a PlanarBooleanSplitEdgeChainLedgerReceipt {
        self.split_ledger_receipt
    }

    pub(crate) fn decision_log_receipt(&self) -> &'a PlanarBooleanSplitDecisionLogReceipt {
        self.decision_log_receipt
    }

    pub(crate) fn validation_receipt(&self) -> &'a PlanarBooleanSplitChainValidationReceipt {
        self.validation_receipt
    }

    pub(crate) fn persistent_naming_receipt(
        &self,
    ) -> &'a PlanarBooleanSplitPersistentNamingReceipt {
        self.persistent_naming_receipt
    }

    pub(crate) fn replay_parity_receipt(&self) -> &'a PlanarBooleanEdgeSplitReplayParityReceipt {
        self.replay_parity_receipt
    }

    pub(crate) fn spatial_touch_authority(&self) -> &'a SpatialGeometryEvidenceTouchAuthority {
        self.spatial_touch_authority
    }

    pub(crate) fn spatial_lookup(&self) -> &'a SpatialEvidenceLookupProduct {
        self.spatial_lookup
    }
}
