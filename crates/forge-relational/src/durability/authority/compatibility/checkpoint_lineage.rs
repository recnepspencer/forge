use crate::durability::data::{DurabilityError, DurableCheckpoint, RecoveryFailureClass};
use crate::lineage::data::LineageCheckpointDigestBasis;

pub(super) fn validate_checkpoint_lineage_artifact(
    checkpoint: &DurableCheckpoint,
) -> Result<(), DurabilityError> {
    let published_lineage_commit_count = checkpoint
        .envelopes
        .iter()
        .filter(|envelope| envelope.has_lineage_authority())
        .count();
    let canonical_published_event_ids = checkpoint
        .envelopes
        .iter()
        .flat_map(|envelope| {
            envelope
                .lineage_digest_basis()
                .canonical_event_ids()
                .iter()
                .copied()
        })
        .collect();
    let published_lineage_event_count = checkpoint
        .envelopes
        .iter()
        .map(|envelope| envelope.lineage_digest_basis().lineage_event_count())
        .sum();
    let published_lineage_decision_count = checkpoint
        .envelopes
        .iter()
        .map(|envelope| envelope.lineage_digest_basis().lineage_decision_count())
        .sum();
    let observed_basis = LineageCheckpointDigestBasis::new(
        published_lineage_commit_count,
        canonical_published_event_ids,
        published_lineage_event_count,
        published_lineage_decision_count,
    );
    if checkpoint.lineage.digest_basis() != &observed_basis {
        return Err(DurabilityError::new(
            RecoveryFailureClass::ReplayFailure,
            "durable checkpoint lineage artifact basis drifted from canonical published lineage",
        ));
    }
    Ok(())
}
