use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::WorthQuerySettledWorkflowProjection;
use crate::runtime::WorthQueryWorkspace;

use super::lifecycle_close::{
    close_operational_projection, WorthQueryClosedProjection, WorthQueryProjectionCloseCoreOutcome,
    WorthQueryProjectionCloseCoreStopKind,
};
use super::terminal_states::{
    WorthQueryCancelledProjectionPhase, WorthQueryDisposedProjectionPhase,
};
use super::{
    WorthQueryLiveBoundWorkflowProjection, WorthQueryProjectionLifecycleCloseCause,
    WorthQueryProjectionLifecycleCloseReceipt, WorthQueryProjectionLifecycleTransitionCounters,
    WorthQueryProjectionPriorTransitionEvidence,
};

pub struct WorthQueryCancelledWorkflowProjection<D, O, F, L: BasisOperationLane> {
    pub(super) closed: WorthQueryClosedProjection<
        WorthQuerySettledWorkflowProjection<D, O, F, L>,
        WorthQueryCancelledProjectionPhase,
    >,
    pub(super) prior_transition: Option<WorthQueryProjectionPriorTransitionEvidence>,
}

pub struct WorthQueryDisposedWorkflowProjection<D, O, F, L: BasisOperationLane> {
    pub(super) closed: WorthQueryClosedProjection<
        WorthQuerySettledWorkflowProjection<D, O, F, L>,
        WorthQueryDisposedProjectionPhase,
    >,
    pub(super) prior_transition: Option<WorthQueryProjectionPriorTransitionEvidence>,
}

#[must_use = "workflow cancellation stops retain the operational projection for retry"]
pub enum WorthQueryWorkflowProjectionCancellationOutcome<D, O, F, L: BasisOperationLane> {
    Cancelled(WorthQueryCancelledWorkflowProjection<D, O, F, L>),
    Stopped(WorthQueryWorkflowProjectionCancellationStop<D, O, F, L>),
}

pub struct WorthQueryWorkflowProjectionCancellationStop<D, O, F, L: BasisOperationLane> {
    live: WorthQueryLiveBoundWorkflowProjection<D, O, F, L>,
    detail: String,
    counters: WorthQueryProjectionLifecycleTransitionCounters,
}

#[must_use = "workflow disposal stops retain the operational projection for retry"]
pub enum WorthQueryWorkflowProjectionDisposalOutcome<D, O, F, L: BasisOperationLane> {
    Disposed(WorthQueryDisposedWorkflowProjection<D, O, F, L>),
    Stopped(WorthQueryWorkflowProjectionDisposalStop<D, O, F, L>),
}

pub struct WorthQueryWorkflowProjectionDisposalStop<D, O, F, L: BasisOperationLane> {
    live: WorthQueryLiveBoundWorkflowProjection<D, O, F, L>,
    detail: String,
    counters: WorthQueryProjectionLifecycleTransitionCounters,
}

impl<D: 'static, O, F, L: BasisOperationLane> WorthQueryLiveBoundWorkflowProjection<D, O, F, L> {
    pub fn cancel(
        self,
        workspace: &mut WorthQueryWorkspace,
    ) -> WorthQueryWorkflowProjectionCancellationOutcome<D, O, F, L> {
        match close_operational_projection::<_, L, _, WorthQueryCancelledProjectionPhase>(
            self.into_owner(),
            WorthQueryProjectionLifecycleCloseCause::Cancellation,
            workspace,
        ) {
            WorthQueryProjectionCloseCoreOutcome::Closed(closed) => {
                WorthQueryWorkflowProjectionCancellationOutcome::Cancelled(
                    WorthQueryCancelledWorkflowProjection {
                        closed,
                        prior_transition: None,
                    },
                )
            }
            WorthQueryProjectionCloseCoreOutcome::Stopped(stop) => {
                let (owner, kind, counters) = stop.into_parts();
                WorthQueryWorkflowProjectionCancellationOutcome::Stopped(
                    WorthQueryWorkflowProjectionCancellationStop {
                        live: WorthQueryLiveBoundWorkflowProjection::from_owner(owner),
                        detail: close_detail(kind),
                        counters,
                    },
                )
            }
        }
    }

    pub fn dispose(
        self,
        workspace: &mut WorthQueryWorkspace,
    ) -> WorthQueryWorkflowProjectionDisposalOutcome<D, O, F, L> {
        match close_operational_projection::<_, L, _, WorthQueryDisposedProjectionPhase>(
            self.into_owner(),
            WorthQueryProjectionLifecycleCloseCause::Disposal,
            workspace,
        ) {
            WorthQueryProjectionCloseCoreOutcome::Closed(closed) => {
                WorthQueryWorkflowProjectionDisposalOutcome::Disposed(
                    WorthQueryDisposedWorkflowProjection {
                        closed,
                        prior_transition: None,
                    },
                )
            }
            WorthQueryProjectionCloseCoreOutcome::Stopped(stop) => {
                let (owner, kind, counters) = stop.into_parts();
                WorthQueryWorkflowProjectionDisposalOutcome::Stopped(
                    WorthQueryWorkflowProjectionDisposalStop {
                        live: WorthQueryLiveBoundWorkflowProjection::from_owner(owner),
                        detail: close_detail(kind),
                        counters,
                    },
                )
            }
        }
    }
}

macro_rules! closed_inspection {
    ($name:ident) => {
        impl<D, O, F, L: BasisOperationLane> $name<D, O, F, L> {
            pub fn identity(&self) -> &str {
                &self.closed.proof().payload().identity
            }

            pub fn snapshot(&self) -> &WorthQuerySettledWorkflowProjection<D, O, F, L> {
                self.closed.source()
            }

            pub fn close_receipt(&self) -> &WorthQueryProjectionLifecycleCloseReceipt {
                self.closed.close_receipt()
            }

            pub fn live_receipt(&self) -> &super::WorthQueryLiveProjectionReceipt {
                self.closed.live_receipt()
            }

            pub fn conditional_provenance(
                &self,
            ) -> &[crate::domain_installation::WorthQueryConditionalProvenance] {
                self.closed.conditional_provenance()
            }

            pub fn prior_transition(&self) -> Option<&WorthQueryProjectionPriorTransitionEvidence> {
                self.prior_transition.as_ref()
            }
        }
    };
}

closed_inspection!(WorthQueryCancelledWorkflowProjection);
closed_inspection!(WorthQueryDisposedWorkflowProjection);

impl<D, O, F, L: BasisOperationLane> WorthQueryCancelledWorkflowProjection<D, O, F, L> {
    pub fn dispose(self) -> WorthQueryDisposedWorkflowProjection<D, O, F, L> {
        WorthQueryDisposedWorkflowProjection {
            closed: self
                .closed
                .transition("worth_query_disposed_cancelled_workflow_projection_v1"),
            prior_transition: self.prior_transition,
        }
    }
}

macro_rules! close_stop_inspection {
    ($name:ident) => {
        impl<D, O, F, L: BasisOperationLane> $name<D, O, F, L> {
            pub fn detail(&self) -> &str {
                &self.detail
            }

            pub fn counters(&self) -> WorthQueryProjectionLifecycleTransitionCounters {
                self.counters
            }

            pub fn into_live(self) -> WorthQueryLiveBoundWorkflowProjection<D, O, F, L> {
                self.live
            }
        }
    };
}

close_stop_inspection!(WorthQueryWorkflowProjectionCancellationStop);
close_stop_inspection!(WorthQueryWorkflowProjectionDisposalStop);

fn close_detail(kind: WorthQueryProjectionCloseCoreStopKind) -> String {
    match kind {
        WorthQueryProjectionCloseCoreStopKind::Runtime(detail) => detail,
    }
}
