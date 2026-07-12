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

    /// Moves the sealed authority into a downstream owner without exposing its
    /// independently pairable construction inputs.
    pub fn into_admitted(
        self,
    ) -> Result<
        (
            WorthQueryConsumedProjectionAuthority,
            Option<ProjectionConsumptionWarnings>,
        ),
        Self,
    > {
        match self {
            Self::Admitted(authority) => Ok((authority, None)),
            Self::AdmittedWithWarnings(authority, warnings) => Ok((authority, Some(warnings))),
            denied => Err(denied),
        }
    }
}
