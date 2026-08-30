//! Defensive installation denial for violated declaration cardinality.

use crate::application_operation::WorthQueryApplicationOperationInstallationDenialKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WorthQueryOperationContractCardinalityDenial {
    #[cfg(test)]
    AmbiguousExternalEffect,
    AmbiguousAftermath,
}

impl WorthQueryOperationContractCardinalityDenial {
    pub(crate) const fn installation_kind(
        self,
    ) -> WorthQueryApplicationOperationInstallationDenialKind {
        match self {
            #[cfg(test)]
            Self::AmbiguousExternalEffect => {
                WorthQueryApplicationOperationInstallationDenialKind::AmbiguousExternalEffectContract
            }
            Self::AmbiguousAftermath => {
                WorthQueryApplicationOperationInstallationDenialKind::AmbiguousAftermathContract
            }
        }
    }
}
