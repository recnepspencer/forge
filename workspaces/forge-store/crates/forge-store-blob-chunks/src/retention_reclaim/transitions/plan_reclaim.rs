use crate::retention_reclaim::counters::BlobRetentionReclaimCounterSnapshot;
use crate::retention_reclaim::permit::BlobRetentionReclaimPermit;
use crate::retention_reclaim::residue::{BlobLocalizedReclaimResidue, BlobReclaimResidueKind};
use crate::retention_reclaim::types::admission::BlobRetentionReclaimAdmission;
pub(crate) fn construct_plan_counters() -> BlobRetentionReclaimCounterSnapshot {
    BlobRetentionReclaimCounterSnapshot::start()
        .with_orphan_candidate()
        .record_replay_convergence_check()
}

pub(crate) fn transition_plan_reclaim(
    admission: BlobRetentionReclaimAdmission,
    residue_kind: BlobReclaimResidueKind,
    counters: BlobRetentionReclaimCounterSnapshot,
) -> BlobRetentionReclaimPermit {
    let (candidate, s6_posture) = admission.into_parts();
    let counters = counters.with_residue_localization().with_permit();
    let residue = BlobLocalizedReclaimResidue::from_candidate(&candidate, residue_kind, counters);
    BlobRetentionReclaimPermit::from_candidate(candidate, s6_posture, residue, counters)
}
