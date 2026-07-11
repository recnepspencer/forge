use super::super::{
    S8AccessLoweringDeferred, S8AccessLoweringDenied, S8ExecutionReadyAccessReceipt,
    S8LoweredAccessReceipt, S8RebindRequiredAccessReceipt, S8StaleLoweredAccessReceipt,
};
use crate::production_transition::define_owner_outcome;

define_owner_outcome!(
    pub S8StaleReadmissionOutcome,
    pub S8StaleReadmissionView,
    S8StaleReadmissionPayload,
    StaleRebindReadmission,
    RebindAndReadmitStaleAccess,
    [
        required => ReadmissionRequired(S8RebindRequiredAccessReceipt): Lowered => RequireRebind => RebindRequired,
        rebound => Rebound(S8LoweredAccessReceipt): RebindRequired => Rebind => Lowered,
        readmitted => Readmitted(S8ExecutionReadyAccessReceipt): Stale => Readmit => Readmitted,
        still_stale => StillStale(S8StaleLoweredAccessReceipt): Stale => Readmit => Stale,
        deferred => Deferred(S8AccessLoweringDeferred): Stale => Defer => Deferred,
        denied => Denied(S8AccessLoweringDenied): Stale => Deny => Denied,
    ]
);

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
    pub(crate) fn contract() -> crate::production_transition::S8OwnerTransitionContract {
        Self::owner_transition_contract()
    }
}
