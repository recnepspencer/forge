use serde::{Deserialize, Serialize};

use crate::certification::WorthDeterministicDigest;

use super::{
    WorthTopologyDerivedRegion, WorthTopologyEditBatch, WorthTopologyEditChangedScope,
    WorthTopologyEditContract, WorthTopologyEditFamily, WorthTopologyEditNamingOutcome,
    WorthTopologyEditNamingRow, WorthTopologyEditNamingScope, WorthTopologyQueryEditExecutionError,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum WorthTopologyEditRejectionClass {
    OutOfClassEdit,
    InvariantBlocked,
    NamingContinuityAmbiguous,
    NamingContinuityRejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthTopologyEditDigest {
    pub digest: WorthDeterministicDigest,
    pub contract_count: usize,
    pub family_count: usize,
    pub changed_scope_count: usize,
    pub naming_scope_count: usize,
    pub derived_region_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthNamingEditContinuityMatrix {
    pub rows: Vec<WorthTopologyEditNamingRow>,
    pub preserved_count: usize,
    pub ambiguous_count: usize,
    pub rejected_count: usize,
}

impl WorthNamingEditContinuityMatrix {
    pub fn outcome_class(&self) -> WorthTopologyEditNamingOutcome {
        if self.rejected_count > 0 {
            WorthTopologyEditNamingOutcome::Rejected
        } else if self.ambiguous_count > 0 {
            WorthTopologyEditNamingOutcome::Ambiguous
        } else {
            WorthTopologyEditNamingOutcome::Preserved
        }
    }

    pub fn rejection_class(&self) -> Option<WorthTopologyEditRejectionClass> {
        match self.outcome_class() {
            WorthTopologyEditNamingOutcome::Preserved => None,
            WorthTopologyEditNamingOutcome::Ambiguous => {
                Some(WorthTopologyEditRejectionClass::NamingContinuityAmbiguous)
            }
            WorthTopologyEditNamingOutcome::Rejected => {
                Some(WorthTopologyEditRejectionClass::NamingContinuityRejected)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthRejectedEditScopeRow {
    pub family: WorthTopologyEditFamily,
    pub rejection_class: WorthTopologyEditRejectionClass,
    pub changed_scopes: Vec<WorthTopologyEditChangedScope>,
    pub naming_scopes: Vec<WorthTopologyEditNamingScope>,
    pub derived_regions: Vec<WorthTopologyDerivedRegion>,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthRejectedEditScopeReport {
    pub rows: Vec<WorthRejectedEditScopeRow>,
}

impl WorthTopologyEditBatch {
    pub fn topology_edit_digest(&self) -> WorthTopologyEditDigest {
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
        WorthTopologyEditDigest {
            digest: digest_rows(rows),
            contract_count: self.contracts().len(),
            family_count: self.families().len(),
            changed_scope_count,
            naming_scope_count,
            derived_region_count,
        }
    }

    pub fn naming_edit_continuity_matrix(&self) -> WorthNamingEditContinuityMatrix {
        let rows = self.naming_report().rows;
        let preserved_count = rows
            .iter()
            .filter(|row| row.outcome == WorthTopologyEditNamingOutcome::Preserved)
            .count();
        let ambiguous_count = rows
            .iter()
            .filter(|row| row.outcome == WorthTopologyEditNamingOutcome::Ambiguous)
            .count();
        let rejected_count = rows
            .iter()
            .filter(|row| row.outcome == WorthTopologyEditNamingOutcome::Rejected)
            .count();
        WorthNamingEditContinuityMatrix {
            rows,
            preserved_count,
            ambiguous_count,
            rejected_count,
        }
    }
}

impl WorthTopologyQueryEditExecutionError {
    pub fn rejection_class(&self) -> Option<WorthTopologyEditRejectionClass> {
        match self {
            Self::UnsupportedMode(_) | Self::UnsupportedFamilies(_) => {
                Some(WorthTopologyEditRejectionClass::OutOfClassEdit)
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
                Some(WorthTopologyEditRejectionClass::InvariantBlocked)
            }
            Self::Query(_)
            | Self::Surface(_)
            | Self::MaterializedDecode(_)
            | Self::UnexpectedInspectionFamily => None,
        }
    }

    pub fn rejected_edit_scope_report(
        &self,
        batch: &WorthTopologyEditBatch,
    ) -> Option<WorthRejectedEditScopeReport> {
        let rejection_class = self.rejection_class()?;
        let detail = self.to_string();
        let rows = rejected_contracts(self, batch)
            .into_iter()
            .map(|contract| WorthRejectedEditScopeRow {
                family: contract.family,
                rejection_class,
                changed_scopes: contract.changed_scopes().to_vec(),
                naming_scopes: contract.naming_scopes().to_vec(),
                derived_regions: contract.derived_regions().to_vec(),
                detail: detail.clone(),
            })
            .collect();
        Some(WorthRejectedEditScopeReport { rows })
    }
}

fn rejected_contracts<'a>(
    error: &WorthTopologyQueryEditExecutionError,
    batch: &'a WorthTopologyEditBatch,
) -> Vec<&'a WorthTopologyEditContract> {
    match error {
        WorthTopologyQueryEditExecutionError::UnsupportedFamilies(families) => batch
            .contracts()
            .iter()
            .filter(|contract| families.contains(&contract.family))
            .collect(),
        _ => batch.contracts().iter().collect(),
    }
}

fn contract_digest_row(contract: &WorthTopologyEditContract) -> String {
    serde_json::to_string(contract).expect("worth topology edit contracts should serialize")
}

fn digest_rows(rows: impl IntoIterator<Item = String>) -> WorthDeterministicDigest {
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
    WorthDeterministicDigest {
        algorithm: "fnv1a64".to_string(),
        digest_hex: format!("{hash:016x}"),
        row_count: count,
    }
}
