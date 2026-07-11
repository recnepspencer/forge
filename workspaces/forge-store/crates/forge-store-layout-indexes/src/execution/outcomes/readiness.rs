use super::super::{
    S8AccessLoweringDeferred, S8ExecutionReadyAccessReceipt, S8LoweredAccessReceipt,
    S8RebindRequiredAccessReceipt, S8StaleLoweredAccessReceipt,
};
use crate::production_transition::define_owner_outcome;

define_owner_outcome!(
    pub S8IndexedExecutionReadinessOutcome,
    pub S8IndexedExecutionReadinessView,
    S8IndexedExecutionReadinessPayload,
    ExecutionReadiness,
    AdmitExecutionReadiness,
    [
        ready => Ready(S8ExecutionReadyAccessReceipt): Lowered => Ready => Ready,
        stale => Stale(S8StaleLoweredAccessReceipt): Lowered => Ready => Stale,
        rebind_required => RebindRequired(S8RebindRequiredAccessReceipt): Lowered => RequireRebind => RebindRequired,
        deferred => Deferred(S8AccessLoweringDeferred): Lowered => Defer => Deferred,
    ]
);

define_owner_outcome!(
    pub S8DegradedExecutionReadinessOutcome,
    pub S8DegradedExecutionReadinessView,
    S8DegradedExecutionReadinessPayload,
    DegradedExactScan,
    ExecuteBudgetedDegradedExactScan,
    [ready => DegradedReady(S8ExecutionReadyAccessReceipt): Lowered => Ready => Ready]
);

#[derive(Debug, PartialEq, Eq)]
enum ReadinessOwnerOutcome {
    Indexed(S8IndexedExecutionReadinessOutcome),
    Degraded(S8DegradedExecutionReadinessOutcome),
}

#[derive(Debug, PartialEq, Eq)]
pub struct S8ExecutionReadinessOutcome {
    owner: ReadinessOwnerOutcome,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8ExecutionReadinessView<'a> {
    Ready(&'a S8ExecutionReadyAccessReceipt),
    Stale(&'a S8StaleLoweredAccessReceipt),
    RebindRequired(&'a S8RebindRequiredAccessReceipt),
    Deferred(&'a S8AccessLoweringDeferred),
}

impl S8ExecutionReadinessOutcome {
    pub(crate) fn ready(ready: S8ExecutionReadyAccessReceipt) -> Self {
        let owner = if ready.path_kind().is_degraded_exact_scan() {
            ReadinessOwnerOutcome::Degraded(S8DegradedExecutionReadinessOutcome::ready(ready))
        } else {
            ReadinessOwnerOutcome::Indexed(S8IndexedExecutionReadinessOutcome::ready(ready))
        };
        Self { owner }
    }
    pub(crate) fn stale(value: S8StaleLoweredAccessReceipt) -> Self {
        Self {
            owner: ReadinessOwnerOutcome::Indexed(S8IndexedExecutionReadinessOutcome::stale(value)),
        }
    }
    pub(crate) fn rebind_required(value: S8RebindRequiredAccessReceipt) -> Self {
        Self {
            owner: ReadinessOwnerOutcome::Indexed(
                S8IndexedExecutionReadinessOutcome::rebind_required(value),
            ),
        }
    }
    pub(crate) fn deferred(value: S8AccessLoweringDeferred) -> Self {
        Self {
            owner: ReadinessOwnerOutcome::Indexed(S8IndexedExecutionReadinessOutcome::deferred(
                value,
            )),
        }
    }
    pub fn view(&self) -> S8ExecutionReadinessView<'_> {
        match &self.owner {
            ReadinessOwnerOutcome::Degraded(value) => match value.view() {
                S8DegradedExecutionReadinessView::DegradedReady(ready) => {
                    S8ExecutionReadinessView::Ready(ready)
                }
            },
            ReadinessOwnerOutcome::Indexed(value) => match value.view() {
                S8IndexedExecutionReadinessView::Ready(ready) => {
                    S8ExecutionReadinessView::Ready(ready)
                }
                S8IndexedExecutionReadinessView::Stale(stale) => {
                    S8ExecutionReadinessView::Stale(stale)
                }
                S8IndexedExecutionReadinessView::RebindRequired(rebind) => {
                    S8ExecutionReadinessView::RebindRequired(rebind)
                }
                S8IndexedExecutionReadinessView::Deferred(reason) => {
                    S8ExecutionReadinessView::Deferred(reason)
                }
            },
        }
    }
    pub fn into_ready(self) -> Result<S8ExecutionReadyAccessReceipt, Self> {
        match self.owner {
            ReadinessOwnerOutcome::Degraded(value) => match value.into_owner_payload() {
                S8DegradedExecutionReadinessPayload::DegradedReady(ready) => Ok(ready),
            },
            ReadinessOwnerOutcome::Indexed(value) => match value.into_owner_payload() {
                S8IndexedExecutionReadinessPayload::Ready(ready) => Ok(ready),
                payload => Err(Self {
                    owner: ReadinessOwnerOutcome::Indexed(
                        S8IndexedExecutionReadinessOutcome::from_owner_payload(payload),
                    ),
                }),
            },
        }
    }
    pub fn into_stale(self) -> Result<S8StaleLoweredAccessReceipt, Self> {
        match self.owner {
            ReadinessOwnerOutcome::Indexed(value) => match value.into_owner_payload() {
                S8IndexedExecutionReadinessPayload::Stale(stale) => Ok(stale),
                payload => Err(Self {
                    owner: ReadinessOwnerOutcome::Indexed(
                        S8IndexedExecutionReadinessOutcome::from_owner_payload(payload),
                    ),
                }),
            },
            owner => Err(Self { owner }),
        }
    }
    pub fn into_rebind_required(self) -> Result<S8RebindRequiredAccessReceipt, Self> {
        match self.owner {
            ReadinessOwnerOutcome::Indexed(value) => match value.into_owner_payload() {
                S8IndexedExecutionReadinessPayload::RebindRequired(rebind) => Ok(rebind),
                payload => Err(Self {
                    owner: ReadinessOwnerOutcome::Indexed(
                        S8IndexedExecutionReadinessOutcome::from_owner_payload(payload),
                    ),
                }),
            },
            owner => Err(Self { owner }),
        }
    }
    pub fn into_deferred(self) -> Result<S8AccessLoweringDeferred, Self> {
        match self.owner {
            ReadinessOwnerOutcome::Indexed(value) => match value.into_owner_payload() {
                S8IndexedExecutionReadinessPayload::Deferred(deferred) => Ok(deferred),
                payload => Err(Self {
                    owner: ReadinessOwnerOutcome::Indexed(
                        S8IndexedExecutionReadinessOutcome::from_owner_payload(payload),
                    ),
                }),
            },
            owner => Err(Self { owner }),
        }
    }
    pub const fn production_transition(
        &self,
    ) -> crate::production_transition::S8LayoutProductionTransition {
        match &self.owner {
            ReadinessOwnerOutcome::Indexed(value) => value.production_transition(),
            ReadinessOwnerOutcome::Degraded(value) => value.production_transition(),
        }
    }
    pub(crate) fn indexed_contract() -> crate::production_transition::S8OwnerTransitionContract {
        S8IndexedExecutionReadinessOutcome::owner_transition_contract()
    }
    pub(crate) fn degraded_contract() -> crate::production_transition::S8OwnerTransitionContract {
        S8DegradedExecutionReadinessOutcome::owner_transition_contract()
    }
}

impl From<S8LoweredAccessReceipt> for S8ExecutionReadinessOutcome {
    fn from(lowered: S8LoweredAccessReceipt) -> Self {
        Self::ready(S8ExecutionReadyAccessReceipt::admit(lowered))
    }
}
