use std::collections::BTreeSet;
use std::path::Path;
use std::process::Command;

use sha2::{Digest, Sha256};

use crate::arguments::CiScheduleLane;

const CI_SCHEDULE_SEED_COUNT: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::courtroom_campaign::bounded_residency_siege) struct ScheduleSeed(u64);

impl ScheduleSeed {
    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn new(value: u64) -> Self {
        Self(value)
    }

    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn value(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::courtroom_campaign::bounded_residency_siege) struct RevisionScheduleSeeds {
    revision: Box<str>,
    seeds: [ScheduleSeed; CI_SCHEDULE_SEED_COUNT],
}

impl RevisionScheduleSeeds {
    pub(in crate::courtroom_campaign::bounded_residency_siege) fn read(
        workspace: &Path,
    ) -> Result<Self, String> {
        let output = Command::new("git")
            .args(["rev-parse", "HEAD"])
            .current_dir(workspace)
            .output()
            .map_err(|error| format!("cannot resolve checked-out revision: {error}"))?;
        if !output.status.success() {
            return Err("git rev-parse HEAD failed for schedule seed derivation".into());
        }
        let revision = String::from_utf8(output.stdout)
            .map_err(|_| "checked-out revision was not UTF-8".to_owned())?;
        Self::derive(revision.trim())
    }

    pub(in crate::courtroom_campaign::bounded_residency_siege) fn derive(
        revision: &str,
    ) -> Result<Self, String> {
        Ok(Self {
            revision: revision.into(),
            seeds: revision_schedule_seeds(revision)?,
        })
    }

    pub(in crate::courtroom_campaign::bounded_residency_siege) fn revision(&self) -> &str {
        &self.revision
    }

    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn seeds(
        &self,
    ) -> &[ScheduleSeed; CI_SCHEDULE_SEED_COUNT] {
        &self.seeds
    }

    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn seed(
        &self,
        lane: CiScheduleLane,
    ) -> ScheduleSeed {
        self.seeds[lane.index()]
    }
}

pub(in crate::courtroom_campaign::bounded_residency_siege) fn revision_schedule_seeds(
    revision: &str,
) -> Result<[ScheduleSeed; CI_SCHEDULE_SEED_COUNT], String> {
    if revision.trim().is_empty() {
        return Err("schedule seed derivation requires a revision identity".into());
    }
    let seeds = (0..CI_SCHEDULE_SEED_COUNT)
        .map(|lane| revision_schedule_seed(revision, lane))
        .collect::<Vec<_>>();
    let distinct = seeds.iter().copied().collect::<BTreeSet<_>>();
    if distinct.len() != CI_SCHEDULE_SEED_COUNT {
        return Err("revision-derived schedule seeds collided".into());
    }
    seeds
        .try_into()
        .map_err(|_| "revision schedule seed inventory has the wrong width".to_owned())
}

fn revision_schedule_seed(revision: &str, lane: usize) -> ScheduleSeed {
    let mut digest = Sha256::new();
    digest.update(b"worth-store-c6-ci-schedule-seed-v2");
    digest.update((revision.len() as u64).to_le_bytes());
    digest.update(revision.as_bytes());
    let bytes: [u8; 32] = digest.finalize().into();
    let revision_identity = u64::from_le_bytes(bytes[..8].try_into().unwrap()) & !0x0f;
    ScheduleSeed::new(revision_identity | lane as u64)
}
