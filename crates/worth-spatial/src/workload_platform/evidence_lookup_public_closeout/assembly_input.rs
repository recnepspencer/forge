use crate::workload_platform::evidence_ledger::SpatialEvidenceSurfaceDeletionLedgerRow;
use crate::workload_platform::evidence_lookup_query_consumer_kit::EvidenceLookupQueryConsumerKitCloseout;
use crate::workload_platform::evidence_lookup_query_surface_matrix::EvidenceLookupQuerySurfaceMatrixCloseout;
use crate::workload_platform::evidence_lookup_source_firewall::EvidenceLookupSourceFirewallReport;

use super::closeout_artifacts::EvidenceLookupPublicCloseoutFamilyStageRow;
use super::error::{EvidenceLookupPublicCloseoutError, EvidenceLookupPublicCloseoutErrorKind};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupPublicCloseoutAssemblyInput {
    family_stage_rows: Vec<EvidenceLookupPublicCloseoutFamilyStageRow>,
    query_surface_matrix: EvidenceLookupQuerySurfaceMatrixCloseout,
    query_consumer_kit: EvidenceLookupQueryConsumerKitCloseout,
    source_firewall_report: EvidenceLookupSourceFirewallReport,
    spatial_deletion_ledger_rows: Vec<SpatialEvidenceSurfaceDeletionLedgerRow>,
}

impl EvidenceLookupPublicCloseoutAssemblyInput {
    pub(crate) fn admit(
        family_stage_rows: Vec<EvidenceLookupPublicCloseoutFamilyStageRow>,
        query_surface_matrix: EvidenceLookupQuerySurfaceMatrixCloseout,
        query_consumer_kit: EvidenceLookupQueryConsumerKitCloseout,
        source_firewall_report: EvidenceLookupSourceFirewallReport,
        spatial_deletion_ledger_rows: Vec<SpatialEvidenceSurfaceDeletionLedgerRow>,
    ) -> Result<Self, EvidenceLookupPublicCloseoutError> {
        if family_stage_rows.is_empty() {
            return Err(EvidenceLookupPublicCloseoutError::new(
                EvidenceLookupPublicCloseoutErrorKind::EmptyFamilyCoverage,
                "public closeout requires at least one family-stage row",
            ));
        }
        Ok(Self {
            family_stage_rows,
            query_surface_matrix,
            query_consumer_kit,
            source_firewall_report,
            spatial_deletion_ledger_rows,
        })
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

    pub const fn source_firewall_report(&self) -> &EvidenceLookupSourceFirewallReport {
        &self.source_firewall_report
    }

    pub fn spatial_deletion_ledger_rows(&self) -> &[SpatialEvidenceSurfaceDeletionLedgerRow] {
        &self.spatial_deletion_ledger_rows
    }
}
