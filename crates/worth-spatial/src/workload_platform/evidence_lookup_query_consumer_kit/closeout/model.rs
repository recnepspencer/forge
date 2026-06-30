#[cfg(test)]
use crate::workload_platform::evidence_lookup_inventory::EvidenceLookupQuerySurface;
#[cfg(test)]
use crate::workload_platform::evidence_lookup_query_surface_matrix::EvidenceLookupQuerySurfaceTouchpoint;

use super::super::counters::EvidenceLookupQueryConsumerKitCounters;
#[cfg(test)]
use super::super::error::{
    EvidenceLookupQueryConsumerKitError, EvidenceLookupQueryConsumerKitErrorKind,
};
use super::super::requirement_row::EvidenceLookupQuerySupportRequirementRow;
use super::super::row::{
    EvidenceLookupQueryConsumerKitBindingRow, EvidenceLookupQueryConsumerResidueRow,
    EvidenceLookupQuerySupportPinRow,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupQueryConsumerKitCloseout {
    pub(super) query_surface_matrix_digest: String,
    pub(super) support_snapshot_digest: String,
    pub(super) support_pin_contract_digest: String,
    pub(super) support_pin_report_digest: String,
    pub(super) evidence_report_identity: String,
    pub(super) evidence_digest_participation_identity: String,
    pub(super) boundary_audit_coverage_identity: String,
    pub(super) boundary_audit_report_identity: String,
    pub(super) consumer_residue_report_identity: String,
    pub(super) consumer_residue_source_inventory_digest: String,
    pub(super) binding_rows: Vec<EvidenceLookupQueryConsumerKitBindingRow>,
    pub(super) support_requirement_rows: Vec<EvidenceLookupQuerySupportRequirementRow>,
    pub(super) support_rows: Vec<EvidenceLookupQuerySupportPinRow>,
    pub(super) query_residue_rows: Vec<EvidenceLookupQueryConsumerResidueRow>,
    pub(super) counters: EvidenceLookupQueryConsumerKitCounters,
    pub(super) closeout_digest: String,
}

impl EvidenceLookupQueryConsumerKitCloseout {
    pub fn query_surface_matrix_digest(&self) -> &str {
        &self.query_surface_matrix_digest
    }

    pub fn support_snapshot_digest(&self) -> &str {
        &self.support_snapshot_digest
    }

    pub fn support_pin_contract_digest(&self) -> &str {
        &self.support_pin_contract_digest
    }

    pub fn support_pin_report_digest(&self) -> &str {
        &self.support_pin_report_digest
    }

    pub fn evidence_report_identity(&self) -> &str {
        &self.evidence_report_identity
    }

    pub fn evidence_digest_participation_identity(&self) -> &str {
        &self.evidence_digest_participation_identity
    }

    pub fn boundary_audit_coverage_identity(&self) -> &str {
        &self.boundary_audit_coverage_identity
    }

    pub fn boundary_audit_report_identity(&self) -> &str {
        &self.boundary_audit_report_identity
    }

    pub fn consumer_residue_report_identity(&self) -> &str {
        &self.consumer_residue_report_identity
    }

    pub fn consumer_residue_source_inventory_digest(&self) -> &str {
        &self.consumer_residue_source_inventory_digest
    }

    pub fn binding_rows(&self) -> &[EvidenceLookupQueryConsumerKitBindingRow] {
        &self.binding_rows
    }

    #[cfg(test)]
    pub(crate) fn binding_rows_for_query_surface(
        &self,
        query_surface: EvidenceLookupQuerySurface,
    ) -> Vec<&EvidenceLookupQueryConsumerKitBindingRow> {
        self.binding_rows
            .iter()
            .filter(|row| row.query_surface() == query_surface)
            .collect()
    }

    #[cfg(test)]
    pub(crate) fn binding_rows_for_touchpoint(
        &self,
        touchpoint: EvidenceLookupQuerySurfaceTouchpoint,
    ) -> Vec<&EvidenceLookupQueryConsumerKitBindingRow> {
        self.binding_rows
            .iter()
            .filter(|row| row.touchpoint() == touchpoint)
            .collect()
    }

    #[cfg(test)]
    pub(crate) fn require_binding_row(
        &self,
        family_identity: &str,
        stage: crate::workload_platform::evidence_ledger::WorkloadEvidenceStage,
        touchpoint: EvidenceLookupQuerySurfaceTouchpoint,
    ) -> Result<&EvidenceLookupQueryConsumerKitBindingRow, EvidenceLookupQueryConsumerKitError>
    {
        self.binding_rows
            .iter()
            .find(|row| {
                row.family_identity() == family_identity
                    && row.stage() == stage
                    && row.touchpoint() == touchpoint
            })
            .ok_or_else(|| {
                EvidenceLookupQueryConsumerKitError::new(
                    EvidenceLookupQueryConsumerKitErrorKind::EmptyCloseout,
                    format!(
                        "missing consumer kit binding row for family `{family_identity}`, stage {:?}, touchpoint {}",
                        stage,
                        touchpoint.as_str()
                    ),
                )
            })
    }

    #[cfg(test)]
    pub(crate) fn query_surface_row_count(
        &self,
        query_surface: EvidenceLookupQuerySurface,
    ) -> usize {
        self.binding_rows_for_query_surface(query_surface).len()
    }

    #[cfg(test)]
    pub(crate) fn support_requirement_rows(&self) -> &[EvidenceLookupQuerySupportRequirementRow] {
        &self.support_requirement_rows
    }

    pub fn support_rows(&self) -> &[EvidenceLookupQuerySupportPinRow] {
        &self.support_rows
    }

    pub fn query_residue_rows(&self) -> &[EvidenceLookupQueryConsumerResidueRow] {
        &self.query_residue_rows
    }

    pub const fn counters(&self) -> &EvidenceLookupQueryConsumerKitCounters {
        &self.counters
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }

    pub const fn claims_spatial_lookup_residue_authority(&self) -> bool {
        false
    }
}
