use super::closeout_counters::WorthGraphReadAccessDeclarationCloseoutCounters;
use super::declaration_catalog_projection::WorthGraphReadDeclarationReadFamilyIdentity;
use super::execution_boundary;
use super::requirement_evidence_projection::WorthGraphReadRequirementRowDigestProjection;
use crate::graph_read_access_declarations::{
    WorthGraphReadAdmissionCapabilityGap, WorthGraphReadDeclarationCappedResidueReport,
    WorthGraphReadDeclarationDeletionLedgerReport, WorthGraphReadDeclarationSourceFirewallReport,
    WorthGraphReadRequirementDerivationCapabilityGap,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessDeclarationMilestoneEightSeed {
    milestone_seven_closeout_digest: String,
    declaration_catalog_digest: String,
    read_family_identities: Vec<WorthGraphReadDeclarationReadFamilyIdentity>,
    requirement_row_evidence: Vec<WorthGraphReadRequirementRowDigestProjection>,
    admission_capability_gaps: Vec<WorthGraphReadAdmissionCapabilityGap>,
    carried_requirement_derivation_gaps: Vec<WorthGraphReadRequirementDerivationCapabilityGap>,
    deletion_firewall_digest: String,
    deletion_ledger_report: WorthGraphReadDeclarationDeletionLedgerReport,
    capped_residue_report: WorthGraphReadDeclarationCappedResidueReport,
    source_firewall_report: WorthGraphReadDeclarationSourceFirewallReport,
    closeout_counters: WorthGraphReadAccessDeclarationCloseoutCounters,
    claims_graph_read_execution: bool,
    claims_access_plan_consumption: bool,
}

impl WorthGraphReadAccessDeclarationMilestoneEightSeed {
    pub(crate) fn new(
        milestone_seven_closeout_digest: impl Into<String>,
        declaration_catalog_digest: impl Into<String>,
        read_family_identities: Vec<WorthGraphReadDeclarationReadFamilyIdentity>,
        requirement_row_evidence: Vec<WorthGraphReadRequirementRowDigestProjection>,
        admission_capability_gaps: Vec<WorthGraphReadAdmissionCapabilityGap>,
        carried_requirement_derivation_gaps: Vec<WorthGraphReadRequirementDerivationCapabilityGap>,
        deletion_firewall_digest: impl Into<String>,
        deletion_ledger_report: WorthGraphReadDeclarationDeletionLedgerReport,
        capped_residue_report: WorthGraphReadDeclarationCappedResidueReport,
        source_firewall_report: WorthGraphReadDeclarationSourceFirewallReport,
        closeout_counters: WorthGraphReadAccessDeclarationCloseoutCounters,
    ) -> Self {
        Self {
            milestone_seven_closeout_digest: milestone_seven_closeout_digest.into(),
            declaration_catalog_digest: declaration_catalog_digest.into(),
            read_family_identities,
            requirement_row_evidence,
            admission_capability_gaps,
            carried_requirement_derivation_gaps,
            deletion_firewall_digest: deletion_firewall_digest.into(),
            deletion_ledger_report,
            capped_residue_report,
            source_firewall_report,
            closeout_counters,
            claims_graph_read_execution: execution_boundary::claims_graph_read_execution(),
            claims_access_plan_consumption: execution_boundary::claims_access_plan_consumption(),
        }
    }

    pub fn milestone_seven_closeout_digest(&self) -> &str {
        &self.milestone_seven_closeout_digest
    }

    pub fn declaration_catalog_digest(&self) -> &str {
        &self.declaration_catalog_digest
    }

    pub fn read_family_identities(&self) -> &[WorthGraphReadDeclarationReadFamilyIdentity] {
        &self.read_family_identities
    }

    pub fn requirement_row_evidence(&self) -> &[WorthGraphReadRequirementRowDigestProjection] {
        &self.requirement_row_evidence
    }

    pub fn admission_capability_gaps(&self) -> &[WorthGraphReadAdmissionCapabilityGap] {
        &self.admission_capability_gaps
    }

    pub fn carried_requirement_derivation_gaps(
        &self,
    ) -> &[WorthGraphReadRequirementDerivationCapabilityGap] {
        &self.carried_requirement_derivation_gaps
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

    pub fn closeout_counters(&self) -> &WorthGraphReadAccessDeclarationCloseoutCounters {
        &self.closeout_counters
    }

    pub const fn claims_graph_read_execution(&self) -> bool {
        self.claims_graph_read_execution
    }

    pub const fn claims_access_plan_consumption(&self) -> bool {
        self.claims_access_plan_consumption
    }

    #[cfg(test)]
    pub(crate) fn with_graph_read_execution_claim_for_tests(&self) -> Self {
        let mut seed = self.clone();
        seed.claims_graph_read_execution = true;
        seed
    }

    #[cfg(test)]
    pub(crate) fn with_access_plan_consumption_claim_for_tests(&self) -> Self {
        let mut seed = self.clone();
        seed.claims_access_plan_consumption = true;
        seed
    }

    #[cfg(test)]
    pub(crate) fn without_read_family_identities_for_tests(&self) -> Self {
        let mut seed = self.clone();
        seed.read_family_identities.clear();
        seed
    }

    #[cfg(test)]
    pub(crate) fn without_requirement_row_evidence_for_tests(&self) -> Self {
        let mut seed = self.clone();
        seed.requirement_row_evidence.clear();
        seed
    }
}
