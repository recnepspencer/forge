use super::super::evidence::S0ArtifactValidationReport;
use super::complexity_contract::S0ComplexityContractReport;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct S0CounterSnapshot {
    pub(super) required_artifact_count: u64,
    pub(super) missing_required_artifact_count: u64,
    pub(super) schema_incompatible_artifact_count: u64,
    pub(super) complexity_contract_count: u64,
    pub(super) missing_complexity_contract_count: u64,
    pub(super) duplicate_complexity_contract_count: u64,
    pub(super) complexity_debt_count: u64,
    pub(super) forbidden_platform_grade_claim_count: u64,
    pub(super) missing_first_audit_row_count: u64,
    pub(super) roadmap_sequence_edge_count: u64,
    pub(super) sequence_inconsistency_count: u64,
    pub(super) spec_closeout_status_mismatch_count: u64,
    pub(super) closed_with_unclosed_prerequisite_count: u64,
    pub(super) milestone_status_row_count: u64,
    pub(super) missing_milestone_status_row_count: u64,
    pub(super) unmapped_deferred_guarantee_count: u64,
    pub(super) semantic_claim_count: u64,
    pub(super) physical_claim_count: u64,
    pub(super) unclassified_terminology_finding_count: u64,
    pub(super) evidence_ref_reresolution_count: u64,
    pub(super) s1_unmet_blocking_prerequisite_count: u64,
    pub(super) overclaimed_physical_phrase_count: u64,
    pub(super) unwaived_sequence_inconsistency_count: u64,
    pub(super) unqualified_release_claim_count: u64,
    pub(super) stale_evidence_rejection_count: u64,
    pub(super) broad_scan_rejection_count: u64,
    pub(super) release_claim_scan_count: u64,
    pub(super) public_claim_rejection_count: u64,
    pub(super) input_manifest_file_count: u64,
    pub(super) input_manifest_byte_count: u64,
    pub(super) input_manifest_reused_file_count: u64,
    pub(super) input_manifest_rescanned_file_count: u64,
    pub(super) requested_scan_scope_count: u64,
    pub(super) admitted_scan_scope_count: u64,
    pub(super) rejected_scan_scope_count: u64,
    pub(super) scanned_file_count: u64,
    pub(super) scanned_byte_count: u64,
    pub(super) unique_evidence_ref_count: u64,
    pub(super) digest_row_byte_count: u64,
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
}
