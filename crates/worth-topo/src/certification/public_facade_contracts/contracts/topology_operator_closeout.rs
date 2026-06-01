use forge_relational::facade::runtime::RelationalRuntime;
use topology::facade::{
    certify_milestone_three_closeout, MilestoneThreeChangedScopeCoverageRow,
    MilestoneThreeDerivedFallbackPolicyDenialRow, MilestoneThreeDerivedRegionCoverageRow,
    MilestoneThreeDerivedReuseLegalityRow, MilestoneThreeDerivedWorkBreadthRow,
    MilestoneThreeDeterminismRuleRow, MilestoneThreeFailureLocalityRow,
    MilestoneThreeHostileCertificationCategoryRow, MilestoneThreeHostileFamilyCoverageRow,
    MilestoneThreeHostileNamingDistributionRow, MilestoneThreeHostileRejectionDistributionRow,
    MilestoneThreeHostileSuiteReport, MilestoneThreeMutationBranchLocalParityRow,
    MilestoneThreeMutationBreadthCounterRow, MilestoneThreeMutationFalloutBreadthRow,
    MilestoneThreeMutationTopologyQueryTraversalRow, MilestoneThreeNamingContinuityBreadthRow,
    MilestoneThreeNamingContinuityMatrixRow, MilestoneThreeOperatorFamilyClosureRow,
    MilestoneThreePrimitiveFamilyClosureRow, MilestoneThreeRejectedMutationScopeReportRow,
    MilestoneThreeReplayBranchBreadthRow, MilestoneThreeReturnGateBlockerRow,
    MilestoneThreeScalePressureRow, MilestoneThreeSideQuestBlockerRow,
    MilestoneThreeSideQuestCloseoutReport, MilestoneThreeSideQuestContractRow,
    MilestoneThreeSplitCollapseChurnWitness, MilestoneThreeTopologyMutationDigestRow,
    MilestoneThreeValidationBreadthRow, MilestoneThreeValidatorFamilyCoverageRow,
    TopologyCertificationError,
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
    let _: &[MilestoneThreeHostileFamilyCoverageRow] = report.family_coverage_rows.as_slice();
    let _: &[MilestoneThreeHostileRejectionDistributionRow] =
        report.rejection_distribution_rows.as_slice();
    let _: &[MilestoneThreeHostileNamingDistributionRow] =
        report.naming_distribution_rows.as_slice();
    let _: &[MilestoneThreeOperatorFamilyClosureRow] =
        report.operator_family_closure_rows.as_slice();
    let _: &[MilestoneThreePrimitiveFamilyClosureRow] =
        report.primitive_family_closure_rows.as_slice();
    let _: &[MilestoneThreeScalePressureRow] = report.scale_pressure_rows.as_slice();
    let _: &[MilestoneThreeTopologyMutationDigestRow] =
        report.topology_mutation_digest_rows.as_slice();
    let _: &[MilestoneThreeNamingContinuityMatrixRow] =
        report.naming_mutation_continuity_matrix_rows.as_slice();
    let _: &[MilestoneThreeNamingContinuityBreadthRow] =
        report.naming_continuity_breadth_rows.as_slice();
    let _: &[MilestoneThreeRejectedMutationScopeReportRow] =
        report.rejected_mutation_scope_report_rows.as_slice();
    let _: &[topology::facade::MilestoneThreeMutationReplayParityRow] =
        report.mutation_replay_parity_rows.as_slice();
    let _: &[MilestoneThreeMutationBranchLocalParityRow] =
        report.mutation_branch_local_parity_rows.as_slice();
    let _: &[MilestoneThreeReplayBranchBreadthRow] = report.replay_branch_breadth_rows.as_slice();
    let _: &[MilestoneThreeMutationTopologyQueryTraversalRow] =
        report.mutation_query_traversal_rows.as_slice();
    let _: &[MilestoneThreeChangedScopeCoverageRow] = report.changed_scope_coverage_rows.as_slice();
    let _: &[MilestoneThreeDerivedRegionCoverageRow] =
        report.derived_region_coverage_rows.as_slice();
    let _: &[MilestoneThreeDeterminismRuleRow] = report.determinism_rule_rows.as_slice();
    let _: &[MilestoneThreeMutationBreadthCounterRow] =
        report.mutation_breadth_counter_rows.as_slice();
    let _: &[MilestoneThreeMutationFalloutBreadthRow] =
        report.mutation_fallout_breadth_rows.as_slice();
    let _: &[MilestoneThreeDerivedFallbackPolicyDenialRow] =
        report.derived_fallback_policy_denial_rows.as_slice();
    let _: &[MilestoneThreeDerivedReuseLegalityRow] = report.derived_reuse_legality_rows.as_slice();
    let _: &[MilestoneThreeDerivedWorkBreadthRow] = report.derived_work_breadth_rows.as_slice();
    let _: &[MilestoneThreeFailureLocalityRow] = report.failure_locality_rows.as_slice();
    let _: &[MilestoneThreeValidatorFamilyCoverageRow] =
        report.validator_family_coverage_rows.as_slice();
    let _: &[MilestoneThreeValidationBreadthRow] = report.validation_breadth_rows.as_slice();
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
    digest_row: &MilestoneThreeTopologyMutationDigestRow,
    naming_row: &MilestoneThreeNamingContinuityMatrixRow,
    naming_breadth_row: &MilestoneThreeNamingContinuityBreadthRow,
    rejection_row: &MilestoneThreeRejectedMutationScopeReportRow,
    replay_row: &topology::facade::MilestoneThreeMutationReplayParityRow,
    branch_row: &MilestoneThreeMutationBranchLocalParityRow,
    replay_branch_breadth_row: &MilestoneThreeReplayBranchBreadthRow,
    traversal_row: &MilestoneThreeMutationTopologyQueryTraversalRow,
    operator_family_row: &MilestoneThreeOperatorFamilyClosureRow,
    primitive_row: &MilestoneThreePrimitiveFamilyClosureRow,
    scale_row: &MilestoneThreeScalePressureRow,
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
    let _: &topology::facade::TopologyMutationDigest = digest_row.topology_mutation_digest();
    let _: usize = digest_row.topology_mutation_digest().fallback_policy_count;
    let _: usize = digest_row
        .topology_mutation_digest()
        .fallback_rejection_policy_count;
    let _: &str = digest_row.row_digest();
    let _: &topology::facade::NamingMutationContinuityMatrix =
        naming_row.naming_mutation_continuity_matrix();
    let _: topology::facade::TopologyMutationNamingOutcome = naming_row.continuity_outcome_class();
    let _: Option<topology::facade::TopologyMutationRejectionClass> =
        naming_row.continuity_rejection_class();
    let _: topology::facade::MilestoneThreeHostileScenario = naming_breadth_row.scenario();
    let _: usize = naming_breadth_row.continuity_row_count();
    let _: usize = naming_breadth_row.preserved_count();
    let _: usize = naming_breadth_row.ambiguous_count();
    let _: usize = naming_breadth_row.rejected_count();
    let _: usize = naming_breadth_row.naming_scope_count();
    let _: usize = naming_breadth_row.replay_step_count();
    let _: bool = naming_breadth_row.replay_checked();
    let _: topology::facade::TopologyMutationNamingOutcome = naming_breadth_row.outcome_class();
    let _: &str = naming_breadth_row.row_digest();
    let _: &topology::facade::RejectedMutationScopeReport =
        rejection_row.rejected_mutation_scope_report();
    let _: topology::facade::TopologyMutationRejectionClass = rejection_row.rejection_class();
    let _: bool = replay_row.replay_checked();
    let _: topology::facade::ReplayParityStatus = replay_row.parity_status();
    let _: usize = replay_row.mismatch_count();
    let _: &str = replay_row.row_digest();
    let _: Option<topology::facade::MilestoneThreeHostileScenario> = branch_row.scenario();
    let _: &str = branch_row.branch_label();
    let _: &str = branch_row.branch_id();
    let _: &str = branch_row.mutation_origin();
    let _: topology::facade::MilestoneThreeHostileOutcomeClass = branch_row.outcome_class();
    let _: Option<topology::facade::TopologyMutationRejectionClass> = branch_row.rejection_class();
    let _: &[topology::facade::TopologyMutationFamily] = branch_row.mutation_families();
    let _: &topology::facade::TopologyMutationDigest = branch_row.topology_mutation_digest();
    let _: &topology::facade::NamingMutationContinuityMatrix =
        branch_row.naming_mutation_continuity_matrix();
    let _: bool = branch_row.branch_head_diverged_from_main();
    let _: bool = branch_row.branch_head_unchanged_after_rejection();
    let _: Option<&topology::facade::DeterministicDigest> = branch_row.branch_truth_digest();
    let _: &str = branch_row.row_digest();
    let _: usize = replay_branch_breadth_row.required_scenario_count();
    let _: usize = replay_branch_breadth_row.replay_checked_scenario_count();
    let _: usize = replay_branch_breadth_row.replay_step_count();
    let _: usize = replay_branch_breadth_row.replay_comparison_step_count();
    let _: usize = replay_branch_breadth_row.replay_mismatch_count();
    let _: usize = replay_branch_breadth_row.branch_local_row_count();
    let _: usize = replay_branch_breadth_row.accepted_branch_local_row_count();
    let _: usize = replay_branch_breadth_row.required_accepted_branch_local_count();
    let _: usize = replay_branch_breadth_row.rejected_branch_local_row_count();
    let _: usize = replay_branch_breadth_row.required_rejected_branch_local_count();
    let _: usize = replay_branch_breadth_row.branch_truth_digest_count();
    let _: usize = replay_branch_breadth_row.unchanged_rejected_branch_count();
    let _: &str = replay_branch_breadth_row.row_digest();
    let _: topology::facade::MilestoneThreeHostileScenario = traversal_row.scenario();
    let _: topology::facade::MilestoneThreeMutationTopologyQueryTraversalView =
        traversal_row.view();
    let _: &str = traversal_row.left_view_digest();
    let _: &str = traversal_row.replay_view_digest();
    let _: bool = traversal_row.parity_verified();
    let _: usize = traversal_row.request_count();
    let _: usize = traversal_row.relationship_proof_admission_count();
    let _: usize = traversal_row.traversal_count();
    let _: &str = traversal_row.row_digest();
    let _: topology::facade::TopologyMutationFamily = operator_family_row.family();
    let _: &[String] = operator_family_row.admitted_lane_labels();
    let _: &[String] = operator_family_row.legal_evidence_labels();
    let _: &[String] = operator_family_row.hostile_evidence_labels();
    let _: &[String] = operator_family_row.replay_evidence_labels();
    let _: &[String] = operator_family_row.rejection_evidence_labels();
    let _: &[String] = operator_family_row.direct_hostile_scenario_labels();
    let _: usize = operator_family_row.legal_execution_count();
    let _: usize = operator_family_row.hostile_workload_count();
    let _: usize = operator_family_row.replay_evidence_count();
    let _: usize = operator_family_row.rejection_evidence_count();
    let _: usize = operator_family_row.localized_rejection_evidence_count();
    let _: usize = operator_family_row.branch_local_evidence_count();
    let _: usize = operator_family_row.primitive_family_evidence_count();
    let _: usize = operator_family_row.scale_pressure_evidence_count();
    let _: usize = operator_family_row.derived_breadth_evidence_count();
    let _: &str = operator_family_row.row_digest();
    let _: &str = primitive_row.primitive_family();
    let _: &schema::facade::topology_authoring::MilestoneOnePrimitiveCase =
        primitive_row.primitive();
    let _: &[topology::facade::TopologyMutationFamily] = primitive_row.mutation_families();
    let _: &topology::facade::TopologyMutationDigest = primitive_row.topology_mutation_digest();
    let _: bool = primitive_row.replay_verified();
    let _: &topology::facade::DeterministicDigest =
        primitive_row.final_materialized_topology_digest();
    let _: &topology::facade::DeterministicDigest =
        primitive_row.replay_final_materialized_topology_digest();
    let _: usize = primitive_row.derived_validation_row_count();
    let _: &str = primitive_row.row_digest();
    let _: topology::facade::MilestoneThreeScalePressureSweep = scale_row.sweep();
    let _: &str = scale_row.sweep_label();
    let _: &str = scale_row.primitive_family();
    let _: &schema::facade::topology_authoring::MilestoneOnePrimitiveCase = scale_row.primitive();
    let _: usize = scale_row.workload_size();
    let _: usize = scale_row.mutation_step_count();
    let _: bool = scale_row.branch_local();
    let _: &topology::facade::TopologyMutationDigest = scale_row.topology_mutation_digest();
    let _: bool = scale_row.replay_verified();
    let _: &str = scale_row.final_state_digest();
    let _: &str = scale_row.replay_final_state_digest();
    let _: usize = scale_row.derived_validation_row_count();
    let _: &str = scale_row.row_digest();
}

fn _milestone_three_aggregate_acceptance_row_contracts(
    family_row: &MilestoneThreeHostileFamilyCoverageRow,
    rejection_distribution_row: &MilestoneThreeHostileRejectionDistributionRow,
    naming_distribution_row: &MilestoneThreeHostileNamingDistributionRow,
    scope_row: &MilestoneThreeChangedScopeCoverageRow,
    region_row: &MilestoneThreeDerivedRegionCoverageRow,
    determinism_row: &MilestoneThreeDeterminismRuleRow,
    breadth_row: &MilestoneThreeMutationBreadthCounterRow,
    fallout_row: &MilestoneThreeMutationFalloutBreadthRow,
    fallback_denial_row: &MilestoneThreeDerivedFallbackPolicyDenialRow,
    reuse_row: &MilestoneThreeDerivedReuseLegalityRow,
    derived_work_row: &MilestoneThreeDerivedWorkBreadthRow,
    locality_row: &MilestoneThreeFailureLocalityRow,
    validator_row: &MilestoneThreeValidatorFamilyCoverageRow,
    validation_breadth_row: &MilestoneThreeValidationBreadthRow,
) {
    let _: topology::facade::TopologyMutationFamily = family_row.family();
    let _: usize = family_row.scenario_count();
    let _: &[topology::facade::MilestoneThreeHostileScenario] = family_row.scenarios();
    let _: &str = family_row.row_digest();
    let _: topology::facade::TopologyMutationRejectionClass =
        rejection_distribution_row.rejection_class();
    let _: usize = rejection_distribution_row.case_count();
    let _: &[topology::facade::MilestoneThreeHostileScenario] =
        rejection_distribution_row.scenarios();
    let _: &str = rejection_distribution_row.row_digest();
    let _: topology::facade::TopologyMutationNamingOutcome =
        naming_distribution_row.continuity_outcome_class();
    let _: usize = naming_distribution_row.case_count();
    let _: &[topology::facade::MilestoneThreeHostileScenario] = naming_distribution_row.scenarios();
    let _: &str = naming_distribution_row.row_digest();
    let _: topology::facade::TopologyMutationChangedScope = scope_row.changed_scope();
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
    let _: usize = breadth_row.mutation_record_count();
    let _: usize = breadth_row.changed_scope_count();
    let _: bool = breadth_row.replay_checked();
    let _: topology::facade::MilestoneThreeMutationFalloutClass = fallout_row.fallout_class();
    let _: topology::facade::MilestoneThreeHostileScenario = fallout_row.scenario();
    let _: topology::facade::TopologyMutationDerivedFallbackPolicy = fallout_row.fallback_policy();
    let _: bool = fallout_row.fallback_policy_exceeded();
    let _: Option<topology::facade::TopologyMutationRejectionClass> =
        fallout_row.fallback_rejection_class();
    let _: usize = fallout_row.declared_derived_region_count();
    let _: usize = fallout_row.derived_validation_row_count();
    let _: usize = fallout_row.fallback_count();
    let _: bool = fallout_row.locality_claim_mismatch();
    let _: &str = fallout_row.row_digest();
    let _: topology::facade::MilestoneThreeHostileScenario = fallback_denial_row.scenario();
    let _: topology::facade::TopologyMutationDerivedFallbackPolicy =
        fallback_denial_row.strict_fallback_policy();
    let _: topology::facade::MilestoneThreeMutationFalloutClass =
        fallback_denial_row.observed_fallout_class();
    let _: usize = fallback_denial_row.observed_fallback_count();
    let _: topology::facade::TopologyMutationRejectionClass =
        fallback_denial_row.denied_rejection_class();
    let _: bool = fallback_denial_row.policy_exceeded();
    let _: &str = fallback_denial_row.row_digest();
    let _: topology::facade::MilestoneThreeHostileScenario = reuse_row.scenario();
    let _: bool = reuse_row.recompute_suppression_claimed();
    let _: bool = reuse_row.equivalence_contract_required();
    let _: bool = reuse_row.replay_materialized_topology_equivalent();
    let _: usize = reuse_row.fallback_count();
    let _: topology::facade::MilestoneThreeMutationFalloutClass = reuse_row.fallout_class();
    let _: Option<&topology::facade::DeterministicDigest> = reuse_row.derived_validation_digest();
    let _: &str = reuse_row.row_digest();
    let _: topology::facade::MilestoneThreeHostileScenario = derived_work_row.scenario();
    let _: topology::facade::MilestoneThreeDerivedWorkBreadthClass =
        derived_work_row.invalidation_breadth_class();
    let _: topology::facade::MilestoneThreeDerivedWorkBreadthClass =
        derived_work_row.rebuild_breadth_class();
    let _: usize = derived_work_row.declared_changed_scope_count();
    let _: usize = derived_work_row.declared_derived_region_count();
    let _: usize = derived_work_row.actual_derived_validation_row_count();
    let _: usize = derived_work_row.fallback_count();
    let _: bool = derived_work_row.locality_claimed();
    let _: bool = derived_work_row.locality_claim_mismatch();
    let _: &str = derived_work_row.row_digest();
    let _: topology::facade::TopologyMutationRejectionClass = locality_row.rejection_class();
    let _: &[topology::facade::TopologyMutationFamily] = locality_row.families();
    let _: &[topology::facade::TopologyMutationChangedScope] = locality_row.changed_scopes();
    let _: &[topology::facade::TopologyDerivedRegion] = locality_row.derived_regions();
    let _: topology::facade::MilestoneThreeHostileScenario = validator_row.scenario();
    let _: topology::facade::MilestoneThreeValidatorFamily = validator_row.validator_family();
    let _: &[String] = validator_row.validator_names();
    let _: usize = validator_row.mutation_family_count();
    let _: usize = validator_row.changed_scope_count();
    let _: usize = validator_row.naming_scope_count();
    let _: usize = validator_row.derived_region_count();
    let _: usize = validator_row.derived_validation_row_count();
    let _: bool = validator_row.localized_rejection_boundary();
    let _: &str = validator_row.row_digest();
    let _: topology::facade::MilestoneThreeHostileScenario = validation_breadth_row.scenario();
    let _: topology::facade::MilestoneThreeHostileOutcomeClass =
        validation_breadth_row.outcome_class();
    let _: usize = validation_breadth_row.validator_family_count();
    let _: usize = validation_breadth_row.validator_name_count();
    let _: usize = validation_breadth_row.mutation_family_count();
    let _: usize = validation_breadth_row.changed_scope_count();
    let _: usize = validation_breadth_row.naming_scope_count();
    let _: usize = validation_breadth_row.derived_region_count();
    let _: usize = validation_breadth_row.derived_validation_row_count();
    let _: usize = validation_breadth_row.localized_rejection_boundary_count();
    let _: bool = validation_breadth_row.replay_checked();
    let _: &str = validation_breadth_row.row_digest();
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
