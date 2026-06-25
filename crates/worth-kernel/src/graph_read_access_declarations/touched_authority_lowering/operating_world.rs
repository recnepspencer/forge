use forge_query::facade::ForgeQueryGraphObligationOperatingWorldSelector;

use crate::graph_read_access_inventory::inventory_lane::{
    WorthGraphReadAccessScopeBinding, WorthGraphReadAccessScopeExpectation,
};

use super::lowering_errors::{
    WorthGraphReadTouchedAuthorityLoweringError, WorthGraphReadTouchedAuthorityLoweringErrorKind,
};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthGraphReadDeclarationOperatingWorld {
    selector_label: String,
    selector_digest: String,
}

impl WorthGraphReadDeclarationOperatingWorld {
    pub(crate) fn from_scope_binding(
        scope_binding: &WorthGraphReadAccessScopeBinding,
    ) -> Result<Self, WorthGraphReadTouchedAuthorityLoweringError> {
        let selector = match scope_binding.scope_expectation() {
            WorthGraphReadAccessScopeExpectation::MilestoneSevenDeclarationCandidateInput => {
                ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority()
            }
            WorthGraphReadAccessScopeExpectation::PreviewDeclarationCandidateInput => {
                ForgeQueryGraphObligationOperatingWorldSelector::preview()
            }
            WorthGraphReadAccessScopeExpectation::BranchDeclarationCandidateInput => {
                ForgeQueryGraphObligationOperatingWorldSelector::branch()
            }
            WorthGraphReadAccessScopeExpectation::QueryAccessRequirementCandidateInput
            | WorthGraphReadAccessScopeExpectation::FutureExecutionReceiptExpectation
            | WorthGraphReadAccessScopeExpectation::DeletionOnlyResidue
            | WorthGraphReadAccessScopeExpectation::CertificationOnlyBoundary
            | WorthGraphReadAccessScopeExpectation::NonGraphReadBoundary => {
                return Err(WorthGraphReadTouchedAuthorityLoweringError::new(
                    WorthGraphReadTouchedAuthorityLoweringErrorKind::UnsupportedOperatingWorldScope,
                ));
            }
        };
        Ok(Self {
            selector_label: selector.as_str().to_string(),
            selector_digest: selector
                .selector_digest()
                .terminal_projection_for_reporting()
                .to_string(),
        })
    }

    pub fn selector_label(&self) -> &str {
        &self.selector_label
    }

    pub fn selector_digest(&self) -> &str {
        &self.selector_digest
    }
}
