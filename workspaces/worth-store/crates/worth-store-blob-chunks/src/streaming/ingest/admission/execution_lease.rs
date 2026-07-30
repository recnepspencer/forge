use worth_store_io_scheduler::{BackgroundIdleCapacityLease, BackgroundIoPressureClass};

use crate::BlobStreamingIngestDenial;

/// Scheduler-issued ingest capacity retained for the complete chunking session.
#[derive(Debug, Eq, PartialEq)]
pub(crate) struct BlobStreamingIngestExecutionLease {
    _lease: BackgroundIdleCapacityLease,
}

impl BlobStreamingIngestExecutionLease {
    pub(crate) fn from_scheduler_lease(
        lease: BackgroundIdleCapacityLease,
    ) -> Result<Self, BlobStreamingIngestDenial> {
        if lease.class() != BackgroundIoPressureClass::IngestPressure {
            return Err(BlobStreamingIngestDenial::BackgroundPressureClassMismatch {
                actual: lease.class(),
            });
        }
        Ok(Self { _lease: lease })
    }
}
