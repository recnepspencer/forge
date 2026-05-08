use serde::{Deserialize, Serialize};

use super::{
    TopologyDerivedRegion, TopologyEditBatch, TopologyEditChangedScope, TopologyEditContract,
    TopologyEditFamily, TopologyEditNamingScope, TopologyOperatorExecutionError,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TopologyEditRejectionClass {
    OutOfClassEdit,
    InvariantBlocked,
    NamingContinuityAmbiguous,
    NamingContinuityRejected,
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
            Self::UnsupportedMode(_) | Self::UnsupportedFamilies(_) => {
                Some(TopologyEditRejectionClass::OutOfClassEdit)
            }
            Self::MissingCreatedEntityReference(_)
            | Self::MissingExistingEntityBinding(_)
            | Self::MissingExistingRelationBinding(_)
            | Self::CreatedEntityKindMismatch { .. }
            | Self::ExistingEntityKindMismatch { .. }
            | Self::ExistingRelationKindMismatch { .. }
            | Self::ExistingRelationSourceMismatch { .. }
            | Self::ExistingEntityOutgoingRelationCountMismatch { .. }
            | Self::ExistingEntityIncomingRelationCountMismatch { .. }
            | Self::ExistingHalfEdgesNotOnSameEdge { .. }
            | Self::ExistingHalfEdgesNotOnSameLoop { .. } => {
                Some(TopologyEditRejectionClass::InvariantBlocked)
            }
            Self::Query(_) | Self::MaterializedDecode(_) | Self::UnexpectedInspectionFamily => None,
        }
    }

    pub fn rejected_edit_scope_report(
        &self,
        batch: &TopologyEditBatch,
    ) -> Option<RejectedEditScopeReport> {
        let rejection_class = self.rejection_class()?;
        let detail = self.to_string();
        let rows = rejected_contracts(self, batch)
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
    batch: &'a TopologyEditBatch,
) -> Vec<&'a TopologyEditContract> {
    match error {
        TopologyOperatorExecutionError::UnsupportedFamilies(families) => batch
            .contracts()
            .iter()
            .filter(|contract| families.contains(&contract.family))
            .collect(),
        _ => batch.contracts().iter().collect(),
    }
}
