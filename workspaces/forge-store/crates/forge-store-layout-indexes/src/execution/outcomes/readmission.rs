use super::super::{
    S8AccessLoweringDeferred, S8AccessLoweringDenied, S8ExecutionReadyAccessReceipt,
    S8LoweredAccessReceipt, S8RebindRequiredAccessReceipt, S8StaleLoweredAccessReceipt,
};

#[derive(Debug, PartialEq, Eq)]
enum S8StaleReadmissionPayload {
    ReadmissionRequired(S8RebindRequiredAccessReceipt),
    Rebound(S8LoweredAccessReceipt),
    Readmitted(S8ExecutionReadyAccessReceipt),
    StillStale(S8StaleLoweredAccessReceipt),
    Deferred(S8AccessLoweringDeferred),
    Denied(S8AccessLoweringDenied),
}

#[derive(Debug, PartialEq, Eq)]
pub struct S8StaleReadmissionOutcome {
    case: S8StaleReadmissionPayload,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8StaleReadmissionView<'a> {
    ReadmissionRequired(&'a S8RebindRequiredAccessReceipt),
    Rebound(&'a S8LoweredAccessReceipt),
    Readmitted(&'a S8ExecutionReadyAccessReceipt),
    StillStale(&'a S8StaleLoweredAccessReceipt),
    Deferred(&'a S8AccessLoweringDeferred),
    Denied(&'a S8AccessLoweringDenied),
}

impl S8StaleReadmissionOutcome {
    pub(crate) fn required(value: S8RebindRequiredAccessReceipt) -> Self {
        Self::from_owner_payload(S8StaleReadmissionPayload::ReadmissionRequired(value))
    }

    pub(crate) fn rebound(value: S8LoweredAccessReceipt) -> Self {
        Self::from_owner_payload(S8StaleReadmissionPayload::Rebound(value))
    }

    pub(crate) fn readmitted(value: S8ExecutionReadyAccessReceipt) -> Self {
        Self::from_owner_payload(S8StaleReadmissionPayload::Readmitted(value))
    }

    pub(crate) fn still_stale(value: S8StaleLoweredAccessReceipt) -> Self {
        Self::from_owner_payload(S8StaleReadmissionPayload::StillStale(value))
    }

    pub(crate) fn deferred(value: S8AccessLoweringDeferred) -> Self {
        Self::from_owner_payload(S8StaleReadmissionPayload::Deferred(value))
    }

    pub(crate) fn denied(value: S8AccessLoweringDenied) -> Self {
        Self::from_owner_payload(S8StaleReadmissionPayload::Denied(value))
    }

    fn from_owner_payload(case: S8StaleReadmissionPayload) -> Self {
        Self { case }
    }

    pub fn view(&self) -> S8StaleReadmissionView<'_> {
        match &self.case {
            S8StaleReadmissionPayload::ReadmissionRequired(value) => {
                S8StaleReadmissionView::ReadmissionRequired(value)
            }
            S8StaleReadmissionPayload::Rebound(value) => S8StaleReadmissionView::Rebound(value),
            S8StaleReadmissionPayload::Readmitted(value) => {
                S8StaleReadmissionView::Readmitted(value)
            }
            S8StaleReadmissionPayload::StillStale(value) => {
                S8StaleReadmissionView::StillStale(value)
            }
            S8StaleReadmissionPayload::Deferred(value) => S8StaleReadmissionView::Deferred(value),
            S8StaleReadmissionPayload::Denied(value) => S8StaleReadmissionView::Denied(value),
        }
    }

    fn into_owner_payload(self) -> S8StaleReadmissionPayload {
        self.case
    }
}

impl S8StaleReadmissionOutcome {
    pub fn into_readmitted(self) -> Result<S8ExecutionReadyAccessReceipt, Self> {
        match self.into_owner_payload() {
            S8StaleReadmissionPayload::Readmitted(ready) => Ok(ready),
            payload => Err(Self::from_owner_payload(payload)),
        }
    }
    pub fn into_required(self) -> Result<S8RebindRequiredAccessReceipt, Self> {
        match self.into_owner_payload() {
            S8StaleReadmissionPayload::ReadmissionRequired(value) => Ok(value),
            payload => Err(Self::from_owner_payload(payload)),
        }
    }
    pub fn into_rebound(self) -> Result<S8LoweredAccessReceipt, Self> {
        match self.into_owner_payload() {
            S8StaleReadmissionPayload::Rebound(value) => Ok(value),
            payload => Err(Self::from_owner_payload(payload)),
        }
    }
    pub fn into_denial(self) -> Result<S8AccessLoweringDenied, Self> {
        match self.into_owner_payload() {
            S8StaleReadmissionPayload::Denied(value) => Ok(value),
            payload => Err(Self::from_owner_payload(payload)),
        }
    }
}
