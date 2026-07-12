use super::{ConsumedProjectionAuthorityDenial, WorthQueryConsumedProjectionAuthority};
use crate::projection_consumption::{
    DeferredProjectionConsumption, DeniedProjectionConsumption, ProjectionConsumptionWarnings,
    SourceMismatchedProjectionConsumption,
};

#[derive(Debug)]
pub enum ProjectionAuthorityOutcome {
    Admitted(WorthQueryConsumedProjectionAuthority),
    AdmittedWithWarnings(
        WorthQueryConsumedProjectionAuthority,
        ProjectionConsumptionWarnings,
    ),
    AuthorityDenied(ConsumedProjectionAuthorityDenial),
    ConsumptionDenied(DeniedProjectionConsumption),
    Deferred(DeferredProjectionConsumption),
    SourceMismatch(SourceMismatchedProjectionConsumption),
}

impl ProjectionAuthorityOutcome {
    pub fn authority(&self) -> Option<&WorthQueryConsumedProjectionAuthority> {
        match self {
            Self::Admitted(authority) | Self::AdmittedWithWarnings(authority, _) => Some(authority),
            Self::AuthorityDenied(_)
            | Self::ConsumptionDenied(_)
            | Self::Deferred(_)
            | Self::SourceMismatch(_) => None,
        }
    }
}
