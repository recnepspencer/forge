use sha2::{Digest, Sha256};

use super::OfflineStoreInspection;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RestartingOfflineScanReceipt {
    crash_boundaries: u64,
    available_boundaries: u64,
    scanned_bytes: u64,
    maximum_revalidated_bytes: u64,
    maximum_resident_buffer_bytes: u64,
    receipt_identity: [u8; 32],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestartingOfflineScanDenial {
    BaselineFailed,
    BoundaryExecutionFailed,
    CheckpointPersistenceFailed,
    RestartFailed,
    RestartChangedTruth,
    RestartExceededReadBound,
    RestartExceededBufferBound,
}

impl OfflineStoreInspection {
    pub fn certify_every_chunk_restart(
        &self,
    ) -> Result<RestartingOfflineScanReceipt, RestartingOfflineScanDenial> {
        self.certify_bounded_restart_matrix(usize::MAX)
    }

    pub fn certify_bounded_restart_matrix(
        &self,
        maximum_crash_cases: usize,
    ) -> Result<RestartingOfflineScanReceipt, RestartingOfflineScanDenial> {
        if maximum_crash_cases < 2 {
            return Err(RestartingOfflineScanDenial::BoundaryExecutionFailed);
        }
        let baseline = self
            .clone()
            .start()
            .map_err(|_| RestartingOfflineScanDenial::BaselineFailed)?
            .finish()
            .map_err(|_| RestartingOfflineScanDenial::BaselineFailed)?;
        let chunk_boundaries = baseline.counters().chunk_touches();
        let scanned_bytes = baseline.counters().bytes_read();
        let mut maximum_revalidated_bytes = 0_u64;
        let mut maximum_resident_buffer_bytes = 0_u64;
        let mut digest = Sha256::new();
        digest.update(b"worth-store-restarting-offline-scan-matrix-v1");
        digest.update(baseline.inspection_evidence_identity());
        let available_boundaries = chunk_boundaries.saturating_add(1);
        let boundaries = selected_boundaries(available_boundaries, maximum_crash_cases);
        digest.update(available_boundaries.to_be_bytes());
        digest.update((boundaries.len() as u64).to_be_bytes());
        for boundary in boundaries.iter().copied() {
            let mut interrupted = self
                .clone()
                .start()
                .map_err(|_| RestartingOfflineScanDenial::BoundaryExecutionFailed)?;
            for _ in 0..boundary {
                if interrupted
                    .advance()
                    .map_err(|_| RestartingOfflineScanDenial::BoundaryExecutionFailed)?
                    .is_none()
                {
                    return Err(RestartingOfflineScanDenial::BoundaryExecutionFailed);
                }
            }
            let encoded = interrupted
                .checkpoint()
                .and_then(|checkpoint| checkpoint.encode())
                .map_err(|_| RestartingOfflineScanDenial::CheckpointPersistenceFailed)?;
            drop(interrupted);
            let restarted = self
                .clone()
                .resume_from_checkpoint_bytes(&encoded)
                .map_err(|_| RestartingOfflineScanDenial::RestartFailed)?
                .finish()
                .map_err(|_| RestartingOfflineScanDenial::RestartFailed)?;
            if restarted.files() != baseline.files()
                || restarted.admitted_bytes() != baseline.admitted_bytes()
            {
                return Err(RestartingOfflineScanDenial::RestartChangedTruth);
            }
            let counters = restarted.counters();
            if counters.bytes_read() != scanned_bytes
                || counters.checkpoint_revalidated_bytes() > scanned_bytes
            {
                return Err(RestartingOfflineScanDenial::RestartExceededReadBound);
            }
            if counters.peak_buffer_bytes() > self.inspection_budget().max_buffer_bytes() as u64 {
                return Err(RestartingOfflineScanDenial::RestartExceededBufferBound);
            }
            maximum_revalidated_bytes =
                maximum_revalidated_bytes.max(counters.checkpoint_revalidated_bytes());
            maximum_resident_buffer_bytes =
                maximum_resident_buffer_bytes.max(counters.peak_buffer_bytes());
            digest.update(boundary.to_be_bytes());
            digest.update(Sha256::digest(&encoded));
            digest.update(restarted.inspection_evidence_identity());
        }
        Ok(RestartingOfflineScanReceipt {
            crash_boundaries: boundaries.len() as u64,
            available_boundaries,
            scanned_bytes,
            maximum_revalidated_bytes,
            maximum_resident_buffer_bytes,
            receipt_identity: digest.finalize().into(),
        })
    }
}

impl RestartingOfflineScanReceipt {
    pub const fn crash_boundaries(self) -> u64 {
        self.crash_boundaries
    }
    pub const fn scanned_bytes(self) -> u64 {
        self.scanned_bytes
    }
    pub const fn available_boundaries(self) -> u64 {
        self.available_boundaries
    }
    pub const fn maximum_revalidated_bytes(self) -> u64 {
        self.maximum_revalidated_bytes
    }
    pub const fn maximum_resident_buffer_bytes(self) -> u64 {
        self.maximum_resident_buffer_bytes
    }
    pub const fn receipt_identity(self) -> [u8; 32] {
        self.receipt_identity
    }
}

fn selected_boundaries(available: u64, maximum: usize) -> Vec<u64> {
    if available <= maximum as u64 {
        return (0..available).collect();
    }
    let maximum = maximum as u64;
    let last = available - 1;
    let mut boundaries = (0..maximum)
        .map(|index| index.saturating_mul(last) / (maximum - 1))
        .collect::<Vec<_>>();
    boundaries.sort_unstable();
    boundaries.dedup();
    boundaries
}
