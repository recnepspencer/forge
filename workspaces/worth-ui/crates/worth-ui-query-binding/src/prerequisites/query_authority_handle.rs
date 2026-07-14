use std::sync::Arc;

use worth_query::facade::foundation::{
    ProjectionConsumptionWarnings,
    WorthQueryConsumedProjectionAuthority,
};
use worth_query::facade::read::WorthQueryProjectionOutcome;

/// Opaque shared retention of the exact Query-minted authority.
///
/// Cloning this handle fans out observation of one authority allocation; it
/// does not clone or reconstruct the authority token.
///
/// ```compile_fail
/// fn consumer_cannot_wrap_authority_directly(
///     authority: worth_query::facade::foundation::WorthQueryConsumedProjectionAuthority,
/// ) -> worth_ui_query_binding::WorthUiQueryAuthorityHandle {
///     worth_ui_query_binding::WorthUiQueryAuthorityHandle(std::sync::Arc::new(authority))
/// }
/// ```
#[derive(Clone, Debug)]
pub struct WorthUiQueryAuthorityHandle(Arc<WorthQueryConsumedProjectionAuthority>);

impl WorthUiQueryAuthorityHandle {
    pub(crate) fn retain(authority: WorthQueryConsumedProjectionAuthority) -> Self {
        Self(Arc::new(authority))
    }

    pub fn from_outcome(
        outcome: WorthQueryProjectionOutcome,
    ) -> Result<(Self, Option<ProjectionConsumptionWarnings>), WorthQueryProjectionOutcome> {
        outcome
            .into_admitted()
            .map(|(authority, warnings)| (Self::retain(*authority), warnings))
    }

    pub fn authority(&self) -> &WorthQueryConsumedProjectionAuthority {
        &self.0
    }

    pub fn shares_authority_with(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl PartialEq for WorthUiQueryAuthorityHandle {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0) || self.0.structurally_equivalent(&other.0)
    }
}

impl Eq for WorthUiQueryAuthorityHandle {}
