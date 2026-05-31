use serde::{Deserialize, Serialize};

use super::{
    TopologyDerivedRegion, TopologyEditChangedScope, TopologyEditContract, TopologyEditFamily,
    TopologyEditNamingScope, TopologyOperatorExecutionError,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TopologyEditRejectionClass {
    OutOfClassEdit,
    InvariantBlocked,
    NamingContinuityAmbiguous,
    NamingContinuityRejected,
    ScopeLocalizationUnavailable,
    DerivedFallbackExceeded,
}

impl TopologyEditRejectionClass {
    pub const ALL: [Self; 6] = [
        Self::OutOfClassEdit,
        Self::InvariantBlocked,
        Self::NamingContinuityAmbiguous,
        Self::NamingContinuityRejected,
        Self::ScopeLocalizationUnavailable,
        Self::DerivedFallbackExceeded,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OutOfClassEdit => "OutOfClassEdit",
            Self::InvariantBlocked => "InvariantBlocked",
            Self::NamingContinuityAmbiguous => "NamingContinuityAmbiguous",
            Self::NamingContinuityRejected => "NamingContinuityRejected",
            Self::ScopeLocalizationUnavailable => "ScopeLocalizationUnavailable",
            Self::DerivedFallbackExceeded => "DerivedFallbackExceeded",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RejectedEditScopeRow {
    pub family: TopologyEditFamily,
    pub rejection_class: TopologyEditRejectionClass,
    pub changed_scopes: Vec<TopologyEditChangedScope>,
    pub naming_scopes: Vec<TopologyEditNamingScope>,
    pub derived_regions: Vec<TopologyDerivedRegion>,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RejectedEditScopeReport {
    pub rows: Vec<RejectedEditScopeRow>,
}

impl TopologyOperatorExecutionError {
    pub fn rejection_class(&self) -> Option<TopologyEditRejectionClass> {
        match self {
            Self::UnsupportedMode(_)
            | Self::UnsupportedFamilies(_)
            | Self::DeclarationEntryRequired { .. }
            | Self::DeclarationEntryProgramRequired { .. } => {
                Some(TopologyEditRejectionClass::OutOfClassEdit)
            }
            Self::DeclarationEntry { .. } => Some(TopologyEditRejectionClass::OutOfClassEdit),
            Self::MissingCreatedEntityReference(_)
            | Self::MissingExistingEntityBinding(_)
            | Self::MissingExistingRelationBinding(_)
            | Self::ExistingEntityOutgoingRelationCountMismatch { .. }
            | Self::ExistingEntityIncomingRelationCountMismatch { .. } => {
                Some(TopologyEditRejectionClass::ScopeLocalizationUnavailable)
            }
            Self::CreatedEntityKindMismatch { .. }
            | Self::ExistingEntityKindMismatch { .. }
            | Self::ExistingRelationKindMismatch { .. }
            | Self::ExistingRelationSourceMismatch { .. }
            | Self::ExistingHalfEdgesNotOnSameEdge { .. }
            | Self::ExistingHalfEdgesNotOnSameLoop { .. } => {
                Some(TopologyEditRejectionClass::InvariantBlocked)
            }
            Self::Query(_) | Self::MaterializedDecode(_) | Self::UnexpectedInspectionFamily => None,
        }
    }

    pub fn rejected_contract_scope_report(
        &self,
        contracts: &[TopologyEditContract],
    ) -> Option<RejectedEditScopeReport> {
        let rejection_class = self.rejection_class()?;
        let detail = self.to_string();
        let rows = rejected_contracts(self, contracts)
            .into_iter()
            .map(|contract| RejectedEditScopeRow {
                family: contract.family,
                rejection_class,
                changed_scopes: contract.changed_scopes().to_vec(),
                naming_scopes: contract.naming_scopes().to_vec(),
                derived_regions: contract.derived_regions().to_vec(),
                detail: detail.clone(),
            })
            .collect();
        Some(RejectedEditScopeReport { rows })
    }
}

fn rejected_contracts<'a>(
    error: &TopologyOperatorExecutionError,
    contracts: &'a [TopologyEditContract],
) -> Vec<&'a TopologyEditContract> {
    match error {
        TopologyOperatorExecutionError::UnsupportedFamilies(families) => contracts
            .iter()
            .filter(|contract| families.contains(&contract.family))
            .collect(),
        TopologyOperatorExecutionError::DeclarationEntryRequired { family, .. } => contracts
            .iter()
            .filter(|contract| contract.family == *family)
            .collect(),
        TopologyOperatorExecutionError::DeclarationEntryProgramRequired { families, .. } => {
            contracts
                .iter()
                .filter(|contract| families.contains(&contract.family))
                .collect()
        }
        _ => contracts.iter().collect(),
    }
}
