use super::closeout_counters::WorthGraphReadAccessDeclarationCloseoutCounters;
use super::declaration_catalog_projection::{
    WorthGraphReadDeclarationCatalogIdentityProjection, WorthGraphReadDeclarationReadFamilyIdentity,
};
use super::errors::{
    WorthGraphReadAccessDeclarationCloseoutError, WorthGraphReadAccessDeclarationCloseoutErrorKind,
};
use super::execution_boundary;
use super::milestone_eight_seed::WorthGraphReadAccessDeclarationMilestoneEightSeed;
use super::proof_digest::stable_digest;
use super::requirement_evidence_projection::{
    WorthGraphReadRequirementEvidenceSummary, WorthGraphReadRequirementRowDigestProjection,
};
use crate::graph_read_access_declarations::{
    WorthGraphReadAccessDeclarationPhaseSevenSeed, WorthGraphReadAdmissionCapabilityGap,
    WorthGraphReadDeclarationCappedResidueReport, WorthGraphReadDeclarationDeletionLedgerReport,
    WorthGraphReadDeclarationSourceFirewallReport,
    WorthGraphReadRequirementDerivationCapabilityGap,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessDeclarationCloseout {
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
    milestone_eight_seed: WorthGraphReadAccessDeclarationMilestoneEightSeed,
    closeout_digest: String,
}

pub fn current_worth_graph_read_access_declaration_closeout(
    seed: &WorthGraphReadAccessDeclarationPhaseSevenSeed,
) -> Result<WorthGraphReadAccessDeclarationCloseout, WorthGraphReadAccessDeclarationCloseoutError> {
    execution_boundary::reject_execution_shaped_seed(seed)?;
    if seed.posture_records().is_empty() {
        return Err(WorthGraphReadAccessDeclarationCloseoutError::new(
            WorthGraphReadAccessDeclarationCloseoutErrorKind::MissingAdmissionPostureProof,
        ));
    }

    let catalog_projection =
        WorthGraphReadDeclarationCatalogIdentityProjection::from_posture_records(
            seed.posture_records(),
        );
    let requirement_summary =
        WorthGraphReadRequirementEvidenceSummary::from_posture_records(seed.posture_records());
    let read_family_identities = catalog_projection.read_family_identities().to_vec();
    let requirement_row_evidence = requirement_summary.requirement_row_evidence().to_vec();
    let closeout_counters = WorthGraphReadAccessDeclarationCloseoutCounters::new(
        seed.posture_records().len(),
        read_family_identities.len(),
        requirement_row_evidence.len(),
        seed.admission_capability_gaps().len(),
        seed.carried_requirement_derivation_gaps().len(),
        seed.deletion_ledger_report().rows().len(),
        seed.capped_residue_report().rows().len(),
        seed.source_firewall_report().region_reports().len(),
    );
    let closeout_digest = closeout_digest(
        seed,
        catalog_projection.catalog_digest(),
        &read_family_identities,
        requirement_summary.requirement_summary_digest(),
        &requirement_row_evidence,
        &closeout_counters,
    );
    let milestone_eight_seed = WorthGraphReadAccessDeclarationMilestoneEightSeed::new(
        closeout_digest.clone(),
        catalog_projection.catalog_digest(),
        read_family_identities.clone(),
        requirement_row_evidence.clone(),
        seed.admission_capability_gaps().to_vec(),
        seed.carried_requirement_derivation_gaps().to_vec(),
        seed.deletion_firewall_digest(),
        seed.deletion_ledger_report().clone(),
        seed.capped_residue_report().clone(),
        seed.source_firewall_report().clone(),
        closeout_counters.clone(),
    );

    Ok(WorthGraphReadAccessDeclarationCloseout {
        declaration_catalog_digest: catalog_projection.catalog_digest().to_string(),
        read_family_identities,
        requirement_row_evidence,
        admission_capability_gaps: seed.admission_capability_gaps().to_vec(),
        carried_requirement_derivation_gaps: seed.carried_requirement_derivation_gaps().to_vec(),
        deletion_firewall_digest: seed.deletion_firewall_digest().to_string(),
        deletion_ledger_report: seed.deletion_ledger_report().clone(),
        capped_residue_report: seed.capped_residue_report().clone(),
        source_firewall_report: seed.source_firewall_report().clone(),
        closeout_counters,
        milestone_eight_seed,
        closeout_digest,
    })
}

impl WorthGraphReadAccessDeclarationCloseout {
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

    pub fn milestone_eight_seed(&self) -> &WorthGraphReadAccessDeclarationMilestoneEightSeed {
        &self.milestone_eight_seed
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }

    pub const fn claims_graph_read_execution(&self) -> bool {
        execution_boundary::claims_graph_read_execution()
    }

    pub const fn claims_access_plan_consumption(&self) -> bool {
        execution_boundary::claims_access_plan_consumption()
    }

    pub const fn claims_graph_read_receipts_complete(&self) -> bool {
        execution_boundary::claims_graph_read_receipts_complete()
    }

    pub const fn claims_milestone_eight_access_plan_adoption(&self) -> bool {
        execution_boundary::claims_milestone_eight_access_plan_adoption()
    }
}

fn closeout_digest(
    seed: &WorthGraphReadAccessDeclarationPhaseSevenSeed,
    declaration_catalog_digest: &str,
    read_family_identities: &[WorthGraphReadDeclarationReadFamilyIdentity],
    requirement_summary_digest: &str,
    requirement_row_evidence: &[WorthGraphReadRequirementRowDigestProjection],
    counters: &WorthGraphReadAccessDeclarationCloseoutCounters,
) -> String {
    let mut parts = vec![
        "worth_graph_read_access_declaration_milestone_seven_closeout_v1".to_string(),
        format!("admission_closeout:{}", seed.admission_closeout_digest()),
        format!("deletion_firewall:{}", seed.deletion_firewall_digest()),
        format!("declaration_catalog:{declaration_catalog_digest}"),
        format!("requirement_summary:{requirement_summary_digest}"),
        format!(
            "admission_gap_count:{}",
            counters.admission_capability_gap_count()
        ),
        format!(
            "carried_requirement_gap_count:{}",
            counters.carried_requirement_derivation_gap_count()
        ),
        format!(
            "deletion_ledger:{}",
            seed.deletion_ledger_report().report_digest()
        ),
        format!(
            "capped_residue:{}",
            seed.capped_residue_report().report_digest()
        ),
        format!(
            "source_firewall:{}",
            seed.source_firewall_report().report_digest()
        ),
    ];
    parts.extend(
        read_family_identities
            .iter()
            .map(|identity| format!("read_family:{}", identity.identity_digest())),
    );
    parts.extend(
        requirement_row_evidence
            .iter()
            .map(|row| format!("requirement_row:{}", row.requirement_row_digest())),
    );
    parts.extend(
        seed.admission_capability_gaps()
            .iter()
            .map(|gap| format!("admission_gap:{}", gap.gap_digest())),
    );
    parts.extend(
        seed.carried_requirement_derivation_gaps()
            .iter()
            .map(|gap| format!("requirement_gap:{}", gap.gap_digest())),
    );
    stable_digest(&parts)
}
