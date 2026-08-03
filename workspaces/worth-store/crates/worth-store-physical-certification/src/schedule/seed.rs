use super::ScheduleReplayDenial;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SchedulePerturbationSeed(u64);

impl SchedulePerturbationSeed {
    pub const fn from_u64(seed: u64) -> Self {
        Self(seed)
    }

    pub fn required(seed: Option<u64>) -> Result<Self, ScheduleReplayDenial> {
        seed.map(Self::from_u64)
            .ok_or(ScheduleReplayDenial::MissingSeed)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

pub const CI_SCHEDULE_PERTURBATION_SEED_COUNT: usize = 16;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceClosureScheduleSeeds {
    source_closure_digest: [u8; 32],
    seeds: [SchedulePerturbationSeed; CI_SCHEDULE_PERTURBATION_SEED_COUNT],
}

impl SourceClosureScheduleSeeds {
    pub fn derive(source_closure_digest: [u8; 32]) -> Result<Self, ScheduleReplayDenial> {
        if source_closure_digest == [0; 32] {
            return Err(ScheduleReplayDenial::MissingSourceClosureDigest);
        }
        let seeds = std::array::from_fn(|lane| derive_lane(source_closure_digest, lane));
        let distinct = seeds.iter().copied().collect::<BTreeSet<_>>();
        if distinct.len() != CI_SCHEDULE_PERTURBATION_SEED_COUNT {
            return Err(ScheduleReplayDenial::ScheduleSeedCollision);
        }
        Ok(Self {
            source_closure_digest,
            seeds,
        })
    }

    pub const fn source_closure_digest(&self) -> [u8; 32] {
        self.source_closure_digest
    }

    pub const fn seeds(&self) -> &[SchedulePerturbationSeed; CI_SCHEDULE_PERTURBATION_SEED_COUNT] {
        &self.seeds
    }

    pub const fn seed(&self, lane: usize) -> Option<SchedulePerturbationSeed> {
        if lane < CI_SCHEDULE_PERTURBATION_SEED_COUNT {
            Some(self.seeds[lane])
        } else {
            None
        }
    }

    pub const fn crash_seam(&self, lane: usize) -> Option<super::C7DurabilityCrashSeam> {
        super::C7DurabilityCrashSeam::for_ci_lane(lane)
    }
}

fn derive_lane(source_closure_digest: [u8; 32], lane: usize) -> SchedulePerturbationSeed {
    let mut digest = Sha256::new();
    digest.update(b"store.physical-reconstruction.c7.schedule.v1");
    digest.update(source_closure_digest);
    digest.update((lane as u64).to_le_bytes());
    let bytes: [u8; 32] = digest.finalize().into();
    SchedulePerturbationSeed::from_u64(u64::from_le_bytes(bytes[..8].try_into().unwrap()))
}

#[cfg(test)]
mod tests {
    use super::{SourceClosureScheduleSeeds, CI_SCHEDULE_PERTURBATION_SEED_COUNT};
    use crate::schedule::C7DurabilityCrashSeam;
    use std::collections::BTreeMap;

    #[test]
    fn sixteen_source_closure_lanes_are_distinct_and_cover_each_seam_twice() {
        let lanes = SourceClosureScheduleSeeds::derive([0xC7; 32]).unwrap();
        let mut coverage = BTreeMap::new();
        for lane in 0..CI_SCHEDULE_PERTURBATION_SEED_COUNT {
            *coverage.entry(lanes.crash_seam(lane).unwrap()).or_insert(0) += 1;
        }
        assert_eq!(coverage.len(), C7DurabilityCrashSeam::ALL.len());
        assert!(coverage.values().all(|count| *count == 2));
        assert_eq!(lanes.crash_seam(CI_SCHEDULE_PERTURBATION_SEED_COUNT), None);
    }
}
