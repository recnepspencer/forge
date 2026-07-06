use super::{
    admission::BlobRetentionReclaimAdmission,
    counters::BlobRetentionReclaimCounterSnapshot,
    permit::BlobRetentionReclaimPermit,
    residue::{BlobLocalizedReclaimResidue, BlobReclaimResidueKind},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobRetentionReclaimRequest {
    admission: BlobRetentionReclaimAdmission,
    residue_kind: BlobReclaimResidueKind,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct BlobRetentionSafeReclaimPlanner;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlobRetentionReclaimOutcome {
    Permitted(BlobRetentionReclaimPermit),
}

impl BlobRetentionReclaimOutcome {
    pub fn into_permit(self) -> BlobRetentionReclaimPermit {
        match self {
            Self::Permitted(permit) => permit,
        }
    }
}

impl BlobRetentionReclaimRequest {
    pub fn for_admission(admission: BlobRetentionReclaimAdmission) -> Self {
        Self {
            admission,
            residue_kind: BlobReclaimResidueKind::FailedReclaimBytes,
        }
    }

    pub fn with_abandoned_resume_residue(mut self) -> Self {
        self.residue_kind = BlobReclaimResidueKind::AbandonedResumeSessionBytes;
        self
    }

    pub const fn admission(&self) -> &BlobRetentionReclaimAdmission {
        &self.admission
    }
}

impl BlobRetentionSafeReclaimPlanner {
    pub fn new_store_owned() -> Self {
        Self
    }

    pub fn plan_reclaim(
        &mut self,
        request: BlobRetentionReclaimRequest,
    ) -> BlobRetentionReclaimOutcome {
        let counters = BlobRetentionReclaimCounterSnapshot::start()
            .with_orphan_candidate()
            .record_replay_convergence_check();
        let (candidate, s6_posture) = request.admission.into_parts();
        let counters = counters.with_residue_localization().with_permit();
        let residue =
            BlobLocalizedReclaimResidue::from_candidate(&candidate, request.residue_kind, counters);
        BlobRetentionReclaimOutcome::Permitted(BlobRetentionReclaimPermit::from_candidate(
            candidate, s6_posture, residue, counters,
        ))
    }
}
