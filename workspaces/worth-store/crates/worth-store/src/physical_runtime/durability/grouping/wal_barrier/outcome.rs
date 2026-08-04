use worth_signal::facade::{AsyncNodeAdmissionClass, AsyncNodeConditionBlockClass};
use worth_store_physical_backend::ArtifactTreeFailure;

use super::PhysicalWalGroupBarrierDeclarationDenial;
use crate::physical_runtime::{
    PhysicalDurabilityGroupBasis, PhysicalExecutorCommandDenial, PhysicalSchedulerDenial,
    PhysicalWorkPreEffectDenial, SealedPhysicalDurabilityGroupMembers,
    WalDurablePhysicalMutationMembers,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWalGroupBarrierFailureCause {
    RuntimeReleased,
    Declaration(PhysicalWalGroupBarrierDeclarationDenial),
    SubmissionDenied(crate::physical_runtime::PhysicalWorkSubmissionDenial),
    SubmissionDeferred(crate::physical_runtime::PhysicalWorkSubmissionDeferred),
    SubmissionStale(crate::physical_runtime::PhysicalWorkSubmissionStale),
    SubmissionFailed(crate::physical_runtime::PhysicalWorkSubmissionFailure),
    PreEffect(PhysicalWorkPreEffectDenial),
    DependencyBlocked {
        class: AsyncNodeAdmissionClass,
        condition: Option<AsyncNodeConditionBlockClass>,
    },
    SchedulerReservationDenied(
        worth_store_io_scheduler::foreground_reservation::PhysicalInstanceForegroundAdmissionDenial,
    ),
    Scheduler(PhysicalSchedulerDenial),
    Command(PhysicalExecutorCommandDenial),
    MediaDeniedBeforeEffect(ArtifactTreeFailure),
}

pub struct IndeterminatePhysicalWalGroupBarrier {
    appended: SealedPhysicalDurabilityGroupMembers,
}

pub enum PhysicalWalGroupBarrierOutcome {
    Durable(WalDurablePhysicalMutationMembers),
    BarrierNotStarted {
        appended: SealedPhysicalDurabilityGroupMembers,
        cause: PhysicalWalGroupBarrierFailureCause,
    },
    Indeterminate(IndeterminatePhysicalWalGroupBarrier),
}

impl IndeterminatePhysicalWalGroupBarrier {
    pub(in crate::physical_runtime) const fn new(
        appended: SealedPhysicalDurabilityGroupMembers,
    ) -> Self {
        Self { appended }
    }

    pub const fn basis(&self) -> PhysicalDurabilityGroupBasis {
        self.appended.basis()
    }

    pub fn member_count(&self) -> usize {
        self.appended.members().len()
    }
}
