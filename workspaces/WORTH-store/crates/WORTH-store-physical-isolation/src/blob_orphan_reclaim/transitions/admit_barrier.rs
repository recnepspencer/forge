use crate::blob_orphan_reclaim::counters::BlobOrphanReclaimCounterSnapshot;
use crate::blob_orphan_reclaim::denial::BlobOrphanReclaimDenial;
use crate::blob_orphan_reclaim::types::barrier::BlobOrphanReclaimBarrier;
use crate::blob_orphan_reclaim::types::partial_orphan::BlobPartialChunkOrphan;

pub(crate) fn transition_admit_barrier(
    orphan: BlobPartialChunkOrphan,
    reachable: bool,
) -> Result<BlobOrphanReclaimBarrier, BlobOrphanReclaimDenial> {
    if reachable {
        return Err(BlobOrphanReclaimDenial::AlreadyReachable);
    }
    Ok(BlobOrphanReclaimBarrier::construct(
        orphan,
        BlobOrphanReclaimCounterSnapshot::start().with_barrier(),
    ))
}