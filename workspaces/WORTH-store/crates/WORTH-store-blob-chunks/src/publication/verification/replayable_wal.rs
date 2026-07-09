use worth_store_recovery_physics::{
    PartialPublicationClassification, RecoveredOrRejectedPartialPublication,
};

use super::super::{BlobPublicationCounterSnapshot, BlobPublicationDenial};

pub(crate) fn verify_replayable_classification(
    classification: &PartialPublicationClassification,
) -> Result<(), BlobPublicationDenial> {
    if matches!(
        classification.recovered_or_rejected(),
        RecoveredOrRejectedPartialPublication::ReplayableUnacknowledgedWal { .. }
    ) {
        Ok(())
    } else {
        Err(BlobPublicationDenial::WalReplayEvidenceRequired {
            counters: BlobPublicationCounterSnapshot::start().with_denied_promotion(),
        })
    }
}

pub(crate) fn replayable_durable_wal<'a>(
    classification: &'a PartialPublicationClassification,
) -> Option<&'a worth_store_recovery_physics::UnacknowledgedDurableWal> {
    classification
        .recovered_or_rejected()
        .replayable_durable_wal()
}
