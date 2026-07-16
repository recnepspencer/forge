use std::sync::Arc;

use worth_query::facade::foundation::{
    ProjectionConsumptionWarnings, WorthQueryConsumedProjectionAuthority,
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

    pub fn binds_prerequisites(
        &self,
        prerequisites: &super::WorthUiQueryPrerequisiteEvidence,
    ) -> bool {
        self.0.binds_resolved_basis(prerequisites.basis())
    }

    pub fn bind_prerequisites(
        &self,
        prerequisites: super::WorthUiQueryPrerequisiteEvidence,
    ) -> Result<
        super::WorthUiQueryPrerequisiteEvidence,
        super::WorthUiQueryMeasurementFactEligibilityError,
    > {
        super::WorthUiQueryMeasurementFactEligibility::bind_query_authority(
            prerequisites,
            self.authority(),
        )
    }

    pub fn basis_digest_for_diagnostics(&self) -> &str {
        self.0.contract().basis_digest().unwrap_or_default()
    }

    pub fn projection_contract_digest_for_diagnostics(&self) -> &str {
        self.0.contract().contract_digest()
    }

    pub fn projection_consumption_receipt_digest_for_diagnostics(&self) -> &str {
        self.0.receipt().receipt_digest()
    }

    pub fn projection_consumption_declaration_digest_for_diagnostics(&self) -> &str {
        self.0.receipt().declaration_digest()
    }

    pub fn projection_fact_set_digest_for_diagnostics(&self) -> &str {
        self.0.receipt().fact_set_digest()
    }

    pub fn projection_source_identity_for_diagnostics(&self) -> &str {
        self.0.source_identity().as_str()
    }
}

impl PartialEq for WorthUiQueryAuthorityHandle {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0) || self.0.structurally_equivalent(&other.0)
    }
}

impl Eq for WorthUiQueryAuthorityHandle {}
