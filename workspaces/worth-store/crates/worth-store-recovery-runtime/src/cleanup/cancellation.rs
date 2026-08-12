use crate::progression::ReopenedPhysicalRecovery;

use super::command_basis::RecoveryCleanupCommandBasis;
use super::plan::{build_plan, RecoveryCleanupPlanBasis};

/// Plan-bound request to stop optional post-publication cleanup at one exact
/// between-command safe point.
///
/// This value is neither `Clone` nor `Copy`; another recovery plan cannot
/// reuse it.
pub struct PhysicalRecoveryCleanupCancellation {
    plan: [u8; 32],
    settled_actions: u64,
}

impl PhysicalRecoveryCleanupCancellation {
    const fn new(plan: [u8; 32], settled_actions: u64) -> Self {
        Self {
            plan,
            settled_actions,
        }
    }

    pub(super) fn admit(self, plan: [u8; 32], action_count: u64) -> Option<u64> {
        (self.plan == plan && self.settled_actions < action_count).then_some(self.settled_actions)
    }
}

pub(crate) fn before_first(
    reopened: &ReopenedPhysicalRecovery,
) -> Option<PhysicalRecoveryCleanupCancellation> {
    cancellation_at(reopened, 0)
}

pub(crate) fn after_action(
    reopened: &ReopenedPhysicalRecovery,
    action_ordinal: u64,
) -> Option<PhysicalRecoveryCleanupCancellation> {
    cancellation_at(reopened, action_ordinal.checked_add(1)?)
}

fn cancellation_at(
    reopened: &ReopenedPhysicalRecovery,
    settled_actions: u64,
) -> Option<PhysicalRecoveryCleanupCancellation> {
    let mut plan = build_plan(RecoveryCleanupPlanBasis {
        selection: &reopened.state.selection,
        base: &reopened.state.base,
        publication: &reopened.expectation,
        fates: &reopened.state.fates,
        limits: reopened.state.authority.limits.declaration(),
    });
    let command_basis = RecoveryCleanupCommandBasis::from_reopened(reopened, plan.candidates())?;
    plan.bind_authority_identity(command_basis.plan_identity());
    (settled_actions < plan.candidates().len() as u64).then_some(
        PhysicalRecoveryCleanupCancellation::new(plan.identity(), settled_actions),
    )
}
