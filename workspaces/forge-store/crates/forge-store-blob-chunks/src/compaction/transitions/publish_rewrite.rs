use crate::compaction::receipt_construction::published_observation::BlobCompactionPublishedObservation;
use crate::compaction::transitions::execute_rewrite::BlobCompactionRewriteExecution;
use crate::BlobCompactionDenial;

pub(crate) fn publish_rewrite(
    execution: BlobCompactionRewriteExecution,
) -> Result<BlobCompactionPublishedObservation, BlobCompactionDenial> {
    BlobCompactionPublishedObservation::publish(execution)
}