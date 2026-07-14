use hadwiger_research::facade::{
    admit_hadwiger_research_handle, audit_g27_exact_moser_basis_checked,
    derive_g27_exact_rotation_pin_equation_checked, enumerate_g27_tight_atom_hitting_sets_checked,
    preflight_g27_cross_ring_fusion_column_generation_checked,
    preflight_g27_pressure_skeleton_spindle_checked,
    replay_g27_cross_ring_column_generation_state_checked,
    replay_g27_exact_rotation_pin_closures_checked, replay_g27_rotation_pin_batch_exact_checked,
    score_g27_rotation_pin_exact_survivors_checked,
    search_g27_cross_ring_fusion_candidates_checked,
    search_g27_pressure_skeleton_spindle_rotations_checked,
    search_g27_rotation_pin_closures_checked,
    test_g27_parameterized_one_anchor_transversal_checked,
    G27CrossRingColumnGenerationReplayPosture, G27CrossRingFusionPreflightPosture,
    G27ExactRotationPinClosureReplayPosture, G27ExactRotationPinEquationPosture,
    G27HittingSetPosture, G27MotifSearchPosture, G27OneAnchorTransversalPosture,
    G27RotationPinBatchExactReplayPosture, G27RotationPinClosurePosture,
    G27RotationPinPressureScorePosture, G27SpindlePreflightPosture, HadwigerCanonicalArtifact,
    HadwigerResearchOperatingContext,
};

#[test]
fn round6_exact_tight_atom_hitting_set_falsifies_size_four_transversal() {
    let handle = handle();
    let report =
        enumerate_g27_tight_atom_hitting_sets_checked(&handle).expect("hitting-set report");

    assert_eq!(report.tight_atom_count(), 168);
    assert_eq!(report.size_le_four_hitting_sets(), 0);
    assert_eq!(report.minimum_hitting_set_size(), 5);
    assert_eq!(
        report.posture(),
        G27HittingSetPosture::SmallTransversalFalsified
    );
    assert_eq!(report.retained_minimum_transversals().len(), 3);
    assert_eq!(
        report.retained_minimum_transversals()[0].vertices(),
        &["4", "5", "6", "7", "14"]
    );
    assert_eq!(
        report.retained_minimum_transversals()[2].vertices(),
        &["5", "7", "9", "13", "16"]
    );
    assert!(report
        .conclusion()
        .contains("minimum tight-atom transversal size is 5"));
    assert!(!report.admits_theorem_authority());
    assert!(!report.registers_query_invariant_authority());
}

#[test]
fn round7_parameterized_one_anchor_route_is_blocked_by_size_five_requirement() {
    let handle = handle();
    let report =
        test_g27_parameterized_one_anchor_transversal_checked(&handle).expect("one-anchor report");

    assert_eq!(
        report.tested_transversal().vertices(),
        &["5", "7", "9", "13", "16"]
    );
    assert_eq!(report.moser_basis_common_anchor_count(), 0);
    assert_eq!(
        report.posture(),
        G27OneAnchorTransversalPosture::SmallTransversalFalsified
    );
    assert!(report.conclusion().contains("size-5 transversal"));
    assert_eq!(report.parent_artifacts().len(), 1);
    assert!(!report.admits_theorem_authority());
    assert!(!report.registers_query_invariant_authority());
}

#[test]
fn round8_pressure_skeleton_spindle_preflight_funds_manufactured_rotation() {
    let handle = handle();
    let report =
        preflight_g27_pressure_skeleton_spindle_checked(&handle).expect("spindle preflight");

    assert_eq!(report.hinge_vertex(), "8");
    assert!(report.fragment_vertices().contains(&"8".to_string()));
    assert!(report.fragment_vertices().contains(&"18".to_string()));
    assert!(report.fragment_vertices().contains(&"23".to_string()));
    assert_eq!(report.tight_atoms_containing_fragment(), 1);
    assert_eq!(
        report.posture(),
        G27SpindlePreflightPosture::FundManufacturedRotation
    );
    assert!(report
        .next_test()
        .contains("manufactured outside-Moser closure"));
    assert_eq!(report.parent_artifacts().len(), 1);
    assert!(!report.admits_theorem_authority());
    assert!(!report.registers_query_invariant_authority());
}

#[test]
fn motif1_spindle_rotation_search_retains_outside_moser_candidates() {
    let handle = handle();
    let report = search_g27_pressure_skeleton_spindle_rotations_checked(&handle)
        .expect("spindle rotation search");

    assert_eq!(report.suppressed_in_ring_rotation_count(), 1);
    assert_eq!(report.retained_candidates().len(), 2);
    assert_eq!(report.retained_candidates()[0].rotation_label(), "pi/6");
    assert_eq!(report.retained_candidates()[0].pin_vertex(), "21");
    assert_eq!(
        report.retained_candidates()[0].nontrivial_pin_closure_count(),
        0
    );
    assert_eq!(
        report.retained_candidates()[1].field_escape_basis(),
        "foreign_sqrt2_rotation"
    );
    assert_eq!(
        report.retained_candidates()[1].nontrivial_pin_closure_count(),
        0
    );
    assert_eq!(
        report.posture(),
        G27MotifSearchPosture::CandidateRetainedNeedsReplay
    );
    assert!(report.next_replay().contains("broaden rotation/pin search"));
    assert!(!report.admits_theorem_authority());
    assert!(!report.registers_query_invariant_authority());
}

#[test]
fn exact_moser_basis_audit_pins_coordinate_model_for_rotation_replay() {
    let handle = handle();
    let report = audit_g27_exact_moser_basis_checked(&handle).expect("basis audit");

    assert_eq!(report.retained_float_coordinate_count(), 27);
    assert_eq!(report.exact_unit_edge_count(), 49);
    assert_eq!(report.exact_non_edge_count(), 302);
    assert_eq!(report.exact_basis().len(), 4);
    assert!(report.exact_basis()[3].contains("sqrt33"));
    assert!(report.conclusion().contains("symbolic coordinates"));
    assert_eq!(report.parent_artifacts().len(), 1);
    assert!(!report.admits_theorem_authority());
    assert!(!report.registers_query_invariant_authority());
}

#[test]
fn broadened_rotation_pin_search_finds_nontrivial_closure_candidate() {
    let handle = handle();
    let report = search_g27_rotation_pin_closures_checked(&handle).expect("rotation-pin search");

    assert_eq!(report.total_candidate_count(), 206);
    assert_eq!(report.retained_candidates().len(), 12);
    assert_eq!(
        report.posture(),
        G27RotationPinClosurePosture::FloatScreenedNeedsExactReplay
    );
    let best = report
        .best_unsuppressed_candidate()
        .expect("unsuppressed candidate retained");
    assert_eq!(best.witness_vertex(), "10");
    assert_eq!(best.pin_vertex(), "27");
    assert_eq!(best.theta_millidegrees(), 103_221);
    assert_eq!(best.closure_pairs().len(), 9);
    assert!(!best.special_angle_suppressed());
    assert!(report.next_replay().contains("exact algebraic angle"));
    assert!(!report.admits_theorem_authority());
    assert!(!report.registers_query_invariant_authority());
}

#[test]
fn exact_rotation_pin_equation_derives_manufactured_field_extension() {
    let handle = handle();
    let report =
        derive_g27_exact_rotation_pin_equation_checked(&handle).expect("exact equation report");

    assert_eq!(report.hinge_vertex(), "8");
    assert_eq!(report.witness_vertex(), "10");
    assert_eq!(report.pin_vertex(), "27");
    assert_eq!(report.closure_pair_count(), 9);
    assert_eq!(report.moving_radius_squared(), "3");
    assert_eq!(report.pin_distance_squared(), "(9-sqrt33)/2");
    assert_eq!(report.witness_pin_dot(), "(9-sqrt33)/4");
    assert_eq!(report.rotated_pin_dot(), "(13-sqrt33)/4");
    assert_eq!(report.height_numerator(), "(7+sqrt33)/8");
    assert_eq!(report.required_extension(), "sqrt((7+sqrt33)/8)");
    assert!(report
        .closure_replay_obligation()
        .contains("manufactured radical"));
    assert_eq!(
        report.posture(),
        G27ExactRotationPinEquationPosture::ManufacturedFieldExtensionRequired
    );
    assert!(report.requires_new_field_extension());
    assert_eq!(report.parent_artifacts().len(), 1);
    assert!(!report.admits_theorem_authority());
    assert!(!report.registers_query_invariant_authority());
}

#[test]
fn exact_rotation_pin_closure_replay_retires_float_closure_lead() {
    let handle = handle();
    let report =
        replay_g27_exact_rotation_pin_closures_checked(&handle).expect("exact closure replay");

    assert_eq!(report.replayed_float_closure_pair_count(), 9);
    assert_eq!(report.branches().len(), 2);
    assert_eq!(report.exact_unit_pair_count(), 2);
    assert_eq!(report.rejected_float_closure_pair_count(), 16);
    assert_eq!(
        report.posture(),
        G27ExactRotationPinClosureReplayPosture::FloatClosureRetired
    );
    for branch in report.branches() {
        assert_eq!(
            branch.exact_unit_pairs(),
            &[("10".to_string(), "27".to_string())]
        );
        assert_eq!(branch.rejected_pairs().len(), 8);
        assert!(branch
            .rejected_pairs()
            .iter()
            .any(|pair| pair.moving_vertex() == "1" && pair.static_vertex() == "27"));
    }
    assert!(report.conclusion().contains("only the intended 10-27 pin"));
    assert_eq!(report.parent_artifacts().len(), 1);
    assert!(!report.admits_theorem_authority());
    assert!(!report.registers_query_invariant_authority());
}

#[test]
fn rotation_pin_batch_exact_replay_retires_broad_closure_but_retains_two_pin_candidates() {
    let handle = handle();
    let report = replay_g27_rotation_pin_batch_exact_checked(&handle).expect("batch exact replay");

    assert_eq!(report.retained_candidate_count(), 12);
    assert_eq!(report.broad_exact_closure_count(), 0);
    assert_eq!(report.small_exact_closure_count(), 3);
    assert_eq!(report.best_candidates().len(), 4);
    assert_eq!(
        report.posture(),
        G27RotationPinBatchExactReplayPosture::BroadClosureRetiredSmallClosuresRetained
    );
    assert!(report
        .best_candidates()
        .iter()
        .all(|candidate| candidate.max_exact_unit_pairs_per_branch() <= 2));
    assert!(report
        .best_candidates()
        .iter()
        .any(|candidate| candidate.max_exact_unit_pairs_per_branch() == 2));
    assert_eq!(report.best_candidates()[0].exact_unit_pairs().len(), 2);
    assert!(report
        .conclusion()
        .contains("two-pin manufactured closures"));
    assert_eq!(report.parent_artifacts().len(), 1);
    assert!(!report.admits_theorem_authority());
    assert!(!report.registers_query_invariant_authority());
}

#[test]
fn rotation_pin_pressure_score_decides_whether_two_pin_survivors_deserve_lp_preflight() {
    let handle = handle();
    let report =
        score_g27_rotation_pin_exact_survivors_checked(&handle).expect("pressure score report");

    assert_eq!(report.scored_candidate_count(), 3);
    assert!(!report.retained_scores().is_empty());
    assert!(report
        .retained_scores()
        .iter()
        .all(|score| score.exact_pair_count() == 2));
    assert!(report.retained_scores()[0].tight_atom_pressure_score() > 0);
    assert_eq!(
        report.posture(),
        G27RotationPinPressureScorePosture::PivotToCrossRingFusion
    );
    assert_eq!(report.fundable_candidate_count(), 0);
    assert!(report.conclusion().contains("pivot to cross-ring fusion"));
    assert_eq!(report.parent_artifacts().len(), 1);
    assert!(!report.admits_theorem_authority());
    assert!(!report.registers_query_invariant_authority());
}

#[test]
fn motif3_cross_ring_fusion_search_retains_foreign_field_candidates() {
    let handle = handle();
    let report = search_g27_cross_ring_fusion_candidates_checked(&handle).expect("fusion search");

    assert_eq!(report.retained_candidates().len(), 3);
    assert_eq!(
        report.retained_candidates()[0].core_label(),
        "76_21_fractional_core"
    );
    assert_eq!(report.retained_candidates()[0].foreign_radicand(), 2);
    assert_eq!(
        report.retained_candidates()[2].pin_family(),
        "column_generation_required"
    );
    assert_eq!(
        report.posture(),
        G27MotifSearchPosture::CandidateRetainedNeedsColumnGeneration
    );
    assert!(report.next_replay().contains("column-generated"));
    assert!(!report.admits_theorem_authority());
    assert!(!report.registers_query_invariant_authority());
}

#[test]
fn cross_ring_fusion_preflight_selects_foreign_fractional_core_for_column_generation() {
    let handle = handle();
    let report = preflight_g27_cross_ring_fusion_column_generation_checked(&handle)
        .expect("fusion preflight");

    assert_eq!(report.scored_candidate_count(), 3);
    assert_eq!(
        report.posture(),
        G27CrossRingFusionPreflightPosture::FundColumnGeneration
    );
    assert_eq!(
        report.selected_candidate().core_label(),
        "76_21_fractional_core"
    );
    assert_eq!(report.selected_candidate().foreign_radicand(), 2);
    assert!(report.selected_candidate().shared_vertex_pressure() >= 20);
    assert_eq!(report.selected_candidate().retained_core_scale_score(), 76);
    assert!(report
        .selected_candidate()
        .column_generation_obligation()
        .contains("master/pricing replay"));
    assert_eq!(report.parent_artifacts().len(), 2);
    assert!(!report.admits_theorem_authority());
    assert!(!report.registers_query_invariant_authority());
}

#[test]
fn cross_ring_column_generation_state_retires_asymptotic_core_as_finite_fusion_target() {
    let handle = handle();
    let report = replay_g27_cross_ring_column_generation_state_checked(&handle)
        .expect("column generation state replay");

    assert_eq!(report.selected_core_label(), "76_21_fractional_core");
    assert_eq!(report.foreign_radicand(), 2);
    assert_eq!(report.shared_vertex(), "8");
    assert_eq!(report.retained_g27_lower_bound().stable_token(), "4/1");
    assert_eq!(report.foreign_core_lower_bound().stable_token(), "76/21");
    assert_eq!(report.lift_needed_to_beat_g27().stable_token(), "8/21");
    assert_eq!(
        report.posture(),
        G27CrossRingColumnGenerationReplayPosture::RetiredAsymptoticDischargingCore
    );
    assert_eq!(report.pricing_obligations().len(), 4);
    assert!(report
        .pricing_obligations()
        .iter()
        .any(|row| row.obligation_kind() == "foreign_core_columns"));
    assert!(report.conclusion().contains("asymptotic Cranston-Rabern"));
    assert!(report.conclusion().contains("8/21"));
    assert_eq!(report.parent_artifacts().len(), 1);
    assert!(!report.admits_theorem_authority());
    assert!(!report.registers_query_invariant_authority());
}

fn handle() -> hadwiger_research::facade::HadwigerResearchHandle {
    admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
        .expect("handle admits")
}
