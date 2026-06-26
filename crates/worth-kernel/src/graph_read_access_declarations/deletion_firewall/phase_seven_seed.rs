use super::capped_residue::WorthGraphReadDeclarationCappedResidueReport;
use super::deletion_ledger::WorthGraphReadDeclarationDeletionLedgerReport;
use super::source_firewall::WorthGraphReadDeclarationSourceFirewallReport;
use crate::graph_read_access_declarations::{
    WorthGraphReadAdmissionCapabilityGap, WorthGraphReadAdmissionPostureRecord,
    WorthGraphReadRequirementDerivationCapabilityGap,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessDeclarationPhaseSevenSeed {
    admission_closeout_digest: String,
    deletion_firewall_digest: String,
    deletion_ledger_report: WorthGraphReadDeclarationDeletionLedgerReport,
    capped_residue_report: WorthGraphReadDeclarationCappedResidueReport,
    source_firewall_report: WorthGraphReadDeclarationSourceFirewallReport,
    posture_records: Vec<WorthGraphReadAdmissionPostureRecord>,
    admission_capability_gaps: Vec<WorthGraphReadAdmissionCapabilityGap>,
    carried_requirement_derivation_gaps: Vec<WorthGraphReadRequirementDerivationCapabilityGap>,
}

impl WorthGraphReadAccessDeclarationPhaseSevenSeed {
    pub(crate) fn new(
        admission_closeout_digest: impl Into<String>,
        deletion_firewall_digest: impl Into<String>,
        deletion_ledger_report: WorthGraphReadDeclarationDeletionLedgerReport,
        capped_residue_report: WorthGraphReadDeclarationCappedResidueReport,
        source_firewall_report: WorthGraphReadDeclarationSourceFirewallReport,
        posture_records: Vec<WorthGraphReadAdmissionPostureRecord>,
        admission_capability_gaps: Vec<WorthGraphReadAdmissionCapabilityGap>,
        carried_requirement_derivation_gaps: Vec<WorthGraphReadRequirementDerivationCapabilityGap>,
    ) -> Self {
        Self {
            admission_closeout_digest: admission_closeout_digest.into(),
            deletion_firewall_digest: deletion_firewall_digest.into(),
            deletion_ledger_report,
            capped_residue_report,
            source_firewall_report,
            posture_records,
            admission_capability_gaps,
            carried_requirement_derivation_gaps,
        }
    }

    pub fn admission_closeout_digest(&self) -> &str {
        &self.admission_closeout_digest
    }

    pub fn deletion_firewall_digest(&self) -> &str {
        &self.deletion_firewall_digest
    }

    pub fn deletion_ledger_report(&self) -> &WorthGraphReadDeclarationDeletionLedgerReport {
        &self.deletion_ledger_report
    }

    pub fn capped_residue_report(&self) -> &WorthGraphReadDeclarationCappedResidueReport {
        &self.capped_residue_report
    }

    pub fn source_firewall_report(&self) -> &WorthGraphReadDeclarationSourceFirewallReport {
        &self.source_firewall_report
    }

    pub fn posture_records(&self) -> &[WorthGraphReadAdmissionPostureRecord] {
        &self.posture_records
    }

    pub fn admission_capability_gaps(&self) -> &[WorthGraphReadAdmissionCapabilityGap] {
        &self.admission_capability_gaps
    }

    pub fn carried_requirement_derivation_gaps(
        &self,
    ) -> &[WorthGraphReadRequirementDerivationCapabilityGap] {
        &self.carried_requirement_derivation_gaps
    }

    pub const fn claims_graph_read_execution(&self) -> bool {
        false
    }

    pub const fn claims_access_plan_consumption(&self) -> bool {
        false
    }
}
