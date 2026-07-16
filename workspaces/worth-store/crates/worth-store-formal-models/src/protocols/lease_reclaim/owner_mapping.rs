use worth_store_physical_isolation::{
    ActiveHazardLease, CrashStableReclaimReuseFence, FreeReuseFenceDenial,
    HazardLeaseReleaseReceipt, LeaseExpiryPosture, OwnedCopyStableReadReceipt,
    ReadHandleRevocationReceipt, ReclaimDecision, ReclaimEligibilityProof,
};

use super::LeaseReclaimAction;

pub const fn map_active_lease(lease: ActiveHazardLease) -> LeaseReclaimAction {
    LeaseReclaimAction::LeaseAcquired {
        slot: lease.slot().get(),
        generation: lease.generation().get(),
    }
}

pub fn map_reclaim_eligibility(proof: &ReclaimEligibilityProof) -> LeaseReclaimAction {
    match proof.decision() {
        ReclaimDecision::Eligible => LeaseReclaimAction::ReclaimAdmitted,
        ReclaimDecision::Blocked(_) => LeaseReclaimAction::ReclaimDeniedByLiveLease,
    }
}

pub fn map_identity_reuse_attempt(
    attempt: &Result<CrashStableReclaimReuseFence, FreeReuseFenceDenial>,
) -> LeaseReclaimAction {
    match attempt {
        Ok(fence) => {
            let generation = fence.generation_advance();
            LeaseReclaimAction::IdentityReuseAdmitted {
                old_generation: generation.old_identity().generation().get(),
                new_generation: generation.reused_identity().generation().get(),
            }
        }
        Err(_) => LeaseReclaimAction::IdentityReuseDenied,
    }
}

pub const fn map_release(receipt: HazardLeaseReleaseReceipt) -> LeaseReclaimAction {
    LeaseReclaimAction::LeaseReleased {
        slot: receipt.slot().get(),
        generation: receipt.generation().get(),
    }
}

pub const fn map_revocation(receipt: ReadHandleRevocationReceipt) -> LeaseReclaimAction {
    LeaseReclaimAction::LeaseRevoked {
        slot: receipt.slot().get(),
        generation: receipt.generation().get(),
    }
}

pub const fn map_owned_copy(receipt: OwnedCopyStableReadReceipt) -> LeaseReclaimAction {
    LeaseReclaimAction::OwnedCopyStabilized {
        slot: receipt.slot().get(),
        generation: receipt.generation().get(),
    }
}

pub const fn map_expiry(posture: LeaseExpiryPosture) -> LeaseReclaimAction {
    match posture {
        LeaseExpiryPosture::ExpiredWithoutAuthority { slot, generation } => {
            LeaseReclaimAction::LeaseExpiredWithoutAuthority {
                slot: slot.get(),
                generation: generation.get(),
            }
        }
        LeaseExpiryPosture::Released(receipt) => map_release(receipt),
        LeaseExpiryPosture::Revoked(receipt) => map_revocation(receipt),
        LeaseExpiryPosture::OwnedCopyStable(receipt) => map_owned_copy(receipt),
    }
}
