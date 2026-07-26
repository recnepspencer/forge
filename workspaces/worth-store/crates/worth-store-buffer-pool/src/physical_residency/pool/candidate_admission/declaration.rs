use super::*;

impl PoolInner {
    pub(super) fn validate_candidate_set(
        &self,
        keys: &[PhysicalCandidateFrameKey],
    ) -> Result<std::collections::VecDeque<PhysicalCandidateFrameKey>, PhysicalResidencyDenial>
    {
        if keys.is_empty() {
            return Err(self.record_denial(PhysicalResidencyDenial::EmptyCandidateBatch));
        }
        let mut unique = std::collections::HashSet::new();
        unique
            .try_reserve(keys.len())
            .map_err(|_| self.record_denial(PhysicalResidencyDenial::AllocationFailed))?;
        let mut complete_artifacts = std::collections::HashSet::new();
        complete_artifacts
            .try_reserve(keys.len())
            .map_err(|_| self.record_denial(PhysicalResidencyDenial::AllocationFailed))?;
        let mut fragmented_artifacts = std::collections::HashSet::new();
        fragmented_artifacts
            .try_reserve(keys.len())
            .map_err(|_| self.record_denial(PhysicalResidencyDenial::AllocationFailed))?;
        for candidate in keys {
            let key = candidate.frame_key();
            if let Err(reason) = self.validate_key(key) {
                return Err(self.record_denial(reason));
            }
            if !unique.insert(key) {
                return Err(self.record_denial(PhysicalResidencyDenial::DuplicateCandidateIdentity));
            }
            let artifact = key.coordinate().artifact();
            if candidate.is_complete_artifact() {
                if fragmented_artifacts.contains(&artifact) || !complete_artifacts.insert(artifact)
                {
                    return Err(
                        self.record_denial(PhysicalResidencyDenial::CandidateCoverageConflict)
                    );
                }
            } else {
                if complete_artifacts.contains(&artifact) {
                    return Err(
                        self.record_denial(PhysicalResidencyDenial::CandidateCoverageConflict)
                    );
                }
                fragmented_artifacts.insert(artifact);
            }
        }
        let mut admitted = std::collections::VecDeque::new();
        admitted
            .try_reserve_exact(keys.len())
            .map_err(|_| self.record_denial(PhysicalResidencyDenial::AllocationFailed))?;
        admitted.extend(keys.iter().copied());
        Ok(admitted)
    }
}

pub(in crate::physical_residency::pool) fn candidate_batch_operation_bytes(
    candidate_count: std::num::NonZeroUsize,
) -> Option<std::num::NonZeroU64> {
    const HASH_TABLE_CAPACITY_FACTOR: usize = 2;
    const HASH_TABLE_FIXED_CONTROL_BYTES: usize = 32;

    let count = candidate_count.get();
    let queue = count.checked_mul(std::mem::size_of::<PhysicalCandidateFrameKey>())?;
    let projected_keys = queue;
    let unique = count.checked_mul(
        HASH_TABLE_CAPACITY_FACTOR
            .checked_mul(std::mem::size_of::<PhysicalFrameKey>().checked_add(1)?)?,
    )?;
    let artifact_set = count.checked_mul(
        HASH_TABLE_CAPACITY_FACTOR
            .checked_mul(std::mem::size_of::<RecordArtifactFile>().checked_add(1)?)?,
    )?;
    let artifacts = artifact_set.checked_mul(2)?;
    let fixed_control = HASH_TABLE_FIXED_CONTROL_BYTES.checked_mul(3)?;
    let bytes = queue
        .checked_add(projected_keys)?
        .checked_add(unique)?
        .checked_add(artifacts)?
        .checked_add(fixed_control)?;
    std::num::NonZeroU64::new(u64::try_from(bytes).ok()?)
}
