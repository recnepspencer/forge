use hadwiger_research::facade::{
    admit_hadwiger_research_handle, decide_g27_row_685_next_program_checked,
    materialize_g27_pressure_escape_lead_checked,
    materialize_g27_unit_attachment_obligation_checked, replay_g27_outside_moser_anchor_checked,
    reproduce_g27_geometric_fractional_witness_checked,
    run_g27_pressure_escape_hypothesis_iterations_checked,
    scan_g27_row_685_moser_anchor_breakers_checked,
    screen_g27_quadratic_survivor_mutation_eligibility_checked,
    search_g27_bounded_quadratic_anchors_checked, CandidateScreeningEvaluationMode,
    CandidateScreeningInvariantFamily, CandidateScreeningVerdict, ExactRational,
    FrontierGraphSeedImport, G27EscapeHypothesisIterationKind, G27MutationEligibilityPosture,
    G27OutsideMoserAnchorCandidate, G27OutsideMoserAnchorPosture, G27OutsideMoserAxis,
    G27QuadraticAnchorExtension, G27RoundDecisionPosture, HadwigerCanonicalArtifact,
    HadwigerResearchOperatingContext,
};

#[test]
fn g27_geometric_fractional_seed_imports_with_public_provenance() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("handle admits");

    let imported = hadwiger_research::facade::import_frontier_graph_seed_checked(
        &handle,
        FrontierGraphSeedImport::g27_geometric_fractional(),
    )
    .expect("G27 retained seed imports");

    assert_eq!(imported.graph_version().vertex_count(), 27);
    assert_eq!(imported.graph_version().edge_count(), 49);
    assert_eq!(
        imported.seed_artifact().source_family(),
        "geometric_fractional_chromatic_g27"
    );
    assert!(!imported.seed_artifact().admits_theorem_authority());
}

#[test]
fn g27_geometric_fractional_retained_witness_replays_as_priority_evidence() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("handle admits");

    let report = reproduce_g27_geometric_fractional_witness_checked(&handle)
        .expect("retained G27 witness replays");

    assert_eq!(report.structural_replay().vertex_count(), 27);
    assert_eq!(report.structural_replay().edge_count(), 49);
    assert_eq!(report.structural_replay().independent_set_count(), 182_304);
    assert_eq!(report.structural_replay().isometry_count(), 16_855);
    assert_eq!(
        report.structural_replay().witness_coordinate_count(),
        16_855
    );
    assert_eq!(report.dual_replay().atom_columns_checked(), 182_304);
    assert_eq!(report.dual_replay().matrix_nonzero_count(), 39_072_252);
    assert_eq!(report.dual_replay().witness_coordinate_count(), 16_855);
    assert!(
        report.dual_replay().positive_slack_columns() + report.dual_replay().zero_slack_columns()
            == 182_304
    );
    assert!(report.dual_replay().common_denominator_digits() > 200);
    assert_eq!(
        report.dual_replay().pressure_report().tight_atom_count(),
        report.dual_replay().zero_slack_columns()
    );
    assert!(!report
        .dual_replay()
        .pressure_report()
        .tight_atom_size_distribution()
        .is_empty());
    assert_eq!(
        report.dual_replay().pressure_report().top_vertices().len(),
        27
    );
    assert!(!report
        .dual_replay()
        .pressure_report()
        .top_isometry_rows()
        .is_empty());
    assert!(
        report.dual_replay().pressure_report().top_vertices()[0].tight_atom_participation()
            >= report.dual_replay().pressure_report().top_vertices()[26].tight_atom_participation()
    );
    assert_eq!(
        report.evaluation().family(),
        CandidateScreeningInvariantFamily::GeometricFractionalChromaticNumber
    );
    assert_eq!(
        report.evaluation().verdict(),
        CandidateScreeningVerdict::Priority
    );
    assert_eq!(
        report.evaluation().mode(),
        CandidateScreeningEvaluationMode::CheckedCertificate
    );
    assert!(report
        .evaluation()
        .evidence()
        .contains("retained_g27_geometric_fractional"));
    assert!(!report.admits_theorem_authority());
    assert!(!report.evaluation().admits_theorem_authority());
}

#[test]
fn g27_pressure_escape_loop_runs_five_evidence_driven_iterations() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("handle admits");

    let run = run_g27_pressure_escape_hypothesis_iterations_checked(&handle)
        .expect("G27 pressure escape loop runs");

    assert_eq!(run.iterations().len(), 5);
    assert_eq!(
        run.iterations()[0].kind(),
        G27EscapeHypothesisIterationKind::OutsideFieldClamp
    );
    assert_eq!(
        run.iterations()[2].kind(),
        G27EscapeHypothesisIterationKind::IsometryBreaker
    );
    assert!(run
        .iterations()
        .iter()
        .all(|iteration| iteration.escape_requirement().contains("Moser")
            || iteration.escape_requirement().contains("outside")));
    assert!(run.best_iteration().score() >= run.iterations()[0].score());
    assert!(!run.admits_theorem_authority());
    assert!(!run.registers_query_invariant_authority());
}

#[test]
fn g27_best_pressure_escape_lead_materializes_non_singleton_obligation() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("handle admits");

    let lead = materialize_g27_pressure_escape_lead_checked(&handle)
        .expect("G27 best pressure escape lead materializes");

    assert_eq!(
        lead.best_iteration().kind(),
        G27EscapeHypothesisIterationKind::IsometryBreaker
    );
    assert_eq!(lead.isometry_detail().row_index(), 685);
    assert_eq!(
        lead.isometry_detail().mapping_pairs(),
        &[
            ("8".to_string(), "13".to_string()),
            ("18".to_string(), "6".to_string())
        ]
    );
    assert_eq!(
        lead.isometry_detail().domain_vertices(),
        &["8".to_string(), "18".to_string()]
    );
    assert_eq!(
        lead.isometry_detail().image_vertices(),
        &["13".to_string(), "6".to_string()]
    );
    assert!(lead.mutation_obligation().requires_outside_moser_geometry());
    assert!(lead
        .mutation_obligation()
        .preserve_requirement()
        .contains("8->13,18->6"));
    assert_eq!(lead.parent_artifacts().len(), 1);
    assert!(!lead.admits_theorem_authority());
    assert!(!lead.registers_query_invariant_authority());
}

#[test]
fn g27_row_685_moser_anchor_scan_retains_capped_breakers_as_suppression_evidence() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("handle admits");

    let report = scan_g27_row_685_moser_anchor_breakers_checked(&handle)
        .expect("row-685 Moser anchor scan runs");

    assert_eq!(report.source_lead().isometry_detail().row_index(), 685);
    assert_eq!(report.expansion(), 1);
    assert_eq!(report.coefficient_points_checked(), 2058);
    assert_eq!(report.breaker_count(), 56);
    assert_eq!(report.retained_breakers().len(), 12);
    assert!(report
        .retained_breakers()
        .iter()
        .all(|candidate| !candidate.adjacent_lead_vertices().is_empty()));
    assert!(report.suppresses_moser_only_breakers());
    assert!(report.suppression_reason().contains("outside-Moser"));
    assert_eq!(report.parent_artifacts().len(), 1);
    assert!(!report.admits_theorem_authority());
    assert!(!report.registers_query_invariant_authority());
}

#[test]
fn g27_unit_attachment_obligation_names_missing_certificate_language() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("handle admits");

    let report = materialize_g27_unit_attachment_obligation_checked(&handle)
        .expect("unit attachment obligation materializes");

    assert_eq!(report.blocked_candidate_count(), 16);
    assert_eq!(report.required_targets(), &["8", "13", "18", "6"]);
    assert!(report
        .required_certificate_language()
        .contains("exact algebraic squared-distance replay"));
    assert!(report.blocks_slack_response_until_satisfied());
    assert_eq!(report.parent_artifacts().len(), 1);
    assert!(!report.admits_theorem_authority());
    assert!(!report.registers_query_invariant_authority());
}

#[test]
fn g27_row_685_decision_funds_unit_attachment_language_not_shape_only_mutation() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("handle admits");

    let report =
        decide_g27_row_685_next_program_checked(&handle).expect("decision report materializes");

    assert_eq!(
        report.decision(),
        G27RoundDecisionPosture::FundUnitAttachmentCertificateLanguage
    );
    assert!(report.keeps_row_685_funded());
    assert!(report.funded_next_program().contains("unit-attachment"));
    assert!(report.blocked_lane().contains("slack response"));
    assert_eq!(report.obligation_report().blocked_candidate_count(), 16);
    assert_eq!(report.parent_artifacts().len(), 1);
    assert!(!report.admits_theorem_authority());
    assert!(!report.registers_query_invariant_authority());
}

#[test]
fn g27_bounded_quadratic_anchor_search_retains_shape_checked_survivors() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("handle admits");

    let report = search_g27_bounded_quadratic_anchors_checked(&handle)
        .expect("bounded quadratic search runs");

    assert_eq!(report.moser_scan().breaker_count(), 56);
    assert_eq!(report.radicands(), &[2, 3]);
    assert_eq!(report.bases_checked(), 12);
    assert_eq!(report.candidates_checked(), 48);
    assert_eq!(report.retained_survivors().len(), 16);
    assert!(report.has_surviving_candidates());
    assert!(report
        .retained_survivors()
        .iter()
        .all(|candidate| candidate.is_outside_moser()));
    assert_eq!(report.parent_artifacts().len(), 1);
    assert!(!report.admits_theorem_authority());
    assert!(!report.registers_query_invariant_authority());
}

#[test]
fn g27_quadratic_survivor_mutation_eligibility_blocks_shape_only_candidates() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("handle admits");

    let report = screen_g27_quadratic_survivor_mutation_eligibility_checked(&handle)
        .expect("mutation eligibility screen runs");

    assert_eq!(report.search_report().candidates_checked(), 48);
    assert_eq!(report.candidates_screened(), 16);
    assert_eq!(report.eligible_count(), 0);
    assert!(!report.admits_mutated_graph_artifacts());
    assert_eq!(report.blockers().len(), 16);
    assert!(report.blockers().iter().all(|blocker| {
        blocker.posture() == G27MutationEligibilityPosture::BlockedMissingUnitAttachmentEvidence
            && blocker
                .required_evidence()
                .contains("unit-attachment certificate")
    }));
    assert_eq!(report.parent_artifacts().len(), 1);
    assert!(!report.admits_theorem_authority());
    assert!(!report.registers_query_invariant_authority());
}

#[test]
fn g27_outside_moser_anchor_language_suppresses_inside_moser_candidate() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("handle admits");
    let candidate = G27OutsideMoserAnchorCandidate::moser_basis("inside-capped", [-1, 2, 4, 3])
        .expect("candidate shape");

    let report = replay_g27_outside_moser_anchor_checked(&handle, candidate)
        .expect("inside Moser anchor replays as suppression");

    assert_eq!(
        report.posture(),
        G27OutsideMoserAnchorPosture::SuppressedInsideMoser
    );
    assert!(!report.candidate().is_outside_moser());
    assert!(report.reason().contains("Moser-basis"));
    assert_eq!(report.source_lead().isometry_detail().row_index(), 685);
    assert_eq!(report.moser_scan().breaker_count(), 56);
    assert!(!report.admits_theorem_authority());
    assert!(!report.registers_query_invariant_authority());
}

#[test]
fn g27_outside_moser_anchor_language_shape_checks_quadratic_candidate() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("handle admits");
    let extension = G27QuadraticAnchorExtension::new(
        G27OutsideMoserAxis::X,
        2,
        ExactRational::fraction(1, 3).expect("non-zero rational"),
    )
    .expect("quadratic extension shape");
    let candidate = G27OutsideMoserAnchorCandidate::quadratic_extension(
        "outside-sqrt2",
        [-1, 2, 4, 3],
        extension,
    )
    .expect("candidate shape");

    let report = replay_g27_outside_moser_anchor_checked(&handle, candidate)
        .expect("outside Moser anchor shape checks");

    assert_eq!(
        report.posture(),
        G27OutsideMoserAnchorPosture::ShapeCheckedOutsideMoser
    );
    assert!(report.candidate().is_outside_moser());
    assert_eq!(report.candidate().extension().unwrap().radicand(), 2);
    assert!(report.reason().contains("outside the retained Moser basis"));
    assert!(!report.admits_theorem_authority());
    assert!(!report.registers_query_invariant_authority());
}
