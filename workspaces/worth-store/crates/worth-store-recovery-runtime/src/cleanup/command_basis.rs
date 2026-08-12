use std::cell::RefCell;

use worth_store::physical_runtime::{
    StoreRecoveryCleanupAttempt, StoreRecoveryCleanupPlan,
};

use crate::progression::ReopenedPhysicalRecovery;

use super::RecoveryCleanupEligibility;

/// Borrowed, proof-bearing Store inputs used to construct one exact cleanup
/// command. Runtime eligibility narrows the candidate set; the Store command
/// independently revalidates checkpoint coverage before accepting it.
pub(super) struct RecoveryCleanupCommandBasis<'a> {
    plan: RefCell<StoreRecoveryCleanupPlan<'a>>,
}

impl<'a> RecoveryCleanupCommandBasis<'a> {
    pub(super) fn from_reopened(
        reopened: &'a ReopenedPhysicalRecovery,
        candidates: &[RecoveryCleanupEligibility],
    ) -> Option<Self> {
        let checkpoint = reopened.state.selection.checkpoint()?.checkpoint();
        let plan = reopened.state.coordination.owner().admit_cleanup_plan(
            &reopened.state.authority.media,
            &reopened.reopened,
            checkpoint,
            candidates
                .iter()
                .map(RecoveryCleanupEligibility::verified_artifact),
        )
        .ok()?;
        Some(Self {
            plan: RefCell::new(plan),
        })
    }

    pub(super) fn plan_identity(&self) -> [u8; 32] {
        self.plan.borrow().identity()
    }

    pub(super) fn execute(
        &self,
        coordination: &worth_store::physical_runtime::PhysicalRecoveryCoordination,
        media: &worth_store::physical_runtime::AdmittedRecoveryFilesystemMedia,
        artifact: worth_store_recovery_physics::WalSegmentArtifactIdentity,
    ) -> StoreRecoveryCleanupAttempt {
        coordination.execute_cleanup_candidate(media, &mut self.plan.borrow_mut(), artifact)
    }
}
