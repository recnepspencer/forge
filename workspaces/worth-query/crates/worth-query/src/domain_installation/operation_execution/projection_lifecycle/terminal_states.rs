use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::WorthQuerySettledDomainProjection;
use crate::runtime::WorthQueryWorkspace;
use worth_proof::PhaseMarker;

use super::lifecycle_close::{
    close_operational_projection, WorthQueryClosedProjection, WorthQueryProjectionCloseCoreOutcome,
    WorthQueryProjectionCloseCoreStopKind,
};
use super::{
    WorthQueryLiveBoundDomainProjection, WorthQueryProjectionLifecycleCloseCause,
    WorthQueryProjectionLifecycleCloseReceipt, WorthQueryProjectionLifecycleTransitionCounters,
};

pub(super) struct WorthQueryCancelledProjectionPhase;
pub(super) struct WorthQueryDisposedProjectionPhase;

impl PhaseMarker for WorthQueryCancelledProjectionPhase {}
impl PhaseMarker for WorthQueryDisposedProjectionPhase {}

pub struct WorthQueryCancelledDomainProjection<D, O, F, L: BasisOperationLane> {
    pub(super) closed: WorthQueryClosedProjection<
        WorthQuerySettledDomainProjection<D, O, F, L>,
        WorthQueryCancelledProjectionPhase,
    >,
    pub(super) prior_transition: Option<WorthQueryProjectionPriorTransitionEvidence>,
}

pub struct WorthQueryDisposedDomainProjection<D, O, F, L: BasisOperationLane> {
    pub(super) closed: WorthQueryClosedProjection<
        WorthQuerySettledDomainProjection<D, O, F, L>,
        WorthQueryDisposedProjectionPhase,
    >,
    pub(super) prior_transition: Option<WorthQueryProjectionPriorTransitionEvidence>,
}

pub enum WorthQueryProjectionPriorTransitionEvidence {
    Replacement {
        witness: crate::domain_installation::WorthQueryReplacementWitness,
        predecessor_close: WorthQueryProjectionLifecycleCloseReceipt,
        work: super::WorthQueryProjectionTransitionWork,
        cleanup_work: super::WorthQueryProjectionCleanupWork,
    },
    Rebind {
        witness: crate::domain_installation::WorthQueryRebindWitness,
        predecessor_close: WorthQueryProjectionLifecycleCloseReceipt,
        work: super::WorthQueryProjectionTransitionWork,
        cleanup_work: super::WorthQueryProjectionCleanupWork,
    },
}

#[must_use = "cancellation stops retain the operational projection for retry"]
pub enum WorthQueryProjectionCancellationOutcome<D, O, F, L: BasisOperationLane> {
    Cancelled(WorthQueryCancelledDomainProjection<D, O, F, L>),
    Stopped(WorthQueryProjectionCancellationStop<D, O, F, L>),
}

pub struct WorthQueryProjectionCancellationStop<D, O, F, L: BasisOperationLane> {
    live: WorthQueryLiveBoundDomainProjection<D, O, F, L>,
    detail: String,
    counters: WorthQueryProjectionLifecycleTransitionCounters,
}

#[must_use = "disposal stops retain the operational projection for retry"]
pub enum WorthQueryProjectionDisposalOutcome<D, O, F, L: BasisOperationLane> {
    Disposed(WorthQueryDisposedDomainProjection<D, O, F, L>),
    Stopped(WorthQueryProjectionDisposalStop<D, O, F, L>),
}

pub struct WorthQueryProjectionDisposalStop<D, O, F, L: BasisOperationLane> {
    live: WorthQueryLiveBoundDomainProjection<D, O, F, L>,
    detail: String,
    counters: WorthQueryProjectionLifecycleTransitionCounters,
}

impl<D: 'static, O, F, L: BasisOperationLane> WorthQueryLiveBoundDomainProjection<D, O, F, L> {
    pub fn cancel(
        self,
        workspace: &mut WorthQueryWorkspace,
    ) -> WorthQueryProjectionCancellationOutcome<D, O, F, L> {
        match close_operational_projection::<_, L, _, WorthQueryCancelledProjectionPhase>(
            self.into_owner(),
            WorthQueryProjectionLifecycleCloseCause::Cancellation,
            workspace,
        ) {
            WorthQueryProjectionCloseCoreOutcome::Closed(closed) => {
                WorthQueryProjectionCancellationOutcome::Cancelled(
                    WorthQueryCancelledDomainProjection {
                        closed,
                        prior_transition: None,
                    },
                )
            }
            WorthQueryProjectionCloseCoreOutcome::Stopped(stop) => {
                let (owner, kind, counters) = stop.into_parts();
                WorthQueryProjectionCancellationOutcome::Stopped(
                    WorthQueryProjectionCancellationStop {
                        live: WorthQueryLiveBoundDomainProjection::from_owner(owner),
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
    ) -> WorthQueryProjectionDisposalOutcome<D, O, F, L> {
        match close_operational_projection::<_, L, _, WorthQueryDisposedProjectionPhase>(
            self.into_owner(),
            WorthQueryProjectionLifecycleCloseCause::Disposal,
            workspace,
        ) {
            WorthQueryProjectionCloseCoreOutcome::Closed(closed) => {
                WorthQueryProjectionDisposalOutcome::Disposed(WorthQueryDisposedDomainProjection {
                    closed,
                    prior_transition: None,
                })
            }
            WorthQueryProjectionCloseCoreOutcome::Stopped(stop) => {
                let (owner, kind, counters) = stop.into_parts();
                WorthQueryProjectionDisposalOutcome::Stopped(WorthQueryProjectionDisposalStop {
                    live: WorthQueryLiveBoundDomainProjection::from_owner(owner),
                    detail: close_detail(kind),
                    counters,
                })
            }
        }
    }
}

impl<D, O, F, L: BasisOperationLane> WorthQueryCancelledDomainProjection<D, O, F, L> {
    pub fn identity(&self) -> &str {
        &self.closed.proof().payload().identity
    }

    pub fn snapshot(&self) -> &WorthQuerySettledDomainProjection<D, O, F, L> {
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

    pub fn dispose(self) -> WorthQueryDisposedDomainProjection<D, O, F, L> {
        WorthQueryDisposedDomainProjection {
            closed: self
                .closed
                .transition("worth_query_disposed_cancelled_projection_v1"),
            prior_transition: self.prior_transition,
        }
    }
}

impl<D, O, F, L: BasisOperationLane> WorthQueryDisposedDomainProjection<D, O, F, L> {
    pub fn identity(&self) -> &str {
        &self.closed.proof().payload().identity
    }

    pub fn snapshot(&self) -> &WorthQuerySettledDomainProjection<D, O, F, L> {
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

macro_rules! close_stop_inspection {
    ($name:ident) => {
        impl<D, O, F, L: BasisOperationLane> $name<D, O, F, L> {
            pub fn detail(&self) -> &str {
                &self.detail
            }

            pub fn counters(&self) -> WorthQueryProjectionLifecycleTransitionCounters {
                self.counters
            }

            pub fn into_live(self) -> WorthQueryLiveBoundDomainProjection<D, O, F, L> {
                self.live
            }
        }
    };
}

close_stop_inspection!(WorthQueryProjectionCancellationStop);
close_stop_inspection!(WorthQueryProjectionDisposalStop);

fn close_detail(kind: WorthQueryProjectionCloseCoreStopKind) -> String {
    match kind {
        WorthQueryProjectionCloseCoreStopKind::Runtime(detail) => detail,
    }
}
