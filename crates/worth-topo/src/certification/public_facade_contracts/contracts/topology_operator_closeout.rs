use forge_relational::facade::runtime::RelationalRuntime;
use topology::facade::{
    certify_milestone_three_closeout, MilestoneThreeChangedScopeCoverageRow,
    MilestoneThreeDerivedRegionCoverageRow, MilestoneThreeDeterminismRuleRow,
    MilestoneThreeEditBranchLocalParityRow, MilestoneThreeEditBreadthCounterRow,
    MilestoneThreeEditFalloutBreadthRow, MilestoneThreeEditedTopologyQueryTraversalRow,
    MilestoneThreeFailureLocalityRow, MilestoneThreeHostileCertificationCategoryRow,
    MilestoneThreeHostileSuiteReport, MilestoneThreeNamingContinuityMatrixRow,
    MilestoneThreePrimitiveFamilyClosureRow, MilestoneThreeRejectedEditScopeReportRow,
    MilestoneThreeReturnGateBlockerRow, MilestoneThreeSideQuestBlockerRow,
    MilestoneThreeSideQuestCloseoutReport, MilestoneThreeSideQuestContractRow,
    MilestoneThreeSplitCollapseChurnWitness, MilestoneThreeTopologyEditDigestRow,
    MilestoneThreeValidatorFamilyCoverageRow, TopologyCertificationError,
};

fn _milestone_three_closeout_function_contract() {
    let _: fn(
        fn() -> RelationalRuntime,
        &str,
    ) -> Result<MilestoneThreeHostileSuiteReport, TopologyCertificationError> =
        certify_milestone_three_closeout::<fn() -> RelationalRuntime>;
}

fn _milestone_three_side_quest_report_contract(report: &MilestoneThreeHostileSuiteReport) {
    let _: &MilestoneThreeSideQuestCloseoutReport = &report.side_quest_closeout_report;
    let _: &[MilestoneThreeSideQuestContractRow] =
        report.side_quest_closeout_report.contract_rows.as_slice();
    let _: &[MilestoneThreeSideQuestBlockerRow] =
        report.side_quest_closeout_report.blocker_rows.as_slice();
    let _: usize = report.side_quest_closeout_report.domain_read_request_count;
    let _: usize = report.side_quest_closeout_report.domain_read_parity_count;
    let _: bool = report.side_quest_closeout_report.phase_three_ready;
    let _: bool = report.side_quest_gate_ready;
    let _: &[MilestoneThreeReturnGateBlockerRow] =
        report.milestone_three_return_gate_blocker_rows.as_slice();
    let _: bool = report.milestone_three_return_gate_ready;
    let _: &[MilestoneThreeHostileCertificationCategoryRow] =
        report.hostile_certification_category_rows.as_slice();
    let _: &[MilestoneThreePrimitiveFamilyClosureRow] =
        report.primitive_family_closure_rows.as_slice();
    let _: &[MilestoneThreeTopologyEditDigestRow] = report.topology_edit_digest_rows.as_slice();
    let _: &[MilestoneThreeNamingContinuityMatrixRow] =
        report.naming_edit_continuity_matrix_rows.as_slice();
    let _: &[MilestoneThreeRejectedEditScopeReportRow] =
        report.rejected_edit_scope_report_rows.as_slice();
    let _: &[topology::facade::MilestoneThreeEditReplayParityRow] =
        report.edit_replay_parity_rows.as_slice();
    let _: &[MilestoneThreeEditBranchLocalParityRow] =
        report.edit_branch_local_parity_rows.as_slice();
    let _: &[MilestoneThreeEditedTopologyQueryTraversalRow] =
        report.edited_query_traversal_rows.as_slice();
    let _: &[MilestoneThreeChangedScopeCoverageRow] = report.changed_scope_coverage_rows.as_slice();
    let _: &[MilestoneThreeDerivedRegionCoverageRow] =
        report.derived_region_coverage_rows.as_slice();
    let _: &[MilestoneThreeDeterminismRuleRow] = report.determinism_rule_rows.as_slice();
    let _: &[MilestoneThreeEditBreadthCounterRow] = report.edit_breadth_counter_rows.as_slice();
    let _: &[MilestoneThreeEditFalloutBreadthRow] = report.edit_fallout_breadth_rows.as_slice();
    let _: &[MilestoneThreeFailureLocalityRow] = report.failure_locality_rows.as_slice();
    let _: &[MilestoneThreeValidatorFamilyCoverageRow] =
        report.validator_family_coverage_rows.as_slice();
    let _: Option<&MilestoneThreeSplitCollapseChurnWitness> = report
        .scenario_reports
        .iter()
        .find_map(|scenario| scenario.split_collapse_churn_witness.as_ref());
}

fn _milestone_three_side_quest_row_contracts(
    contract_row: &MilestoneThreeSideQuestContractRow,
    blocker_row: &MilestoneThreeSideQuestBlockerRow,
) {
    let _: &str = contract_row.contract_name.as_str();
    let _: &str = contract_row.status.as_str();
    let _: &str = contract_row.reason.as_str();
    let _: &str = contract_row.row_digest.as_str();
    let _: &str = blocker_row.blocker_name.as_str();
    let _: &str = blocker_row.status.as_str();
    let _: &str = blocker_row.reason.as_str();
    let _: &str = blocker_row.row_digest.as_str();
}

fn _milestone_three_return_gate_blocker_row_contract(
    blocker_row: &MilestoneThreeReturnGateBlockerRow,
) {
    let _: &str = blocker_row.blocker_name.as_str();
    let _: &str = blocker_row.reason.as_str();
    let _: &str = blocker_row.row_digest.as_str();
}

fn _milestone_three_direct_acceptance_row_contracts(
    category_row: &MilestoneThreeHostileCertificationCategoryRow,
    digest_row: &MilestoneThreeTopologyEditDigestRow,
    naming_row: &MilestoneThreeNamingContinuityMatrixRow,
    rejection_row: &MilestoneThreeRejectedEditScopeReportRow,
    replay_row: &topology::facade::MilestoneThreeEditReplayParityRow,
    branch_row: &MilestoneThreeEditBranchLocalParityRow,
    traversal_row: &MilestoneThreeEditedTopologyQueryTraversalRow,
    primitive_row: &MilestoneThreePrimitiveFamilyClosureRow,
) {
    let _: topology::facade::MilestoneThreeHostileCertificationCategory = category_row.category();
    let _: topology::facade::MilestoneThreeHostileCertificationStatus = category_row.status();
    let _: usize = category_row.scenario_count();
    let _: usize = category_row.evidence_count();
    let _: usize = category_row.replay_verified_count();
    let _: usize = category_row.diagnostic_locality_count();
    let _: &[String] = category_row.evidence_labels();
    let _: &[String] = category_row.gap_labels();
    let _: &str = category_row.row_digest();
    let _: topology::facade::MilestoneThreeHostileScenario = digest_row.scenario();
    let _: &topology::facade::TopologyEditDigest = digest_row.topology_edit_digest();
    let _: &str = digest_row.row_digest();
    let _: &topology::facade::NamingEditContinuityMatrix =
        naming_row.naming_edit_continuity_matrix();
    let _: topology::facade::TopologyEditNamingOutcome = naming_row.continuity_outcome_class();
    let _: Option<topology::facade::TopologyEditRejectionClass> =
        naming_row.continuity_rejection_class();
    let _: &topology::facade::RejectedEditScopeReport = rejection_row.rejected_edit_scope_report();
    let _: topology::facade::TopologyEditRejectionClass = rejection_row.rejection_class();
    let _: bool = replay_row.replay_checked();
    let _: topology::facade::ReplayParityStatus = replay_row.parity_status();
    let _: usize = replay_row.mismatch_count();
    let _: &str = replay_row.row_digest();
    let _: Option<topology::facade::MilestoneThreeHostileScenario> = branch_row.scenario();
    let _: &str = branch_row.branch_label();
    let _: &str = branch_row.branch_id();
    let _: &str = branch_row.mutation_origin();
    let _: topology::facade::MilestoneThreeHostileOutcomeClass = branch_row.outcome_class();
    let _: Option<topology::facade::TopologyEditRejectionClass> = branch_row.rejection_class();
    let _: &[topology::facade::TopologyEditFamily] = branch_row.edit_families();
    let _: &topology::facade::TopologyEditDigest = branch_row.topology_edit_digest();
    let _: &topology::facade::NamingEditContinuityMatrix =
        branch_row.naming_edit_continuity_matrix();
    let _: bool = branch_row.branch_head_diverged_from_main();
    let _: bool = branch_row.branch_head_unchanged_after_rejection();
    let _: Option<&topology::facade::DeterministicDigest> = branch_row.branch_truth_digest();
    let _: &str = branch_row.row_digest();
    let _: topology::facade::MilestoneThreeHostileScenario = traversal_row.scenario();
    let _: topology::facade::MilestoneThreeEditedTopologyQueryTraversalView = traversal_row.view();
    let _: &str = traversal_row.left_view_digest();
    let _: &str = traversal_row.replay_view_digest();
    let _: bool = traversal_row.parity_verified();
    let _: usize = traversal_row.request_count();
    let _: usize = traversal_row.relationship_proof_admission_count();
    let _: usize = traversal_row.traversal_count();
    let _: &str = traversal_row.row_digest();
    let _: &str = primitive_row.primitive_family();
    let _: &schema::facade::topology_authoring::MilestoneOnePrimitiveCase =
        primitive_row.primitive();
    let _: &[topology::facade::TopologyEditFamily] = primitive_row.edit_families();
    let _: &topology::facade::TopologyEditDigest = primitive_row.topology_edit_digest();
    let _: bool = primitive_row.replay_verified();
    let _: &topology::facade::DeterministicDigest =
        primitive_row.final_materialized_topology_digest();
    let _: &topology::facade::DeterministicDigest =
        primitive_row.replay_final_materialized_topology_digest();
    let _: usize = primitive_row.derived_validation_row_count();
    let _: &str = primitive_row.row_digest();
}

fn _milestone_three_aggregate_acceptance_row_contracts(
    scope_row: &MilestoneThreeChangedScopeCoverageRow,
    region_row: &MilestoneThreeDerivedRegionCoverageRow,
    determinism_row: &MilestoneThreeDeterminismRuleRow,
    breadth_row: &MilestoneThreeEditBreadthCounterRow,
    fallout_row: &MilestoneThreeEditFalloutBreadthRow,
    locality_row: &MilestoneThreeFailureLocalityRow,
    validator_row: &MilestoneThreeValidatorFamilyCoverageRow,
) {
    let _: topology::facade::TopologyEditChangedScope = scope_row.changed_scope();
    let _: usize = scope_row.scenario_count();
    let _: &[topology::facade::MilestoneThreeHostileScenario] = scope_row.scenarios();
    let _: &str = scope_row.row_digest();
    let _: topology::facade::TopologyDerivedRegion = region_row.derived_region();
    let _: usize = region_row.scenario_count();
    let _: &[topology::facade::MilestoneThreeHostileScenario] = region_row.scenarios();
    let _: topology::facade::MilestoneThreeHostileScenario = determinism_row.scenario();
    let _: topology::facade::MilestoneThreeDeterminismRuleKind = determinism_row.rule_kind();
    let _: usize = determinism_row.evidence_count();
    let _: bool = determinism_row.replay_verified();
    let _: bool = determinism_row.diagnostic_classification_stable();
    let _: bool = determinism_row.tie_break_evidence_stable();
    let _: &str = determinism_row.row_digest();
    let _: topology::facade::MilestoneThreeHostileScenario = breadth_row.scenario();
    let _: usize = breadth_row.contract_count();
    let _: usize = breadth_row.changed_scope_count();
    let _: bool = breadth_row.replay_checked();
    let _: topology::facade::MilestoneThreeEditFalloutClass = fallout_row.fallout_class();
    let _: topology::facade::MilestoneThreeHostileScenario = fallout_row.scenario();
    let _: usize = fallout_row.declared_derived_region_count();
    let _: usize = fallout_row.derived_validation_row_count();
    let _: usize = fallout_row.fallback_count();
    let _: bool = fallout_row.locality_claim_mismatch();
    let _: &str = fallout_row.row_digest();
    let _: topology::facade::TopologyEditRejectionClass = locality_row.rejection_class();
    let _: &[topology::facade::TopologyEditFamily] = locality_row.families();
    let _: &[topology::facade::TopologyEditChangedScope] = locality_row.changed_scopes();
    let _: &[topology::facade::TopologyDerivedRegion] = locality_row.derived_regions();
    let _: topology::facade::MilestoneThreeHostileScenario = validator_row.scenario();
    let _: topology::facade::MilestoneThreeValidatorFamily = validator_row.validator_family();
    let _: &[String] = validator_row.validator_names();
    let _: usize = validator_row.edit_family_count();
    let _: usize = validator_row.changed_scope_count();
    let _: usize = validator_row.naming_scope_count();
    let _: usize = validator_row.derived_region_count();
    let _: usize = validator_row.derived_validation_row_count();
    let _: bool = validator_row.localized_rejection_boundary();
    let _: &str = validator_row.row_digest();
}

fn _milestone_three_split_collapse_witness_contract(
    witness: &MilestoneThreeSplitCollapseChurnWitness,
) {
    let _: &str = witness.original_wire_identity.as_str();
    let _: &str = witness.split_wire_identity.as_str();
    let _: &str = witness.collapse_wire_identity.as_str();
    let _: &[String] = witness.moved_half_edge_identities.as_slice();
    let _: &[String] = witness.retained_half_edge_identities.as_slice();
    let _: usize = witness.split_step_wire_count;
    let _: usize = witness.final_wire_count;
}

#[test]
fn milestone_three_side_quest_closeout_is_public_report_surface() {}
