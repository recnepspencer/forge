use super::super::{
    S8AccessLoweringDeferred, S8ExecutionReadyAccessReceipt, S8LoweredAccessReceipt,
    S8RebindRequiredAccessReceipt, S8StaleLoweredAccessReceipt,
};

#[derive(Debug, PartialEq, Eq)]
enum S8IndexedExecutionReadinessPayload {
    Ready(S8ExecutionReadyAccessReceipt),
    Stale(S8StaleLoweredAccessReceipt),
    RebindRequired(S8RebindRequiredAccessReceipt),
    Deferred(S8AccessLoweringDeferred),
}

#[derive(Debug, PartialEq, Eq)]
pub struct S8IndexedExecutionReadinessOutcome {
    case: S8IndexedExecutionReadinessPayload,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8IndexedExecutionReadinessView<'a> {
    Ready(&'a S8ExecutionReadyAccessReceipt),
    Stale(&'a S8StaleLoweredAccessReceipt),
    RebindRequired(&'a S8RebindRequiredAccessReceipt),
    Deferred(&'a S8AccessLoweringDeferred),
}

impl S8IndexedExecutionReadinessOutcome {
    pub(crate) fn ready(value: S8ExecutionReadyAccessReceipt) -> Self {
        Self::from_owner_payload(S8IndexedExecutionReadinessPayload::Ready(value))
    }

    pub(crate) fn stale(value: S8StaleLoweredAccessReceipt) -> Self {
        Self::from_owner_payload(S8IndexedExecutionReadinessPayload::Stale(value))
    }

    pub(crate) fn rebind_required(value: S8RebindRequiredAccessReceipt) -> Self {
        Self::from_owner_payload(S8IndexedExecutionReadinessPayload::RebindRequired(value))
    }

    pub(crate) fn deferred(value: S8AccessLoweringDeferred) -> Self {
        Self::from_owner_payload(S8IndexedExecutionReadinessPayload::Deferred(value))
    }

    fn from_owner_payload(case: S8IndexedExecutionReadinessPayload) -> Self {
        Self { case }
    }

    pub fn view(&self) -> S8IndexedExecutionReadinessView<'_> {
        match &self.case {
            S8IndexedExecutionReadinessPayload::Ready(value) => {
                S8IndexedExecutionReadinessView::Ready(value)
            }
            S8IndexedExecutionReadinessPayload::Stale(value) => {
                S8IndexedExecutionReadinessView::Stale(value)
            }
            S8IndexedExecutionReadinessPayload::RebindRequired(value) => {
                S8IndexedExecutionReadinessView::RebindRequired(value)
            }
            S8IndexedExecutionReadinessPayload::Deferred(value) => {
                S8IndexedExecutionReadinessView::Deferred(value)
            }
        }
    }

    fn into_owner_payload(self) -> S8IndexedExecutionReadinessPayload {
        self.case
    }
}

#[derive(Debug, PartialEq, Eq)]
enum S8DegradedExecutionReadinessPayload {
    DegradedReady(S8ExecutionReadyAccessReceipt),
}

#[derive(Debug, PartialEq, Eq)]
pub struct S8DegradedExecutionReadinessOutcome {
    case: S8DegradedExecutionReadinessPayload,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8DegradedExecutionReadinessView<'a> {
    DegradedReady(&'a S8ExecutionReadyAccessReceipt),
}

impl S8DegradedExecutionReadinessOutcome {
    pub(crate) fn ready(value: S8ExecutionReadyAccessReceipt) -> Self {
        Self::from_owner_payload(S8DegradedExecutionReadinessPayload::DegradedReady(value))
    }

    fn from_owner_payload(case: S8DegradedExecutionReadinessPayload) -> Self {
        Self { case }
    }

    pub fn view(&self) -> S8DegradedExecutionReadinessView<'_> {
        match &self.case {
            S8DegradedExecutionReadinessPayload::DegradedReady(value) => {
                S8DegradedExecutionReadinessView::DegradedReady(value)
            }
        }
    }

    fn into_owner_payload(self) -> S8DegradedExecutionReadinessPayload {
        self.case
    }
}

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
}

impl From<S8LoweredAccessReceipt> for S8ExecutionReadinessOutcome {
    fn from(lowered: S8LoweredAccessReceipt) -> Self {
        Self::ready(S8ExecutionReadyAccessReceipt::admit(lowered))
    }
}
