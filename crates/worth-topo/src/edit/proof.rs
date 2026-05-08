use serde::{Deserialize, Serialize};

use crate::certification::DeterministicDigest;

use super::{
    TopologyDerivedRegion, TopologyEditBatch, TopologyEditChangedScope, TopologyEditContract,
    TopologyEditFamily, TopologyEditNamingOutcome, TopologyEditNamingRow, TopologyEditNamingScope,
    TopologyQueryEditExecutionError,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TopologyEditRejectionClass {
    OutOfClassEdit,
    InvariantBlocked,
    NamingContinuityAmbiguous,
    NamingContinuityRejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyEditDigest {
    pub digest: DeterministicDigest,
    pub contract_count: usize,
    pub family_count: usize,
    pub changed_scope_count: usize,
    pub naming_scope_count: usize,
    pub derived_region_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NamingEditContinuityMatrix {
    pub rows: Vec<TopologyEditNamingRow>,
    pub preserved_count: usize,
    pub ambiguous_count: usize,
    pub rejected_count: usize,
}

impl NamingEditContinuityMatrix {
    pub fn outcome_class(&self) -> TopologyEditNamingOutcome {
        if self.rejected_count > 0 {
            TopologyEditNamingOutcome::Rejected
        } else if self.ambiguous_count > 0 {
            TopologyEditNamingOutcome::Ambiguous
        } else {
            TopologyEditNamingOutcome::Preserved
        }
    }

    pub fn rejection_class(&self) -> Option<TopologyEditRejectionClass> {
        match self.outcome_class() {
            TopologyEditNamingOutcome::Preserved => None,
            TopologyEditNamingOutcome::Ambiguous => {
                Some(TopologyEditRejectionClass::NamingContinuityAmbiguous)
            }
            TopologyEditNamingOutcome::Rejected => {
                Some(TopologyEditRejectionClass::NamingContinuityRejected)
            }
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

impl TopologyEditBatch {
    pub fn topology_edit_digest(&self) -> TopologyEditDigest {
        let rows = self.contracts().iter().map(contract_digest_row);
        let changed_scope_count = self
            .contracts()
            .iter()
            .map(|contract| contract.changed_scopes().len())
            .sum();
        let naming_scope_count = self
            .contracts()
            .iter()
            .map(|contract| contract.naming_scopes().len())
            .sum();
        let derived_region_count = self
            .contracts()
            .iter()
            .map(|contract| contract.derived_regions().len())
            .sum();
        TopologyEditDigest {
            digest: digest_rows(rows),
            contract_count: self.contracts().len(),
            family_count: self.families().len(),
            changed_scope_count,
            naming_scope_count,
            derived_region_count,
        }
    }

    pub fn naming_edit_continuity_matrix(&self) -> NamingEditContinuityMatrix {
        let rows = self.naming_report().rows;
        let preserved_count = rows
            .iter()
            .filter(|row| row.outcome == TopologyEditNamingOutcome::Preserved)
            .count();
        let ambiguous_count = rows
            .iter()
            .filter(|row| row.outcome == TopologyEditNamingOutcome::Ambiguous)
            .count();
        let rejected_count = rows
            .iter()
            .filter(|row| row.outcome == TopologyEditNamingOutcome::Rejected)
            .count();
        NamingEditContinuityMatrix {
            rows,
            preserved_count,
            ambiguous_count,
            rejected_count,
        }
    }
}

impl TopologyQueryEditExecutionError {
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
            Self::Query(_)
            | Self::Surface(_)
            | Self::MaterializedDecode(_)
            | Self::UnexpectedInspectionFamily => None,
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
    error: &TopologyQueryEditExecutionError,
    batch: &'a TopologyEditBatch,
) -> Vec<&'a TopologyEditContract> {
    match error {
        TopologyQueryEditExecutionError::UnsupportedFamilies(families) => batch
            .contracts()
            .iter()
            .filter(|contract| families.contains(&contract.family))
            .collect(),
        _ => batch.contracts().iter().collect(),
    }
}

fn contract_digest_row(contract: &TopologyEditContract) -> String {
    serde_json::to_string(contract).expect(" topology edit contracts should serialize")
}

fn digest_rows(rows: impl IntoIterator<Item = String>) -> DeterministicDigest {
    let mut count = 0usize;
    let mut hash = 0xcbf29ce484222325u64;
    for row in rows {
        count += 1;
        for byte in row.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= u64::from(b'\n');
        hash = hash.wrapping_mul(0x100000001b3);
    }
    DeterministicDigest {
        algorithm: "fnv1a64".to_string(),
        digest_hex: format!("{hash:016x}"),
        row_count: count,
    }
}
