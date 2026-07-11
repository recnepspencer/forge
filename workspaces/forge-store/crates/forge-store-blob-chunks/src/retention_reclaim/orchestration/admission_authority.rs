use forge_store_physical_isolation::BlobOrphanReclaimBarrier;

use crate::retention_reclaim::admission::reachability_gate::admit_via_reachability_gate;
use crate::retention_reclaim::counters::BlobRetentionReclaimCounterSnapshot;
use crate::retention_reclaim::denial::BlobRetentionReclaimDenial;
use crate::retention_reclaim::holds::BlobRetentionHold;
use crate::retention_reclaim::types::admission::BlobRetentionReclaimAdmission;
use crate::retention_reclaim::verification::hold_blocking::deny_retention_hold;
use crate::{BlobChunkIdentity, BlobChunkReachabilityRegistry, BlobReclaimPolicyEvidence};

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
        reclaim_policy_evidence: BlobReclaimPolicyEvidence,
    ) -> Result<BlobRetentionReclaimAdmission, BlobRetentionReclaimDenial> {
        admit_via_reachability_gate(reachability, chunk_identity, reclaim_policy_evidence, None)
    }

    pub fn admit_abandoned_resume_orphan(
        self,
        reachability: &BlobChunkReachabilityRegistry,
        chunk_identity: &BlobChunkIdentity,
        barrier: &BlobOrphanReclaimBarrier,
        reclaim_policy_evidence: BlobReclaimPolicyEvidence,
    ) -> Result<BlobRetentionReclaimAdmission, BlobRetentionReclaimDenial> {
        admit_via_reachability_gate(reachability, chunk_identity, reclaim_policy_evidence, Some(barrier))
    }

    pub fn deny_retention_hold(self, hold: &BlobRetentionHold) -> BlobRetentionReclaimDenial {
        deny_retention_hold(hold)
    }

    pub fn deny_missing_s6_reclaim_posture(self) -> BlobRetentionReclaimDenial {
        BlobRetentionReclaimDenial::MissingS6ReclaimPosture {
            counters: BlobRetentionReclaimCounterSnapshot::start()
                .with_orphan_candidate()
                .record_replay_convergence_check()
                .record_reclaim_policy_evidence_denial(),
        }
    }
}
