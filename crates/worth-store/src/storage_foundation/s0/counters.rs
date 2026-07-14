use super::evidence::{S0ArtifactValidationReport, S0ComplexityContractName};
use super::manifest::{S0AuditInputManifest, S0InputManifestDelta};
use super::milestones::{MilestonePhysicalStatusRow, RoadmapSequenceStatusMatrix};
use super::{
    DeferredPhysicalGuaranteeMap, ReleaseClaimReport, S0ArtifactValidationCostSurface,
    SemanticPhysicalClaimReport, TerminologyAllowedUse, TerminologyRiskReport,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum S0ComplexityStatus {
    Declared,
    Verified,
    Debt,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct S0ComplexityContract {
    name: &'static str,
    status: S0ComplexityStatus,
    max_global_scans: u64,
    max_unindexed_repo_passes: u64,
}

impl S0ComplexityContract {
    pub fn declared(name: &'static str) -> Self {
        Self {
            name,
            status: S0ComplexityStatus::Declared,
            max_global_scans: 0,
            max_unindexed_repo_passes: 0,
        }
    }

    pub fn verified(
        name: &'static str,
        max_global_scans: u64,
        max_unindexed_repo_passes: u64,
    ) -> Self {
        Self {
            name,
            status: S0ComplexityStatus::Verified,
            max_global_scans,
            max_unindexed_repo_passes,
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn status(&self) -> S0ComplexityStatus {
        self.status
    }

    pub fn max_global_scans(&self) -> u64 {
        self.max_global_scans
    }

    pub fn max_unindexed_repo_passes(&self) -> u64 {
        self.max_unindexed_repo_passes
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct S0ComplexityContractReport {
    required_contract_count: u64,
    observed_contract_count: u64,
    missing_required_contracts: Vec<&'static str>,
    duplicate_contracts: Vec<&'static str>,
    debt_contracts: Vec<&'static str>,
    max_global_scans: u64,
    max_unindexed_repo_passes: u64,
}

impl S0ComplexityContractReport {
    pub fn from_contracts(
        required_contracts: impl IntoIterator<Item = S0ComplexityContractName>,
        observed_contracts: impl IntoIterator<Item = S0ComplexityContract>,
    ) -> Self {
        let required = required_contracts
            .into_iter()
            .map(|name| name.as_str())
            .collect::<BTreeSet<_>>();
        let mut duplicate_names = BTreeSet::new();
        let mut observed = BTreeMap::new();
        for contract in observed_contracts {
            if observed.insert(contract.name(), contract.clone()).is_some() {
                duplicate_names.insert(contract.name());
            }
        }
        let missing_required_contracts = required
            .iter()
            .filter(|name| !observed.contains_key(**name))
            .copied()
            .collect::<Vec<_>>();
        let duplicate_contracts = duplicate_names.into_iter().collect::<Vec<_>>();
        let debt_contracts = observed
            .values()
            .filter(|contract| contract.status() != S0ComplexityStatus::Verified)
            .map(S0ComplexityContract::name)
            .collect::<Vec<_>>();
        let max_global_scans = observed
            .values()
            .map(S0ComplexityContract::max_global_scans)
            .sum();
        let max_unindexed_repo_passes = observed
            .values()
            .map(S0ComplexityContract::max_unindexed_repo_passes)
            .sum();
        Self {
            required_contract_count: required.len() as u64,
            observed_contract_count: observed.len() as u64,
            missing_required_contracts,
            duplicate_contracts,
            debt_contracts,
            max_global_scans,
            max_unindexed_repo_passes,
        }
    }

    pub fn observed_contract_count(&self) -> u64 {
        self.observed_contract_count
    }

    pub fn required_contract_count(&self) -> u64 {
        self.required_contract_count
    }

    pub fn missing_complexity_contract_count(&self) -> u64 {
        self.missing_required_contracts.len() as u64
    }

    pub fn duplicate_complexity_contract_count(&self) -> u64 {
        self.duplicate_contracts.len() as u64
    }

    pub fn complexity_debt_count(&self) -> u64 {
        self.debt_contracts.len() as u64
    }

    pub fn max_global_scans(&self) -> u64 {
        self.max_global_scans
    }

    pub fn max_unindexed_repo_passes(&self) -> u64 {
        self.max_unindexed_repo_passes
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct S0CounterSnapshot {
    required_artifact_count: u64,
    missing_required_artifact_count: u64,
    schema_incompatible_artifact_count: u64,
    complexity_contract_count: u64,
    missing_complexity_contract_count: u64,
    duplicate_complexity_contract_count: u64,
    complexity_debt_count: u64,
    forbidden_platform_grade_claim_count: u64,
    missing_first_audit_row_count: u64,
    roadmap_sequence_edge_count: u64,
    sequence_inconsistency_count: u64,
    spec_closeout_status_mismatch_count: u64,
    closed_with_unclosed_prerequisite_count: u64,
    milestone_status_row_count: u64,
    missing_milestone_status_row_count: u64,
    unmapped_deferred_guarantee_count: u64,
    semantic_claim_count: u64,
    physical_claim_count: u64,
    unclassified_terminology_finding_count: u64,
    evidence_ref_reresolution_count: u64,
    s1_unmet_blocking_prerequisite_count: u64,
    overclaimed_physical_phrase_count: u64,
    unwaived_sequence_inconsistency_count: u64,
    unqualified_release_claim_count: u64,
    stale_evidence_rejection_count: u64,
    broad_scan_rejection_count: u64,
    release_claim_scan_count: u64,
    public_claim_rejection_count: u64,
    input_manifest_file_count: u64,
    input_manifest_byte_count: u64,
    input_manifest_reused_file_count: u64,
    input_manifest_rescanned_file_count: u64,
    requested_scan_scope_count: u64,
    admitted_scan_scope_count: u64,
    rejected_scan_scope_count: u64,
    scanned_file_count: u64,
    scanned_byte_count: u64,
    unique_evidence_ref_count: u64,
    digest_row_byte_count: u64,
}

impl S0CounterSnapshot {
    pub fn from_artifact_and_complexity_reports(
        report: &S0ArtifactValidationReport,
        complexity: &S0ComplexityContractReport,
    ) -> Self {
        Self {
            required_artifact_count: report.required_artifact_count(),
            missing_required_artifact_count: report.missing_required_artifact_count(),
            schema_incompatible_artifact_count: report.schema_incompatible_artifact_count(),
            complexity_contract_count: complexity.observed_contract_count(),
            missing_complexity_contract_count: complexity.missing_complexity_contract_count(),
            duplicate_complexity_contract_count: complexity.duplicate_complexity_contract_count(),
            complexity_debt_count: complexity.complexity_debt_count(),
            forbidden_platform_grade_claim_count: 0,
            missing_first_audit_row_count: 0,
            roadmap_sequence_edge_count: 0,
            sequence_inconsistency_count: 0,
            spec_closeout_status_mismatch_count: 0,
            closed_with_unclosed_prerequisite_count: 0,
            milestone_status_row_count: 0,
            missing_milestone_status_row_count: 0,
            unmapped_deferred_guarantee_count: 0,
            semantic_claim_count: 0,
            physical_claim_count: 0,
            unclassified_terminology_finding_count: 0,
            evidence_ref_reresolution_count: 0,
            s1_unmet_blocking_prerequisite_count: 0,
            overclaimed_physical_phrase_count: 0,
            unwaived_sequence_inconsistency_count: 0,
            unqualified_release_claim_count: 0,
            stale_evidence_rejection_count: 0,
            broad_scan_rejection_count: 0,
            release_claim_scan_count: 0,
            public_claim_rejection_count: 0,
            input_manifest_file_count: 0,
            input_manifest_byte_count: 0,
            input_manifest_reused_file_count: 0,
            input_manifest_rescanned_file_count: 0,
            requested_scan_scope_count: 0,
            admitted_scan_scope_count: 0,
            rejected_scan_scope_count: 0,
            scanned_file_count: 0,
            scanned_byte_count: 0,
            unique_evidence_ref_count: 0,
            digest_row_byte_count: 0,
        }
    }

    pub fn required_artifact_count(&self) -> u64 {
        self.required_artifact_count
    }

    pub fn missing_required_artifact_count(&self) -> u64 {
        self.missing_required_artifact_count
    }

    pub fn schema_incompatible_artifact_count(&self) -> u64 {
        self.schema_incompatible_artifact_count
    }

    pub fn complexity_contract_count(&self) -> u64 {
        self.complexity_contract_count
    }

    pub fn missing_complexity_contract_count(&self) -> u64 {
        self.missing_complexity_contract_count
    }

    pub fn duplicate_complexity_contract_count(&self) -> u64 {
        self.duplicate_complexity_contract_count
    }

    pub fn complexity_debt_count(&self) -> u64 {
        self.complexity_debt_count
    }

    pub fn evidence_ref_reresolution_count(&self) -> u64 {
        self.evidence_ref_reresolution_count
    }

    pub fn input_manifest_file_count(&self) -> u64 {
        self.input_manifest_file_count
    }

    pub fn input_manifest_byte_count(&self) -> u64 {
        self.input_manifest_byte_count
    }

    pub fn input_manifest_reused_file_count(&self) -> u64 {
        self.input_manifest_reused_file_count
    }

    pub fn input_manifest_rescanned_file_count(&self) -> u64 {
        self.input_manifest_rescanned_file_count
    }

    pub fn requested_scan_scope_count(&self) -> u64 {
        self.requested_scan_scope_count
    }

    pub fn admitted_scan_scope_count(&self) -> u64 {
        self.admitted_scan_scope_count
    }

    pub fn rejected_scan_scope_count(&self) -> u64 {
        self.rejected_scan_scope_count
    }

    pub fn roadmap_sequence_edge_count(&self) -> u64 {
        self.roadmap_sequence_edge_count
    }

    pub fn sequence_inconsistency_count(&self) -> u64 {
        self.sequence_inconsistency_count
    }

    pub fn spec_closeout_status_mismatch_count(&self) -> u64 {
        self.spec_closeout_status_mismatch_count
    }

    pub fn closed_with_unclosed_prerequisite_count(&self) -> u64 {
        self.closed_with_unclosed_prerequisite_count
    }

    pub fn milestone_status_row_count(&self) -> u64 {
        self.milestone_status_row_count
    }

    pub fn missing_milestone_status_row_count(&self) -> u64 {
        self.missing_milestone_status_row_count
    }

    pub fn semantic_claim_count(&self) -> u64 {
        self.semantic_claim_count
    }

    pub fn physical_claim_count(&self) -> u64 {
        self.physical_claim_count
    }

    pub fn unmapped_deferred_guarantee_count(&self) -> u64 {
        self.unmapped_deferred_guarantee_count
    }

    pub fn overclaimed_physical_phrase_count(&self) -> u64 {
        self.overclaimed_physical_phrase_count
    }

    pub fn unclassified_terminology_finding_count(&self) -> u64 {
        self.unclassified_terminology_finding_count
    }

    pub fn unqualified_release_claim_count(&self) -> u64 {
        self.unqualified_release_claim_count
    }

    pub fn release_claim_scan_count(&self) -> u64 {
        self.release_claim_scan_count
    }

    pub fn public_claim_rejection_count(&self) -> u64 {
        self.public_claim_rejection_count
    }

    pub fn unique_evidence_ref_count(&self) -> u64 {
        self.unique_evidence_ref_count
    }

    pub fn digest_row_byte_count(&self) -> u64 {
        self.digest_row_byte_count
    }

    pub fn with_validation_costs<'a>(
        mut self,
        costs: impl IntoIterator<Item = &'a S0ArtifactValidationCostSurface>,
    ) -> Self {
        self.digest_row_byte_count = costs
            .into_iter()
            .map(S0ArtifactValidationCostSurface::canonicalized_row_byte_count)
            .sum();
        self
    }

    pub fn with_input_manifest(
        mut self,
        manifest: &S0AuditInputManifest,
        delta: Option<&S0InputManifestDelta>,
    ) -> Self {
        self.input_manifest_file_count = manifest.breadth_summary().matched_file_count();
        self.input_manifest_byte_count = manifest.breadth_summary().matched_byte_count();
        self.requested_scan_scope_count = manifest.scan_cost().requested_scan_scope_count();
        self.admitted_scan_scope_count = manifest.scan_cost().admitted_scan_scope_count();
        self.rejected_scan_scope_count = manifest.scan_cost().rejected_scan_scope_count();
        self.scanned_file_count = manifest.scan_cost().scanned_file_count();
        self.scanned_byte_count = manifest.scan_cost().scanned_byte_count();
        self.broad_scan_rejection_count = manifest.scan_cost().rejected_scan_scope_count();
        if let Some(delta) = delta {
            self.input_manifest_reused_file_count = delta.reused_file_count();
            self.input_manifest_rescanned_file_count = delta.rescanned_file_count();
        }
        self
    }

    pub fn with_sequence_matrix(mut self, matrix: &RoadmapSequenceStatusMatrix) -> Self {
        self.roadmap_sequence_edge_count = matrix.prerequisite_edges().len() as u64;
        self.sequence_inconsistency_count = matrix.inconsistencies().len() as u64;
        self.spec_closeout_status_mismatch_count = matrix
            .inconsistencies()
            .iter()
            .filter(|(_, inconsistency)| {
                *inconsistency
                    == super::milestones::MilestoneSequenceInconsistency::SpecCloseoutStatusMismatch
            })
            .count() as u64;
        self.closed_with_unclosed_prerequisite_count = matrix
            .inconsistencies()
            .iter()
            .filter(|(_, inconsistency)| {
                *inconsistency
                    == super::milestones::MilestoneSequenceInconsistency::ClosedWithUnclosedPrerequisite
            })
            .count() as u64;
        self.unwaived_sequence_inconsistency_count = matrix.unwaived_inconsistency_count();
        self.s1_unmet_blocking_prerequisite_count = matrix.unwaived_inconsistency_count();
        self
    }

    pub fn with_milestone_status_rows(
        mut self,
        rows: &[MilestonePhysicalStatusRow],
        required_row_count: u64,
    ) -> Self {
        self.milestone_status_row_count = rows.len() as u64;
        self.missing_milestone_status_row_count =
            required_row_count.saturating_sub(self.milestone_status_row_count);
        self.semantic_claim_count = rows
            .iter()
            .flat_map(MilestonePhysicalStatusRow::claim_families)
            .filter(|family| {
                matches!(
                    family,
                    super::milestones::SemanticPhysicalClaimFamily::SemanticAuthority
                        | super::milestones::SemanticPhysicalClaimFamily::RecoverySemantics
                        | super::milestones::SemanticPhysicalClaimFamily::RetentionSemantics
                        | super::milestones::SemanticPhysicalClaimFamily::SubscriptionSupport
                        | super::milestones::SemanticPhysicalClaimFamily::CompatibilitySemantics
                        | super::milestones::SemanticPhysicalClaimFamily::TieringPlacement
                        | super::milestones::SemanticPhysicalClaimFamily::ReplicationSemantics
                )
            })
            .count() as u64;
        self.physical_claim_count = rows
            .iter()
            .flat_map(MilestonePhysicalStatusRow::claim_families)
            .filter(|family| {
                matches!(
                    family,
                    super::milestones::SemanticPhysicalClaimFamily::PhysicalSubstrate
                        | super::milestones::SemanticPhysicalClaimFamily::PhysicalBoundedness
                        | super::milestones::SemanticPhysicalClaimFamily::PhysicalIntegrity
                        | super::milestones::SemanticPhysicalClaimFamily::PhysicalRecoveryPhysics
                        | super::milestones::SemanticPhysicalClaimFamily::PhysicalIsolation
                        | super::milestones::SemanticPhysicalClaimFamily::PhysicalIo
                        | super::milestones::SemanticPhysicalClaimFamily::PhysicalOperationalSafety
                        | super::milestones::SemanticPhysicalClaimFamily::PhysicalSecurity
                )
            })
            .count() as u64;
        self.forbidden_platform_grade_claim_count = 0;
        self
    }

    pub fn with_claim_report(mut self, report: &SemanticPhysicalClaimReport) -> Self {
        self.semantic_claim_count = report
            .rows()
            .iter()
            .filter(|row| {
                matches!(
                    row.claim_family(),
                    super::milestones::SemanticPhysicalClaimFamily::SemanticAuthority
                        | super::milestones::SemanticPhysicalClaimFamily::RecoverySemantics
                        | super::milestones::SemanticPhysicalClaimFamily::RetentionSemantics
                        | super::milestones::SemanticPhysicalClaimFamily::SubscriptionSupport
                        | super::milestones::SemanticPhysicalClaimFamily::CompatibilitySemantics
                        | super::milestones::SemanticPhysicalClaimFamily::TieringPlacement
                        | super::milestones::SemanticPhysicalClaimFamily::ReplicationSemantics
                )
            })
            .count() as u64;
        self.physical_claim_count = report
            .rows()
            .iter()
            .filter(|row| {
                matches!(
                    row.claim_family(),
                    super::milestones::SemanticPhysicalClaimFamily::PhysicalSubstrate
                        | super::milestones::SemanticPhysicalClaimFamily::PhysicalBoundedness
                        | super::milestones::SemanticPhysicalClaimFamily::PhysicalIntegrity
                        | super::milestones::SemanticPhysicalClaimFamily::PhysicalRecoveryPhysics
                        | super::milestones::SemanticPhysicalClaimFamily::PhysicalIsolation
                        | super::milestones::SemanticPhysicalClaimFamily::PhysicalIo
                        | super::milestones::SemanticPhysicalClaimFamily::PhysicalOperationalSafety
                        | super::milestones::SemanticPhysicalClaimFamily::PhysicalSecurity
                )
            })
            .count() as u64;
        self
    }

    pub fn with_deferred_guarantee_map(mut self, map: &DeferredPhysicalGuaranteeMap) -> Self {
        self.unmapped_deferred_guarantee_count = map
            .rows()
            .iter()
            .filter(|row| row.row_id().as_str().trim().is_empty())
            .count() as u64;
        self
    }

    pub fn with_terminology_report(mut self, report: &TerminologyRiskReport) -> Self {
        self.overclaimed_physical_phrase_count = report
            .rows()
            .iter()
            .filter(|row| {
                matches!(
                    row.allowed_use(),
                    TerminologyAllowedUse::OverclaimedPhysicalPosture
                )
            })
            .count() as u64;
        self.unclassified_terminology_finding_count = 0;
        self.unique_evidence_ref_count = report
            .rows()
            .iter()
            .flat_map(|row| row.evidence_refs().iter().cloned())
            .collect::<BTreeSet<_>>()
            .len() as u64;
        self.scanned_file_count = report
            .rows()
            .iter()
            .map(|row| row.subject_path_or_symbol())
            .collect::<BTreeSet<_>>()
            .len() as u64;
        self
    }

    pub fn with_release_claim_report(mut self, report: &ReleaseClaimReport) -> Self {
        self.release_claim_scan_count = report.scanned_surface_count();
        self.public_claim_rejection_count = report.rejection_count();
        self.unqualified_release_claim_count = report.unqualified_release_claim_count();
        self
    }

    pub fn has_release_blocking_debt(&self) -> bool {
        self.missing_required_artifact_count != 0
            || self.schema_incompatible_artifact_count != 0
            || self.missing_complexity_contract_count != 0
            || self.duplicate_complexity_contract_count != 0
            || self.complexity_debt_count != 0
            || self.forbidden_platform_grade_claim_count != 0
            || self.missing_first_audit_row_count != 0
            || self.missing_milestone_status_row_count != 0
            || self.unmapped_deferred_guarantee_count != 0
            || self.unclassified_terminology_finding_count != 0
            || self.evidence_ref_reresolution_count != 0
            || self.s1_unmet_blocking_prerequisite_count != 0
            || self.overclaimed_physical_phrase_count != 0
            || self.unwaived_sequence_inconsistency_count != 0
            || self.unqualified_release_claim_count != 0
            || self.stale_evidence_rejection_count != 0
            || self.broad_scan_rejection_count != 0
    }
}
