use crate::runtime::{
    ForgeQueryAspectTouch, ForgeQueryDerivedMaterializationTarget, ForgeQueryLiveArtifactTarget,
    ForgeQueryWriteReceipt,
};

use forge_runtime_bridge::facade::BridgeBatchMutationAuthorityBundle;

pub(super) struct ForgeQueryBatchReceiptAggregates {
    pub(super) touched_aspects: Vec<ForgeQueryAspectTouch>,
    pub(super) affected_live_view_targets: Vec<ForgeQueryLiveArtifactTarget>,
    pub(super) affected_derived_view_targets: Vec<ForgeQueryDerivedMaterializationTarget>,
    pub(super) considered_computed_view_count: usize,
    pub(super) considered_effect_count: usize,
    pub(super) delivered_effect_count: usize,
    pub(super) pending_write_intent_count: usize,
    pub(super) suppressed_effect_count: usize,
    pub(super) meaningful_effect_suppression_count: usize,
    pub(super) effect_expression_failure_count: usize,
    pub(super) refresh_fallback: bool,
}

pub(super) fn derive_batch_receipt_aggregates(
    write_receipts: &[ForgeQueryWriteReceipt],
) -> ForgeQueryBatchReceiptAggregates {
    let mut touched_aspects = std::collections::BTreeSet::new();
    for touch in write_receipts
        .iter()
        .flat_map(|receipt| receipt.deltas())
        .flat_map(|delta| delta.admitted_touched_aspects())
    {
        touched_aspects.insert(touch.clone());
    }

    let mut affected_live_view_targets = write_receipts
        .iter()
        .flat_map(|receipt| receipt.affected_live_view_targets().iter().cloned())
        .collect::<Vec<_>>();
    affected_live_view_targets.sort();
    affected_live_view_targets.dedup();

    let mut affected_derived_view_targets = write_receipts
        .iter()
        .flat_map(|receipt| receipt.affected_derived_view_targets().iter().cloned())
        .collect::<Vec<_>>();
    affected_derived_view_targets.sort();
    affected_derived_view_targets.dedup();

    ForgeQueryBatchReceiptAggregates {
        touched_aspects: touched_aspects.into_iter().collect(),
        affected_live_view_targets,
        affected_derived_view_targets,
        considered_computed_view_count: write_receipts
            .iter()
            .map(ForgeQueryWriteReceipt::considered_computed_view_count)
            .sum(),
        considered_effect_count: write_receipts
            .iter()
            .map(ForgeQueryWriteReceipt::considered_effect_count)
            .sum(),
        delivered_effect_count: write_receipts
            .iter()
            .map(ForgeQueryWriteReceipt::delivered_effect_count)
            .sum(),
        pending_write_intent_count: write_receipts
            .iter()
            .map(ForgeQueryWriteReceipt::pending_write_intent_count)
            .sum(),
        suppressed_effect_count: write_receipts
            .iter()
            .map(ForgeQueryWriteReceipt::suppressed_effect_count)
            .sum(),
        meaningful_effect_suppression_count: write_receipts
            .iter()
            .map(ForgeQueryWriteReceipt::meaningful_effect_suppression_count)
            .sum(),
        effect_expression_failure_count: write_receipts
            .iter()
            .map(ForgeQueryWriteReceipt::effect_expression_failure_count)
            .sum(),
        refresh_fallback: write_receipts
            .iter()
            .any(ForgeQueryWriteReceipt::refresh_fallback),
    }
}

pub(super) fn batch_bridge_evidence_from_receipts(
    write_receipts: &[ForgeQueryWriteReceipt],
) -> Option<BridgeBatchMutationAuthorityBundle> {
    let components = write_receipts
        .iter()
        .filter_map(|receipt| receipt.inner.bridge_authority.clone())
        .collect::<Vec<_>>();
    if components.len() != write_receipts.len() {
        return None;
    }
    BridgeBatchMutationAuthorityBundle::from_components(&components)
}
