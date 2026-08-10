use super::super::artifacts::S0ArtifactValidationCostSurface;
use super::super::manifest::{S0AuditInputManifest, S0InputManifestDelta};
use super::super::milestones::{
    MilestonePhysicalStatusRow, MilestoneSequenceInconsistency, RoadmapSequenceStatusMatrix,
    SemanticPhysicalClaimFamily,
};
use super::super::{
    DeferredPhysicalGuaranteeMap, ReleaseClaimReport, SemanticPhysicalClaimReport,
    TerminologyAllowedUse, TerminologyRiskReport,
};
use super::counter_snapshot::S0CounterSnapshot;
use std::collections::BTreeSet;

impl S0CounterSnapshot {
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
                *inconsistency == MilestoneSequenceInconsistency::SpecCloseoutStatusMismatch
            })
            .count() as u64;
        self.closed_with_unclosed_prerequisite_count = matrix
            .inconsistencies()
            .iter()
            .filter(|(_, inconsistency)| {
                *inconsistency == MilestoneSequenceInconsistency::ClosedWithUnclosedPrerequisite
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
                    SemanticPhysicalClaimFamily::SemanticAuthority
                        | SemanticPhysicalClaimFamily::RecoverySemantics
                        | SemanticPhysicalClaimFamily::RetentionSemantics
                        | SemanticPhysicalClaimFamily::SubscriptionSupport
                        | SemanticPhysicalClaimFamily::CompatibilitySemantics
                        | SemanticPhysicalClaimFamily::TieringPlacement
                        | SemanticPhysicalClaimFamily::ReplicationSemantics
                )
            })
            .count() as u64;
        self.physical_claim_count = rows
            .iter()
            .flat_map(MilestonePhysicalStatusRow::claim_families)
            .filter(|family| {
                matches!(
                    family,
                    SemanticPhysicalClaimFamily::PhysicalSubstrate
                        | SemanticPhysicalClaimFamily::PhysicalBoundedness
                        | SemanticPhysicalClaimFamily::PhysicalIntegrity
                        | SemanticPhysicalClaimFamily::PhysicalRecoveryPhysics
                        | SemanticPhysicalClaimFamily::PhysicalIsolation
                        | SemanticPhysicalClaimFamily::PhysicalIo
                        | SemanticPhysicalClaimFamily::PhysicalOperationalSafety
                        | SemanticPhysicalClaimFamily::PhysicalSecurity
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
                    SemanticPhysicalClaimFamily::SemanticAuthority
                        | SemanticPhysicalClaimFamily::RecoverySemantics
                        | SemanticPhysicalClaimFamily::RetentionSemantics
                        | SemanticPhysicalClaimFamily::SubscriptionSupport
                        | SemanticPhysicalClaimFamily::CompatibilitySemantics
                        | SemanticPhysicalClaimFamily::TieringPlacement
                        | SemanticPhysicalClaimFamily::ReplicationSemantics
                )
            })
            .count() as u64;
        self.physical_claim_count = report
            .rows()
            .iter()
            .filter(|row| {
                matches!(
                    row.claim_family(),
                    SemanticPhysicalClaimFamily::PhysicalSubstrate
                        | SemanticPhysicalClaimFamily::PhysicalBoundedness
                        | SemanticPhysicalClaimFamily::PhysicalIntegrity
                        | SemanticPhysicalClaimFamily::PhysicalRecoveryPhysics
                        | SemanticPhysicalClaimFamily::PhysicalIsolation
                        | SemanticPhysicalClaimFamily::PhysicalIo
                        | SemanticPhysicalClaimFamily::PhysicalOperationalSafety
                        | SemanticPhysicalClaimFamily::PhysicalSecurity
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
