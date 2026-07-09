use crate::lineage::data::{
    LineageDecisionLogDigestBasis, LineageEventBatchDigestBasis, PublishedLineageArtifact,
};

use super::primitive_terms::ReplayDigestBuilder;

pub(crate) fn digest_lineage_event_batch_surface(
    published_lineage: &PublishedLineageArtifact,
) -> [u8; 32] {
    let basis = published_lineage.observed_event_batch_digest_basis();
    let mut builder = ReplayDigestBuilder::new("WORTH.relational.replay.lineage.event_batch.v1")
        .branch_id(published_lineage.branch_id())
        .usize(basis.canonical_event_ids().len());
    for event_id in basis.canonical_event_ids() {
        builder = builder.u64(*event_id);
    }
    builder.finish()
}

pub(crate) fn digest_lineage_decision_log_surface(
    published_lineage: &PublishedLineageArtifact,
) -> [u8; 32] {
    let basis = published_lineage.observed_decision_log_digest_basis();
    let mut builder = ReplayDigestBuilder::new("WORTH.relational.replay.lineage.decision_log.v1")
        .branch_id(published_lineage.branch_id())
        .usize(basis.canonical_decision_kinds().len());
    for decision_kind in basis.canonical_decision_kinds() {
        builder = builder.label(decision_kind);
    }
    builder.finish()
}

pub(crate) fn digest_lineage_event_summary(basis: &LineageEventBatchDigestBasis) -> [u8; 32] {
    ReplayDigestBuilder::new("WORTH.relational.replay.lineage.event_summary.v1")
        .usize(basis.canonical_event_ids().len())
        .branch_id(basis.branch_id())
        .finish()
}

pub(crate) fn digest_lineage_decision_summary(basis: &LineageDecisionLogDigestBasis) -> [u8; 32] {
    ReplayDigestBuilder::new("WORTH.relational.replay.lineage.decision_summary.v1")
        .usize(basis.canonical_decision_kinds().len())
        .branch_id(basis.branch_id())
        .finish()
}
