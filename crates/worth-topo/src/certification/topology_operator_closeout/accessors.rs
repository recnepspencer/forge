use crate::certification::ReplayParityStatus;
use crate::topology_operators::{
    NamingEditContinuityMatrix, RejectedEditScopeReport, TopologyDerivedRegion,
    TopologyEditChangedScope, TopologyEditDigest, TopologyEditFamily, TopologyEditNamingOutcome,
    TopologyEditNamingScope, TopologyEditRejectionClass,
};

use super::report::{
    MilestoneThreeChangedScopeCoverageRow, MilestoneThreeDerivedRegionCoverageRow,
    MilestoneThreeDeterminismRuleKind, MilestoneThreeDeterminismRuleRow,
    MilestoneThreeEditBranchLocalParityRow, MilestoneThreeEditBreadthCounterRow,
    MilestoneThreeEditFalloutBreadthRow, MilestoneThreeEditFalloutClass,
    MilestoneThreeEditReplayParityRow, MilestoneThreeFailureLocalityRow,
    MilestoneThreeHostileOutcomeClass, MilestoneThreeHostileScenario,
    MilestoneThreeNamingContinuityMatrixRow, MilestoneThreeRejectedEditScopeReportRow,
    MilestoneThreeTopologyEditDigestRow, MilestoneThreeValidatorFamily,
    MilestoneThreeValidatorFamilyCoverageRow,
};

impl MilestoneThreeTopologyEditDigestRow {
    pub fn scenario(&self) -> MilestoneThreeHostileScenario {
        self.scenario
    }

    pub fn topology_edit_digest(&self) -> &TopologyEditDigest {
        &self.topology_edit_digest
    }

    pub fn row_digest(&self) -> &str {
        self.row_digest.as_str()
    }
}

impl MilestoneThreeNamingContinuityMatrixRow {
    pub fn scenario(&self) -> MilestoneThreeHostileScenario {
        self.scenario
    }

    pub fn naming_edit_continuity_matrix(&self) -> &NamingEditContinuityMatrix {
        &self.naming_edit_continuity_matrix
    }

    pub fn continuity_outcome_class(&self) -> TopologyEditNamingOutcome {
        self.continuity_outcome_class
    }

    pub fn continuity_rejection_class(&self) -> Option<TopologyEditRejectionClass> {
        self.continuity_rejection_class
    }

    pub fn row_digest(&self) -> &str {
        self.row_digest.as_str()
    }
}

impl MilestoneThreeRejectedEditScopeReportRow {
    pub fn scenario(&self) -> MilestoneThreeHostileScenario {
        self.scenario
    }

    pub fn rejection_class(&self) -> TopologyEditRejectionClass {
        self.rejection_class
    }

    pub fn rejected_edit_scope_report(&self) -> &RejectedEditScopeReport {
        &self.rejected_edit_scope_report
    }

    pub fn row_digest(&self) -> &str {
        self.row_digest.as_str()
    }
}

impl MilestoneThreeEditReplayParityRow {
    pub fn scenario(&self) -> MilestoneThreeHostileScenario {
        self.scenario
    }

    pub fn replay_checked(&self) -> bool {
        self.replay_checked
    }

    pub fn parity_status(&self) -> ReplayParityStatus {
        self.parity_status
    }

    pub fn mismatch_count(&self) -> usize {
        self.mismatch_count
    }

    pub fn step_count(&self) -> usize {
        self.step_count
    }

    pub fn replay_step_count(&self) -> usize {
        self.replay_step_count
    }

    pub fn row_digest(&self) -> &str {
        self.row_digest.as_str()
    }
}

impl MilestoneThreeEditBranchLocalParityRow {
    pub fn scenario(&self) -> Option<MilestoneThreeHostileScenario> {
        self.scenario
    }

    pub fn branch_label(&self) -> &str {
        self.branch_label.as_str()
    }

    pub fn branch_id(&self) -> &str {
        self.branch_id.as_str()
    }

    pub fn mutation_origin(&self) -> &str {
        self.mutation_origin.as_str()
    }

    pub fn outcome_class(&self) -> MilestoneThreeHostileOutcomeClass {
        self.outcome_class
    }

    pub fn rejection_class(&self) -> Option<TopologyEditRejectionClass> {
        self.rejection_class
    }

    pub fn edit_families(&self) -> &[TopologyEditFamily] {
        &self.edit_families
    }

    pub fn topology_edit_digest(&self) -> &TopologyEditDigest {
        &self.topology_edit_digest
    }

    pub fn naming_edit_continuity_matrix(&self) -> &NamingEditContinuityMatrix {
        &self.naming_edit_continuity_matrix
    }

    pub fn branch_head_diverged_from_main(&self) -> bool {
        self.branch_head_diverged_from_main
    }

    pub fn branch_head_unchanged_after_rejection(&self) -> bool {
        self.branch_head_unchanged_after_rejection
    }

    pub fn branch_truth_digest(&self) -> Option<&crate::certification::DeterministicDigest> {
        self.branch_truth_digest.as_ref()
    }

    pub fn row_digest(&self) -> &str {
        self.row_digest.as_str()
    }
}

impl MilestoneThreeChangedScopeCoverageRow {
    pub fn changed_scope(&self) -> TopologyEditChangedScope {
        self.changed_scope
    }

    pub fn scenario_count(&self) -> usize {
        self.scenario_count
    }

    pub fn scenarios(&self) -> &[MilestoneThreeHostileScenario] {
        &self.scenarios
    }

    pub fn row_digest(&self) -> &str {
        self.row_digest.as_str()
    }
}

impl MilestoneThreeValidatorFamilyCoverageRow {
    pub fn scenario(&self) -> MilestoneThreeHostileScenario {
        self.scenario
    }

    pub fn validator_family(&self) -> MilestoneThreeValidatorFamily {
        self.validator_family
    }

    pub fn validator_names(&self) -> &[String] {
        &self.validator_names
    }

    pub fn edit_family_count(&self) -> usize {
        self.edit_family_count
    }

    pub fn changed_scope_count(&self) -> usize {
        self.changed_scope_count
    }

    pub fn naming_scope_count(&self) -> usize {
        self.naming_scope_count
    }

    pub fn derived_region_count(&self) -> usize {
        self.derived_region_count
    }

    pub fn derived_validation_row_count(&self) -> usize {
        self.derived_validation_row_count
    }

    pub fn localized_rejection_boundary(&self) -> bool {
        self.localized_rejection_boundary
    }

    pub fn row_digest(&self) -> &str {
        self.row_digest.as_str()
    }
}

impl MilestoneThreeDerivedRegionCoverageRow {
    pub fn derived_region(&self) -> TopologyDerivedRegion {
        self.derived_region
    }

    pub fn scenario_count(&self) -> usize {
        self.scenario_count
    }

    pub fn scenarios(&self) -> &[MilestoneThreeHostileScenario] {
        &self.scenarios
    }

    pub fn row_digest(&self) -> &str {
        self.row_digest.as_str()
    }
}

impl MilestoneThreeDeterminismRuleRow {
    pub fn scenario(&self) -> MilestoneThreeHostileScenario {
        self.scenario
    }

    pub fn rule_kind(&self) -> MilestoneThreeDeterminismRuleKind {
        self.rule_kind
    }

    pub fn evidence_count(&self) -> usize {
        self.evidence_count
    }

    pub fn replay_verified(&self) -> bool {
        self.replay_verified
    }

    pub fn diagnostic_classification_stable(&self) -> bool {
        self.diagnostic_classification_stable
    }

    pub fn tie_break_evidence_stable(&self) -> bool {
        self.tie_break_evidence_stable
    }

    pub fn row_digest(&self) -> &str {
        self.row_digest.as_str()
    }
}

impl MilestoneThreeEditBreadthCounterRow {
    pub fn scenario(&self) -> MilestoneThreeHostileScenario {
        self.scenario
    }

    pub fn contract_count(&self) -> usize {
        self.contract_count
    }

    pub fn family_count(&self) -> usize {
        self.family_count
    }

    pub fn changed_scope_count(&self) -> usize {
        self.changed_scope_count
    }

    pub fn naming_scope_count(&self) -> usize {
        self.naming_scope_count
    }

    pub fn derived_region_count(&self) -> usize {
        self.derived_region_count
    }

    pub fn replay_step_count(&self) -> usize {
        self.replay_step_count
    }

    pub fn replay_checked(&self) -> bool {
        self.replay_checked
    }

    pub fn row_digest(&self) -> &str {
        self.row_digest.as_str()
    }
}

impl MilestoneThreeEditFalloutBreadthRow {
    pub fn scenario(&self) -> MilestoneThreeHostileScenario {
        self.scenario
    }

    pub fn fallout_class(&self) -> MilestoneThreeEditFalloutClass {
        self.fallout_class
    }

    pub fn declared_derived_region_count(&self) -> usize {
        self.declared_derived_region_count
    }

    pub fn derived_validation_row_count(&self) -> usize {
        self.derived_validation_row_count
    }

    pub fn fallback_count(&self) -> usize {
        self.fallback_count
    }

    pub fn locality_claim_mismatch(&self) -> bool {
        self.locality_claim_mismatch
    }

    pub fn row_digest(&self) -> &str {
        self.row_digest.as_str()
    }
}

impl MilestoneThreeFailureLocalityRow {
    pub fn scenario(&self) -> MilestoneThreeHostileScenario {
        self.scenario
    }

    pub fn rejection_class(&self) -> TopologyEditRejectionClass {
        self.rejection_class
    }

    pub fn scope_row_count(&self) -> usize {
        self.scope_row_count
    }

    pub fn families(&self) -> &[TopologyEditFamily] {
        &self.families
    }

    pub fn changed_scopes(&self) -> &[TopologyEditChangedScope] {
        &self.changed_scopes
    }

    pub fn naming_scopes(&self) -> &[TopologyEditNamingScope] {
        &self.naming_scopes
    }

    pub fn derived_regions(&self) -> &[TopologyDerivedRegion] {
        &self.derived_regions
    }

    pub fn row_digest(&self) -> &str {
        self.row_digest.as_str()
    }
}
