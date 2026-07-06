use forge_store_physical_isolation::BlobOrphanReclaimBarrier;

use crate::{
    BlobChunkIdentity, BlobChunkReachabilityRegistry, BlobReachabilityReclaimDecision,
    S6BlobReclaimNonClaimHandoff,
};

use super::{
    candidate::{BlobRetentionOrphanCandidate, BlobRetentionPhysicalOrphanClaim},
    denial::BlobRetentionReclaimDenial,
    holds::BlobRetentionHold,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobRetentionReclaimAdmission {
    candidate: BlobRetentionOrphanCandidate,
    s6_posture: S6BlobReclaimNonClaimHandoff,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlobRetentionReclaimAdmissionAuthority {
    _private: (),
}

impl BlobRetentionReclaimAdmissionAuthority {
    pub const fn store_owned() -> Self {
        Self { _private: () }
    }

    pub fn admit_reachability_orphan(
        self,
        reachability: &BlobChunkReachabilityRegistry,
        chunk_identity: &BlobChunkIdentity,
        s6_posture: S6BlobReclaimNonClaimHandoff,
    ) -> Result<BlobRetentionReclaimAdmission, BlobRetentionReclaimDenial> {
        self.reject_live_holds(reachability)?;
        let release = match reachability.reclaim_decision_for(chunk_identity) {
            BlobReachabilityReclaimDecision::ReclaimPermitted(release) => release,
            BlobReachabilityReclaimDecision::ReclaimDenied(_) => {
                return Err(BlobRetentionReclaimDenial::ReachabilityReclaimDenied {
                    counters: super::counters::BlobRetentionReclaimCounterSnapshot::start()
                        .with_orphan_candidate()
                        .record_replay_convergence_check()
                        .record_reachability_denial(),
                });
            }
        };
        reject_s6_scope_mismatch(&release, s6_posture)?;
        let physical_claim =
            BlobRetentionPhysicalOrphanClaim::from_admitted_s6_posture(&release, s6_posture)?;
        Ok(BlobRetentionReclaimAdmission {
            candidate: BlobRetentionOrphanCandidate::from_reachability_release(
                release,
                physical_claim,
            )?,
            s6_posture,
        })
    }

    pub fn admit_abandoned_resume_orphan(
        self,
        reachability: &BlobChunkReachabilityRegistry,
        chunk_identity: &BlobChunkIdentity,
        barrier: &BlobOrphanReclaimBarrier,
        s6_posture: S6BlobReclaimNonClaimHandoff,
    ) -> Result<BlobRetentionReclaimAdmission, BlobRetentionReclaimDenial> {
        self.reject_live_holds(reachability)?;
        let release = match reachability.reclaim_decision_for(chunk_identity) {
            BlobReachabilityReclaimDecision::ReclaimPermitted(release) => release,
            BlobReachabilityReclaimDecision::ReclaimDenied(_) => {
                return Err(BlobRetentionReclaimDenial::ReachabilityReclaimDenied {
                    counters: super::counters::BlobRetentionReclaimCounterSnapshot::start()
                        .with_orphan_candidate()
                        .record_replay_convergence_check()
                        .record_reachability_denial(),
                });
            }
        };
        reject_s6_scope_mismatch(&release, s6_posture)?;
        let physical_claim =
            BlobRetentionPhysicalOrphanClaim::from_admitted_s6_posture(&release, s6_posture)?;
        if !physical_claim.matches_resume_barrier(barrier) {
            return Err(
                BlobRetentionReclaimDenial::ReclaimCandidateIdentityMismatch {
                    counters: super::counters::BlobRetentionReclaimCounterSnapshot::start()
                        .with_orphan_candidate()
                        .record_replay_convergence_check()
                        .record_identity_mismatch_denial(),
                },
            );
        }
        Ok(BlobRetentionReclaimAdmission {
            candidate: BlobRetentionOrphanCandidate::from_abandoned_resume_barrier(
                release, barrier,
            )?,
            s6_posture,
        })
    }

    pub fn deny_retention_hold(self, hold: &BlobRetentionHold) -> BlobRetentionReclaimDenial {
        BlobRetentionReclaimDenial::ReclaimBlockedByRetentionHold {
            kind: hold.kind(),
            counters: super::counters::BlobRetentionReclaimCounterSnapshot::start()
                .with_orphan_candidate()
                .record_replay_convergence_check()
                .record_hold_denial(hold.kind()),
        }
    }

    pub fn deny_missing_s6_reclaim_posture(self) -> BlobRetentionReclaimDenial {
        BlobRetentionReclaimDenial::MissingS6ReclaimPosture {
            counters: super::counters::BlobRetentionReclaimCounterSnapshot::start()
                .with_orphan_candidate()
                .record_replay_convergence_check()
                .record_s6_posture_denial(),
        }
    }

    fn reject_live_holds(
        self,
        reachability: &BlobChunkReachabilityRegistry,
    ) -> Result<(), BlobRetentionReclaimDenial> {
        if let Some(hold) = reachability.first_retention_hold_for_reclaim() {
            return Err(self.deny_retention_hold(&hold));
        }
        Ok(())
    }
}

impl BlobRetentionReclaimAdmission {
    pub const fn candidate(&self) -> &BlobRetentionOrphanCandidate {
        &self.candidate
    }

    pub const fn s6_posture(&self) -> S6BlobReclaimNonClaimHandoff {
        self.s6_posture
    }

    pub(crate) fn into_parts(self) -> (BlobRetentionOrphanCandidate, S6BlobReclaimNonClaimHandoff) {
        (self.candidate, self.s6_posture)
    }
}

fn reject_s6_scope_mismatch(
    release: &crate::BlobReachabilityReclaimRelease,
    s6_posture: S6BlobReclaimNonClaimHandoff,
) -> Result<(), BlobRetentionReclaimDenial> {
    if s6_posture.carries_blob_lifecycle_claim()
        || s6_posture.security_metadata() != release.released_edges()[0].security_metadata()
    {
        return Err(BlobRetentionReclaimDenial::S6ReclaimPostureScopeMismatch {
            counters: super::counters::BlobRetentionReclaimCounterSnapshot::start()
                .with_orphan_candidate()
                .record_replay_convergence_check()
                .record_s6_posture_denial(),
        });
    }
    Ok(())
}
