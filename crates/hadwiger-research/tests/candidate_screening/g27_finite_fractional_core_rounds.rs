use hadwiger_research::facade::{
    analyze_g27_algebraic_field_friction_checked, analyze_g27_dual_slack_inversion_checked,
    analyze_g27_w_circles_marginal_pressure_channel_checked,
    analyze_g27_w_circles_tight_atom_contacts_checked,
    audit_g27_w_circles_607_exact_geometry_checked,
    audit_g27_w_circles_607_finite_fractional_core_checked,
    export_g27_same_field_dominant_mwis_artifact_checked,
    preflight_g27_same_field_pb_sat_threshold_checked, preflight_g27_same_field_structure_checked,
    price_g27_w_circles_fixed_dual_channels_checked,
    replay_g27_same_field_dominant_mwis_witness_checked,
    run_g27_same_field_threshold_mwis_bnb_checked, search_g27_same_field_witness_repair_checked,
    search_g27_w_circles_same_field_pressure_interfaces_checked,
    search_g27_w_circles_slack_halo_interfaces_checked, G27AlgebraicFieldFrictionPosture,
    G27DualSlackInversionPosture, G27FiniteFractionalCoreAuditPosture, G27FixedDualPricingPosture,
    G27MwisWitnessReplayStatus, G27PbSatPreflightStatus, G27SameFieldMarginalPressurePosture,
    G27SameFieldPressureInterfacePosture, G27StructurePreflightStatus, G27ThresholdMwisBnbStatus,
    G27TightAtomContactPosture, G27WitnessRepairStatus,
};

use crate::support::handle;

#[test]
fn finite_607_core_audit_retains_machine_replayable_weighted_fractional_lead() {
    let handle = handle();
    let report = audit_g27_w_circles_607_finite_fractional_core_checked(&handle)
        .expect("finite W_circles_607 audit should replay");

    assert_eq!(report.vertex_count(), 607);
    assert_eq!(report.edge_count(), 3_390);
    assert_eq!(report.weight_count(), 607);
    assert_eq!(report.integer_weight_sum(), 1_999_983);
    assert_eq!(report.weighted_independence_upper_bound(), 512_933);
    assert_eq!(
        report.retained_lower_bound().stable_token(),
        "1999983/512933"
    );
    assert_eq!(
        report.lift_needed_to_beat_g27().stable_token(),
        "51749/512933"
    );
    assert_eq!(
        report.posture(),
        G27FiniteFractionalCoreAuditPosture::RetainedFiniteCoreNeedsWeightedIndependenceReplay
    );
    assert!(report.source_url().contains("AvoidingDistance1"));
    assert!(report.archive_sha256().starts_with("sha256:"));
    assert!(report.data_sha256().starts_with("sha256:"));
    assert!(report
        .next_obligation()
        .contains("weighted-independent-set certificate"));
    assert!(!report.admits_theorem_authority());
    assert!(!report.registers_query_invariant_authority());
}

#[test]
fn finite_607_core_exact_geometry_replays_retained_unit_edges_in_g27_field() {
    let handle = handle();
    let report = audit_g27_w_circles_607_exact_geometry_checked(&handle)
        .expect("W_circles_607 exact geometry audit should replay");

    assert_eq!(report.vertex_count(), 607);
    assert_eq!(report.retained_edge_count(), 3_390);
    assert_eq!(report.replayed_edge_count(), 3_390);
    assert_eq!(
        report.shared_field_basis(),
        &["1", "sqrt3", "sqrt11", "sqrt33"]
    );
    assert!(report
        .conclusion()
        .contains("same Q(sqrt3,sqrt11,sqrt33) field"));
    assert!(report.vertex_source_sha256().starts_with("sha256:"));
    assert!(!report.admits_theorem_authority());
    assert!(!report.registers_query_invariant_authority());
}

#[test]
fn same_field_pressure_interface_search_tests_exact_weighted_capacity() {
    let handle = handle();
    let report = search_g27_w_circles_same_field_pressure_interfaces_checked(&handle)
        .expect("same-field pressure interface search should replay exactly");

    assert_eq!(report.searched_anchor_pairs(), 183);
    assert_eq!(report.lift_gap_weight_numerator(), 51_749);
    assert!(!report.retained_candidates().is_empty());
    assert!(report.retained_candidates().len() <= 8);
    assert!(report
        .retained_candidates()
        .windows(2)
        .all(|pair| { pair[0].cross_unit_contact_count() >= pair[1].cross_unit_contact_count() }));

    let best = &report.retained_candidates()[0];
    assert_eq!(best.g27_anchor(), "23");
    assert_eq!(best.g27_anchor_pressure(), 52);
    assert_eq!(best.w_anchor(), 254);
    assert_eq!(best.cross_unit_contact_count(), 358);
    assert_eq!(best.contact_weight_sum(), 2_289_724);
    assert_eq!(best.g27_priced_contact_count(), 358);
    assert_eq!(best.g27_unpriced_contact_count(), 0);
    assert_eq!(best.g27_unpriced_contact_weight_sum(), 0);
    assert!(best.optimistic_capacity_clears_lift_gap());
    assert!(!best.g27_unpriced_capacity_clears_lift_gap());
    assert_eq!(
        report.posture(),
        G27SameFieldPressureInterfacePosture::RetiredNoDenseExactInterface
    );
    assert!(report.conclusion().contains("no high-weight W anchor"));
    assert!(!report.admits_theorem_authority());
    assert!(!report.registers_query_invariant_authority());
}

#[test]
fn slack_halo_interface_search_tests_unpriced_capacity() {
    let handle = handle();
    let report = search_g27_w_circles_slack_halo_interfaces_checked(&handle)
        .expect("slack-halo interface search should replay exactly");

    assert_eq!(report.searched_anchor_pairs(), 1_647);
    assert_eq!(report.lift_gap_weight_numerator(), 51_749);
    assert!(!report.retained_candidates().is_empty());
    assert!(report.retained_candidates().len() <= 8);
    assert!(report.retained_candidates().windows(2).all(|pair| {
        pair[0].g27_unpriced_contact_weight_sum() >= pair[1].g27_unpriced_contact_weight_sum()
    }));

    let best = &report.retained_candidates()[0];
    assert_eq!(best.g27_anchor(), "23");
    assert_eq!(best.g27_anchor_pressure(), 52);
    assert_eq!(best.w_anchor(), 254);
    assert_eq!(best.cross_unit_contact_count(), 358);
    assert_eq!(best.contact_weight_sum(), 2_289_724);
    assert_eq!(best.g27_priced_contact_count(), 358);
    assert_eq!(best.g27_unpriced_contact_count(), 0);
    assert_eq!(best.g27_unpriced_contact_weight_sum(), 0);
    assert_eq!(
        report.posture(),
        G27SameFieldPressureInterfacePosture::RetiredNoDenseExactInterface
    );
    assert!(!report.admits_theorem_authority());
    assert!(!report.registers_query_invariant_authority());
}

#[test]
fn marginal_pressure_channel_report_extracts_exact_contact_diagnostic() {
    let handle = handle();
    let report = analyze_g27_w_circles_marginal_pressure_channel_checked(&handle)
        .expect("marginal pressure channel report should replay exactly");

    assert_eq!(report.g27_anchor(), 23);
    assert_eq!(report.w_anchor(), 254);
    assert_eq!(report.exact_contact_count(), 358);
    assert_eq!(report.contact_weight_sum(), 2_289_724);
    assert_eq!(report.normalized_score_token(), "6955146274232/130008417");
    assert!(report.normalized_score_clears_lift_numerator());
    assert_eq!(report.top_one_share_token(), "127179855495/6955146274232");
    assert_eq!(report.top_five_share_token(), "269928621527/3477573137116");
    assert_eq!(report.top_ten_share_token(), "492512036999/3477573137116");
    assert_eq!(
        report.posture(),
        G27SameFieldMarginalPressurePosture::HeuristicOnlyNeedsReducedCostModel
    );
    assert!(report.conclusion().contains("do not treat"));

    let top = &report.top_channels()[0];
    assert_eq!(top.g27_vertex(), 13);
    assert_eq!(top.w_vertex(), 304);
    assert_eq!(top.w_weight(), 36_195);
    assert_eq!(top.g27_tight_participation(), 36);
    assert_eq!(top.normalized_contribution_token(), "36195/37");
    assert!(!report.admits_theorem_authority());
    assert!(!report.registers_query_invariant_authority());
}

#[test]
fn algebraic_field_friction_requires_retained_exact_foreign_geometry() {
    let handle = handle();
    let report = analyze_g27_algebraic_field_friction_checked(&handle)
        .expect("algebraic field-friction report");

    assert!(!report.candidates().is_empty());
    assert!(report
        .candidates()
        .iter()
        .any(|candidate| candidate.shared_vertex_pressure() >= 20));
    assert!(report
        .candidates()
        .iter()
        .all(|candidate| !candidate.retained_foreign_exact_geometry()));
    assert_eq!(
        report.posture(),
        G27AlgebraicFieldFrictionPosture::RetiredMissingForeignExactGeometry
    );
    assert!(report
        .conclusion()
        .contains("no retained exact foreign coordinate model"));
    assert!(!report.admits_theorem_authority());
    assert!(!report.registers_query_invariant_authority());
}

#[test]
fn dual_slack_inversion_admits_only_if_slack_interface_survives_pessimistic_gate() {
    let handle = handle();
    let report =
        analyze_g27_dual_slack_inversion_checked(&handle).expect("dual-slack inversion report");

    assert!(!report.candidates().is_empty());
    assert!(report.candidates().len() <= 5);
    assert!(report
        .candidates()
        .windows(2)
        .all(|pair| pair[0].slack_inversion_score() >= pair[1].slack_inversion_score()));
    assert!(report
        .candidates()
        .iter()
        .all(|candidate| candidate.tight_neighbor_count() >= 1));
    assert_eq!(
        report.posture(),
        G27DualSlackInversionPosture::RetiredWeakSlackInterface
    );
    assert!(report.funded_candidate().is_none());
    assert!(report
        .candidates()
        .iter()
        .all(|candidate| candidate.slack_inversion_score() < 2_000));
    assert!(report
        .conclusion()
        .contains("no candidate has enough tight-neighborhood pressure"));
    assert!(!report.admits_theorem_authority());
    assert!(!report.registers_query_invariant_authority());
}

#[test]
fn tight_atom_contact_report_funds_fixed_dual_pricing_triage() {
    let handle = handle();
    let report = analyze_g27_w_circles_tight_atom_contacts_checked(&handle)
        .expect("tight-atom contact triage should replay exactly");
    let top_channel = report
        .top_channels()
        .first()
        .expect("at least one tight atom should be ranked");

    assert_eq!(report.g27_anchor(), 23);
    assert_eq!(report.w_anchor(), 254);
    assert_eq!(report.exact_contact_count(), 358);
    assert_eq!(report.contact_weight_sum(), 2_289_724);
    assert_eq!(report.tight_atom_count(), 168);
    assert_eq!(report.contacted_tight_atom_count(), 168);
    assert_eq!(
        report.posture(),
        G27TightAtomContactPosture::FundFixedDualPricing
    );
    assert_eq!(top_channel.atom_size(), 10);
    assert_eq!(top_channel.touched_vertex_count(), 10);
    assert_eq!(top_channel.contact_weight_sum(), 1_044_873);
    assert_eq!(
        top_channel.touched_vertices(),
        &[1, 3, 6, 10, 11, 12, 13, 21, 26, 27]
    );
    assert!(!report.admits_theorem_authority());
    assert!(!report.registers_query_invariant_authority());
}

#[test]
fn fixed_dual_pricing_uses_exact_compatible_w_mwis() {
    let handle = handle();
    let report = price_g27_w_circles_fixed_dual_channels_checked(&handle)
        .expect("fixed-dual pricing diagnostic should replay exactly");
    let top_channel = report
        .top_channels()
        .first()
        .expect("at least one tight atom should be priced");

    assert_eq!(report.g27_anchor(), 23);
    assert_eq!(report.w_anchor(), 254);
    assert_eq!(report.priced_tight_atom_count(), 10);
    assert_eq!(report.w_global_alpha_weight(), 512_933);
    assert_eq!(
        report.posture(),
        G27FixedDualPricingPosture::NeedsStrongerMwisCertificate
    );
    assert_eq!(top_channel.compatibility_summary(), (502, 104));
    assert_eq!(
        top_channel.mwis_summary(),
        (498_748, 758_402, false, 9, 494, 8, 154)
    );
    assert_eq!(
        top_channel.lp_summary(),
        (722_367, 526_858, 521_799, 228, 6, 774_445, 931, 1_181, false, 3)
    );
    assert!(!report.admits_theorem_authority());
    assert!(!report.registers_query_invariant_authority());
}

#[test]
#[ignore = "bounded MWIS research probe takes about two minutes on this instance"]
fn threshold_mwis_bnb_keeps_same_field_alignment_undecided() {
    let handle = handle();
    let report = run_g27_same_field_threshold_mwis_bnb_checked(&handle)
        .expect("threshold MWIS branch-and-bound should replay deterministically");

    assert_eq!(report.status(), G27ThresholdMwisBnbStatus::UndecidedNodeCap);
    assert_eq!(report.component_summary(), (502, 9, 494, 61_894, 451_039));
    assert_eq!(
        report.search_summary(),
        (498_748, 498_748, 743_476, 20, 2, 0, 0, 18)
    );
    assert_eq!(report.witness_vertices().len(), 154);
    assert!(!report.admits_theorem_authority());
    assert!(!report.registers_query_invariant_authority());
}

#[test]
#[ignore = "LP-guided witness repair recomputes cut LP guidance"]
fn lp_guided_witness_repair_is_bounded_and_exactly_replayed() {
    let handle = handle();
    let report = search_g27_same_field_witness_repair_checked(&handle)
        .expect("LP-guided witness repair should replay deterministically");

    assert_eq!(
        report.status(),
        G27WitnessRepairStatus::NotFoundWithinBudget
    );
    assert_eq!(
        report.search_summary(),
        (502, 498_748, 498_748, 512_933, 120, 0, 154)
    );
    assert_eq!(report.witness_vertices().len(), 154);
    assert!(!report.admits_theorem_authority());
    assert!(!report.registers_query_invariant_authority());
}

#[test]
#[ignore = "PB/SAT preflight replays exact compatible geometry and takes about two minutes"]
fn pb_sat_threshold_preflight_bounds_weighted_totalizer_size() {
    let handle = handle();
    let report = preflight_g27_same_field_pb_sat_threshold_checked(&handle)
        .expect("PB/SAT threshold preflight should replay deterministically");

    assert_eq!(report.status(), G27PbSatPreflightStatus::EncodingTooLarge);
    assert_eq!(
        report.instance_summary(),
        (502, 9, 494, 61_894, 451_039, 2_225)
    );
    assert_eq!(
        report.encoding_summary(),
        (2_102_958, 40_784_945_678, 50_000, 37)
    );
    assert!(!report.admits_theorem_authority());
    assert!(!report.registers_query_invariant_authority());
}

#[test]
#[ignore = "structural preflight replays exact compatible geometry and is a kill-test probe"]
fn structure_preflight_classifies_native_decomposition_route() {
    let handle = handle();
    let report = preflight_g27_same_field_structure_checked(&handle)
        .expect("structural preflight should replay deterministically");

    assert_eq!(
        report.status(),
        G27StructurePreflightStatus::RetireNativeStructure
    );
    assert_eq!(
        report.instance_summary(),
        (502, 9, 494, 2_225, 61_894, 451_039, 436_854)
    );
    assert_eq!(report.degree_summary(), (2, 9, 20, 6));
    assert_eq!(report.decomposition_summary(), (0, 1, 494, 1, 0, 1));
    assert_eq!(report.width_summary(), (161, 24_634, 131, 21_058));
    assert!(!report.admits_theorem_authority());
    assert!(!report.registers_query_invariant_authority());
}

#[test]
#[ignore = "MWIS artifact export replays exact compatible geometry and takes about two minutes"]
fn dominant_mwis_artifact_exports_and_replays_incumbent() {
    let handle = handle();
    let artifact = export_g27_same_field_dominant_mwis_artifact_checked(&handle)
        .expect("dominant MWIS artifact should replay deterministically");
    let replay =
        replay_g27_same_field_dominant_mwis_witness_checked(&handle, artifact.incumbent_vertices())
            .expect("incumbent witness should replay deterministically");

    assert_eq!(
        artifact.instance_summary(),
        (502, 494, 2_225, 61_894, 451_039, 436_854)
    );
    assert_eq!(artifact.dominant_vertices().len(), 494);
    assert_eq!(artifact.dominant_weights().len(), 494);
    assert_eq!(artifact.dominant_edges().len(), 2_225);
    assert_eq!(artifact.stable_digest().len(), 64);
    assert_eq!(
        replay.status(),
        G27MwisWitnessReplayStatus::BelowThresholdIndependentSet
    );
    assert_eq!(replay.summary(), (436_854, 498_748, 146, None));
    assert!(!artifact.admits_theorem_authority());
    assert!(!artifact.registers_query_invariant_authority());
    assert!(!replay.admits_theorem_authority());
    assert!(!replay.registers_query_invariant_authority());
}
