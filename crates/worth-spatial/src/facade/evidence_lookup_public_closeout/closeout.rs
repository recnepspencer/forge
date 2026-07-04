use crate::workload_platform::evidence_ledger::SpatialEvidenceSurfaceDeletionLedgerRow;
use crate::workload_platform::evidence_lookup_query_consumer_kit::EvidenceLookupQueryConsumerKitCloseout;
use crate::workload_platform::evidence_lookup_query_surface_matrix::EvidenceLookupQuerySurfaceMatrixCloseout;
use crate::workload_platform::evidence_lookup_source_firewall::EvidenceLookupSourceFirewallReport;
use crate::workload_platform::evidence_lookup_workload_cutover::EvidenceLookupMilestoneTwelveSeed;

use super::counters::EvidenceLookupPublicCloseoutCounters;
use super::family_stage_row::EvidenceLookupPublicCloseoutFamilyStageRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupPublicCloseout {
    pub(crate) spatial_compiled_product_family_digest: String,
    pub(crate) family_stage_rows: Vec<EvidenceLookupPublicCloseoutFamilyStageRow>,
    pub(crate) query_surface_matrix: EvidenceLookupQuerySurfaceMatrixCloseout,
    pub(crate) query_consumer_kit: EvidenceLookupQueryConsumerKitCloseout,
    pub(crate) query_boundary_support_digest: String,
    pub(crate) source_firewall_report: EvidenceLookupSourceFirewallReport,
    pub(crate) spatial_deletion_ledger_rows: Vec<SpatialEvidenceSurfaceDeletionLedgerRow>,
    pub(crate) counters: EvidenceLookupPublicCloseoutCounters,
    pub(crate) family_coverage_digest: String,
    pub(crate) spatial_deletion_ledger_digest: String,
    pub(crate) residue_audit_digest: String,
    pub(crate) milestone_twelve_seed: EvidenceLookupMilestoneTwelveSeed,
    pub(crate) closeout_digest: String,
}

impl EvidenceLookupPublicCloseout {
    pub fn spatial_compiled_product_family_digest(&self) -> &str {
        &self.spatial_compiled_product_family_digest
    }
    pub fn family_stage_rows(&self) -> &[EvidenceLookupPublicCloseoutFamilyStageRow] {
        &self.family_stage_rows
    }
    pub const fn query_surface_matrix(&self) -> &EvidenceLookupQuerySurfaceMatrixCloseout {
        &self.query_surface_matrix
    }
    pub const fn query_consumer_kit(&self) -> &EvidenceLookupQueryConsumerKitCloseout {
        &self.query_consumer_kit
    }
    pub fn query_boundary_support_digest(&self) -> &str {
        &self.query_boundary_support_digest
    }
    pub const fn source_firewall_report(&self) -> &EvidenceLookupSourceFirewallReport {
        &self.source_firewall_report
    }
    pub fn spatial_deletion_ledger(&self) -> &[SpatialEvidenceSurfaceDeletionLedgerRow] {
        &self.spatial_deletion_ledger_rows
    }
    pub const fn counters(&self) -> &EvidenceLookupPublicCloseoutCounters {
        &self.counters
    }
    pub fn family_coverage_digest(&self) -> &str {
        &self.family_coverage_digest
    }
    pub fn spatial_deletion_ledger_digest(&self) -> &str {
        &self.spatial_deletion_ledger_digest
    }
    pub fn residue_audit_digest(&self) -> &str {
        &self.residue_audit_digest
    }
    pub fn milestone_twelve_seed(&self) -> &EvidenceLookupMilestoneTwelveSeed {
        &self.milestone_twelve_seed
    }
    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }
}
