use worth_signal::facade::{AsyncNodeAdmissionClass, AsyncNodeConditionBlockClass};
use worth_store_physical_backend::ArtifactTreeFailure;

use crate::physical_runtime::{
    PhysicalExecutorCommandDenial, PhysicalSchedulerDenial, PhysicalWorkPreEffectDenial,
    WalAppendedPhysicalMutation, WalDurablePhysicalMutation,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWalBarrierFailureCause {
    RuntimeReleased,
    PolicyOrRuntimeMismatch,
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

pub struct WalBarrierIndeterminatePhysicalMutation {
    appended: WalAppendedPhysicalMutation,
}

pub enum PhysicalWalBarrierOutcome {
    Durable(WalDurablePhysicalMutation),
    BarrierNotStarted {
        appended: WalAppendedPhysicalMutation,
        cause: PhysicalWalBarrierFailureCause,
    },
    Indeterminate(WalBarrierIndeterminatePhysicalMutation),
}

impl WalBarrierIndeterminatePhysicalMutation {
    pub(in crate::physical_runtime) const fn new(appended: WalAppendedPhysicalMutation) -> Self {
        Self { appended }
    }

    pub const fn mutation_identity(&self) -> crate::physical_runtime::PhysicalMutationIdentity {
        self.appended.mutation_identity()
    }

    pub const fn member_basis(&self) -> crate::physical_runtime::PhysicalWalMemberBasis {
        self.appended.reserved().member_basis()
    }
}
