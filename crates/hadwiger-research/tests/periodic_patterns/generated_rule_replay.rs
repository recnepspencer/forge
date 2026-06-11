use hadwiger_research::facade::*;

fn handle() -> HadwigerResearchHandle {
    admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
        .unwrap()
}

fn rational(value: i128) -> ExactRational {
    ExactRational::integer(value)
}

fn quotient() -> PeriodicQuotientCell {
    let cell = TilingCell::builder("holonomy-cell")
        .with_rectangular_tile(
            RectangularTileRegion::new(
                "tile-a",
                TilingColorId::new("red").unwrap(),
                rational(0),
                rational(1),
                rational(0),
                rational(1),
            )
            .unwrap()
            .with_boundary_ownership(
                BoundaryOwnershipPolicy::owned_half_open("left,bottom").unwrap(),
            ),
        )
        .unwrap()
        .finish()
        .unwrap();
    PeriodicQuotientCell::builder("holonomy-quotient", cell.reference())
        .with_source_cell(cell)
        .with_lattice_basis_vector("u", rational(2), rational(0))
        .unwrap()
        .with_translation_rule(
            PeriodicTranslationRule::new("wrap", "tile-a", "tile-a")
                .with_translation("u")
                .unwrap()
                .with_color_preserved()
                .unwrap(),
        )
        .unwrap()
        .finish()
        .unwrap()
}

fn graph_version() -> GraphVersion {
    let handle = handle();
    let declaration = declare_research_request_checked(
        &handle,
        CandidateGraphDeclaration::new("periodic-closure-graph").with_graph_version("v1"),
    )
    .admitted()
    .unwrap();
    let graph = GraphIdentity::from_query_declaration("periodic-closure-graph", declaration.into())
        .unwrap();
    GraphVersion::builder(graph.reference(), "v1")
        .with_vertex("a")
        .unwrap()
        .with_vertex("b")
        .unwrap()
        .with_undirected_edge("a", "b")
        .unwrap()
        .finish()
        .unwrap()
}

fn holonomy_loop(to_color: &str) -> ColorHolonomyLoopCertificate {
    ColorHolonomyLoopCertificate::builder("loop-a", "tile-a", "red")
        .with_color_permutation("permute", [("red", to_color), ("blue", "blue")])
        .unwrap()
        .finish()
        .unwrap()
}

fn transcript(status: &str) -> ScreeningSolverTranscript {
    ScreeningSolverTranscript::new(
        "periodic-pattern-test",
        "phase4",
        format!("transcript-{status}"),
        status,
    )
    .unwrap()
}

fn screening_unit_square(region_id: &str, x_offset: i128) -> ScreeningRectangularRegion {
    ScreeningRectangularRegion::new(
        region_id,
        ScreeningRational::integer(x_offset),
        ScreeningRational::integer(x_offset + 1),
        ScreeningRational::integer(0),
        ScreeningRational::integer(1),
    )
    .unwrap()
}

fn periodic_conflict_model() -> PeriodicQuotientRectangleModel {
    PeriodicQuotientRectangleModel::new(
        "generated-periodic-rectangles",
        vec![
            PeriodicQuotientTile::new("left", "red", screening_unit_square("left-region", 0))
                .unwrap(),
            PeriodicQuotientTile::new("right", "red", screening_unit_square("right-region", 2))
                .unwrap(),
        ],
    )
    .unwrap()
}

#[test]
fn generated_pattern_replay_rejects_incompatible_color_holonomy_loop() {
    let handle = handle();
    let quotient = quotient();
    let suite = GeneratedPatternReplaySuite::builder("generated-suite-a", quotient.reference())
        .with_periodic_quotient_cell(quotient)
        .unwrap()
        .with_color_holonomy_loop(holonomy_loop("blue"))
        .unwrap()
        .finish()
        .unwrap();

    let checked = certify_generated_pattern_replay_checked(&handle, suite).unwrap();

    assert!(checked.has_rejected_generated_rule());
    assert!(checked.reusable_negative_evidence().is_some());
    assert_eq!(
        checked.report().counters().color_holonomy_loops_checked(),
        1
    );
    assert_eq!(
        checked
            .report()
            .counters()
            .translation_rotation_closures_checked(),
        0
    );
    assert_eq!(
        checked.report().counters().query_declarations_performed(),
        2
    );
    assert!(!checked.report().query_declaration_digest().is_empty());
    assert!(!checked.admits_theorem_authority());
    assert!(!checked.registers_query_invariant_authority());
}

#[test]
fn generated_pattern_replay_accepts_identity_compatible_color_holonomy_loop() {
    let handle = handle();
    let quotient = quotient();
    let suite =
        GeneratedPatternReplaySuite::builder("generated-suite-identity", quotient.reference())
            .with_periodic_quotient_cell(quotient)
            .unwrap()
            .with_color_holonomy_loop(holonomy_loop("red"))
            .unwrap()
            .finish()
            .unwrap();

    let checked = certify_generated_pattern_replay_checked(&handle, suite).unwrap();

    assert_eq!(checked.report().evaluations().len(), 1);
    assert!(checked.report().blockers().is_empty());
    assert_eq!(
        checked.report().counters().color_holonomy_loops_checked(),
        1
    );
    assert_eq!(
        checked.report().counters().query_declarations_performed(),
        2
    );
    assert!(!checked.has_rejected_generated_rule());
    assert!(!checked.admits_theorem_authority());
}

#[test]
fn generated_pattern_replay_runs_periodic_quotient_conflict_certificate() {
    let handle = handle();
    let quotient = quotient();
    let conflict = PeriodicQuotientConflictCertificate::new(
        "left",
        "right",
        ScreeningRational::integer(-1),
        ScreeningRational::integer(0),
        transcript("periodic-quotient-wraparound-conflict"),
    )
    .unwrap();
    let suite = GeneratedPatternReplaySuite::builder("generated-suite-wrap", quotient.reference())
        .with_periodic_quotient_cell(quotient)
        .unwrap()
        .with_periodic_quotient_conflict_certificate(periodic_conflict_model(), conflict)
        .unwrap()
        .finish()
        .unwrap();

    let checked = certify_generated_pattern_replay_checked(&handle, suite).unwrap();

    assert_eq!(checked.report().evaluations().len(), 1);
    assert_eq!(checked.report().blockers().len(), 1);
    assert!(checked
        .report()
        .parent_artifacts()
        .contains(&checked.report().evaluations()[0].reference()));
    assert_eq!(
        checked
            .report()
            .counters()
            .periodic_quotient_conflicts_checked(),
        1
    );
    assert_eq!(
        checked.report().counters().query_declarations_performed(),
        2
    );
    assert_eq!(
        checked
            .report()
            .counters()
            .screening_evaluations_performed(),
        1
    );
    assert!(checked.has_rejected_generated_rule());
}

#[test]
fn generated_pattern_replay_rejects_corrupt_periodic_quotient_conflict_certificate() {
    let handle = handle();
    let quotient = quotient();
    let corrupt_conflict = PeriodicQuotientConflictCertificate::new(
        "left",
        "right",
        ScreeningRational::integer(3),
        ScreeningRational::integer(0),
        transcript("bad-periodic-quotient-wraparound-conflict"),
    )
    .unwrap();
    let suite =
        GeneratedPatternReplaySuite::builder("generated-suite-bad-wrap", quotient.reference())
            .with_periodic_quotient_cell(quotient)
            .unwrap()
            .with_periodic_quotient_conflict_certificate(
                periodic_conflict_model(),
                corrupt_conflict,
            )
            .unwrap()
            .finish()
            .unwrap();

    let error = certify_generated_pattern_replay_checked(&handle, suite)
        .expect_err("corrupt periodic quotient certificate must fail closed");

    assert!(matches!(
        error,
        GeneratedPatternReplayError::Screening(
            CandidateScreeningError::CertificateReplayRejected {
                family: CandidateScreeningInvariantFamily::PeriodicQuotientGraph,
                reason: "periodic_translated_pair_has_no_unit_conflict"
            }
        )
    ));
}

#[test]
fn equivalent_generated_suites_converge_and_changed_loop_changes_digest() {
    let quotient = quotient();
    let left = GeneratedPatternReplaySuite::builder("generated-suite-b", quotient.reference())
        .with_periodic_quotient_cell(quotient.clone())
        .unwrap()
        .with_color_holonomy_loop(holonomy_loop("red"))
        .unwrap()
        .finish()
        .unwrap();
    let right = GeneratedPatternReplaySuite::builder("generated-suite-b", quotient.reference())
        .with_color_holonomy_loop(holonomy_loop("red"))
        .unwrap()
        .with_periodic_quotient_cell(quotient.clone())
        .unwrap()
        .finish()
        .unwrap();
    let changed = GeneratedPatternReplaySuite::builder("generated-suite-b", quotient.reference())
        .with_periodic_quotient_cell(quotient)
        .unwrap()
        .with_color_holonomy_loop(holonomy_loop("blue"))
        .unwrap()
        .finish()
        .unwrap();

    assert_eq!(left.artifact_digest(), right.artifact_digest());
    assert_ne!(left.artifact_digest(), changed.artifact_digest());
}

#[test]
fn generated_pattern_replay_suite_requires_at_least_one_replay_certificate() {
    let quotient = quotient();

    let result =
        GeneratedPatternReplaySuite::builder("generated-suite-empty", quotient.reference())
            .finish();

    assert!(matches!(
        result,
        Err(GeneratedPatternReplayError::Shape(
            GeneratedPatternReplayShapeError::MissingReplayCertificate
        ))
    ));
}

#[test]
fn generated_pattern_replay_runs_substitution_and_extension_certificates() {
    let handle = handle();
    let quotient = quotient();
    let substitution = SubstitutionConsistencyCertificate::new(
        "substitution-a",
        1,
        vec![SubstitutionConsistencyFailureKind::Boundary],
        transcript("substitution-boundary-failure"),
    )
    .unwrap();
    let extension = FinitePatchBoundaryExtensionCertificate::new(
        "extension-a",
        vec!["red-blue".to_string()],
        Vec::new(),
        transcript("extension-failure"),
    )
    .unwrap();
    let suite = GeneratedPatternReplaySuite::builder("generated-suite-c", quotient.reference())
        .with_periodic_quotient_cell(quotient)
        .unwrap()
        .with_substitution_certificate(substitution)
        .unwrap()
        .with_finite_patch_extension_certificate(extension)
        .unwrap()
        .finish()
        .unwrap();

    let checked = certify_generated_pattern_replay_checked(&handle, suite).unwrap();

    assert_eq!(checked.report().evaluations().len(), 2);
    assert_eq!(checked.report().blockers().len(), 2);
    assert_eq!(
        checked
            .report()
            .counters()
            .substitution_certificates_checked(),
        1
    );
    assert_eq!(
        checked
            .report()
            .counters()
            .finite_patch_extensions_checked(),
        1
    );
    assert_eq!(
        checked
            .report()
            .counters()
            .screening_evaluations_performed(),
        2
    );
    assert!(checked.has_rejected_generated_rule());
}

#[test]
fn generated_pattern_replay_runs_translation_rotation_closure_certificate() {
    let handle = handle();
    let quotient = quotient();
    let graph = graph_version();
    let closure = TranslationRotationClosureCertificate::new(
        "closure-a",
        vec![
            ("a".to_string(), "a".to_string()),
            ("b".to_string(), "b".to_string()),
        ],
        vec![("a".to_string(), "b".to_string())],
        transcript("translation-closure-conflict"),
    )
    .unwrap();
    let suite = GeneratedPatternReplaySuite::builder("generated-suite-d", quotient.reference())
        .with_periodic_quotient_cell(quotient)
        .unwrap()
        .with_translation_rotation_closure_certificate(graph, closure)
        .unwrap()
        .finish()
        .unwrap();

    let checked = certify_generated_pattern_replay_checked(&handle, suite).unwrap();

    assert_eq!(checked.report().evaluations().len(), 1);
    assert_eq!(checked.report().blockers().len(), 1);
    assert_eq!(
        checked
            .report()
            .counters()
            .translation_rotation_closures_checked(),
        1
    );
    assert!(checked.has_rejected_generated_rule());
}
