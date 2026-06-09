const AUDITED_CONSTRUCTION_QUERY_CERTIFICATION_FILES: [(&str, &str); 4] = [
    (
        "worth-kernel.construction-runtime-truth",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/tests/support/runtime_truth.rs"
        )),
    ),
    (
        "worth-kernel.construction-query-projection-parity",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/tests/support/projection_consumption.rs"
        )),
    ),
    (
        "worth-kernel.construction-corpus-mod",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/certification/mod.rs"
        )),
    ),
    (
        "worth-kernel.branch-preview-basis-support",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/tests/support/branch_preview_basis.rs"
        )),
    ),
];

const FORBIDDEN_PARITY_BYPASS_PATTERNS: [&str; 4] = [
    "prepare_primitive_construction_topology_query_admitted_handoff_from_request",
    "prepare_primitive_construction_birth_placement_facts",
    "prepare_primitive_construction_rejected_facts",
    "prepare_realization_snapshot",
];

const FORBIDDEN_CONSTRUCTION_REPORT_SHELF_PATTERNS: [&str; 12] = [
    "family_boundary_report",
    "family_boundary_drift_report",
    "lane_report",
    "milestone_closeout/report",
    "ordering_report",
    "PrimitiveConstructionFamilyBoundaryTransitionClass",
    "PrimitiveConstructionFamilyBoundaryLowerLayerWitnessSummary",
    "PrimitiveConstructionCompoundOrderLaneReport",
    "PrimitiveConstructionCompoundAuthoringOrderRow",
    "prepare_primitive_construction_compound_ordering_parity_report",
    "PrimitiveConstructionCompoundMilestoneCloseoutReport",
    "PrimitiveConstructionCorpusCloseoutGateStatus",
];

const FORBIDDEN_SIMPLEX_LADDER_SHELF_PATTERNS: [&str; 6] = [
    "simplex_ladder_report",
    "simplex_ladder_support",
    "prepare_primitive_construction_simplex_realization_strategy_ladder_report",
    "PrimitiveConstructionSimplexRealizationStrategyLadderReport",
    "PrimitiveConstructionSimplexRealizationLadderRow",
    "prepare_simplex_ladder_row",
];

#[test]
fn phase_nine_construction_query_certification_no_longer_rebuilds_runtime_truth_locally() {
    let violations = AUDITED_CONSTRUCTION_QUERY_CERTIFICATION_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_PARITY_BYPASS_PATTERNS
                .iter()
                .filter(move |pattern| {
                    *label != "worth-kernel.construction-runtime-truth"
                        && source.contains(**pattern)
                })
                .map(move |pattern| format!("{label}:{pattern}"))
                .chain([
                    "prepare_primitive_construction_certification_runtime_truth(request.clone())",
                    "let replay_truth = prepare_primitive_construction_certification_runtime_truth(request);",
                    "ReplayDrift { family: PrimitiveConstructionFamily }",
                    "mod basis_preview_parity;",
                ]
                .into_iter()
                .filter(move |pattern| source.contains(*pattern))
                .map(move |pattern| format!("{label}:{pattern}")))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "phase-nine construction deletion proof failed because query certification lanes still rebuilt runtime truth locally instead of consuming the shared certification runtime-truth seam: {violations:?}"
    );
}

#[test]
fn phase_nine_construction_corpus_no_longer_teaches_deleted_boundary_drift_or_lower_layer_summary_shelves(
) {
    let violations = AUDITED_CONSTRUCTION_QUERY_CERTIFICATION_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_CONSTRUCTION_REPORT_SHELF_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "phase-nine construction deletion proof failed because the corpus report band reintroduced the deleted boundary-drift alias or lower-layer witness summary shelf: {violations:?}"
    );
}

#[test]
fn phase_nine_construction_corpus_root_no_longer_teaches_deleted_family_boundary_report_shelf() {
    let certification_mod = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/certification/mod.rs"
    ));
    let violations = FORBIDDEN_CONSTRUCTION_REPORT_SHELF_PATTERNS
        .iter()
        .filter(|pattern| certification_mod.contains(**pattern))
        .map(|pattern| format!("worth-kernel.construction-certification-mod:{pattern}"))
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "phase-nine construction deletion proof failed because the corpus root still teaches the deleted family-boundary shelf: {violations:?}"
    );
}

#[test]
fn phase_nine_construction_corpus_no_longer_teaches_deleted_simplex_ladder_report_shelves() {
    let violations = AUDITED_CONSTRUCTION_QUERY_CERTIFICATION_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_SIMPLEX_LADDER_SHELF_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "phase-nine construction deletion proof failed because the corpus report band reintroduced the deleted simplex ladder report shelf: {violations:?}"
    );
}

#[test]
fn phase_nine_construction_corpus_catalog_and_simplex_registry_no_longer_live_in_certification_tree(
) {
    let tests_support_mod = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/mod.rs"
    ));
    let replay_generation = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/corpus_replay_generation.rs"
    ));
    let simplex_ladder = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/corpus_simplex_ladder.rs"
    ));
    let violations = [
        (
            "worth-kernel.certification-mod",
            AUDITED_CONSTRUCTION_QUERY_CERTIFICATION_FILES[2].1,
            "mod corpus;",
        ),
        (
            "worth-kernel.corpus-replay-generation",
            replay_generation,
            "certification::corpus::{\n    primitive_construction_corpus",
        ),
        (
            "worth-kernel.corpus-simplex-ladder",
            simplex_ladder,
            "construction::certification::corpus::{\n    required_simplex_exhaustion_witness_kinds, required_simplex_ladder_scenarios,",
        ),
        (
            "worth-kernel.tests-support-mod",
            tests_support_mod,
            "pub(crate) mod corpus_cases;",
        ),
        (
            "worth-kernel.tests-support-mod",
            tests_support_mod,
            "pub(crate) mod corpus_simplex_registry;",
        ),
    ]
    .into_iter()
    .filter_map(|(label, source, pattern)| {
        if label == "worth-kernel.tests-support-mod" {
            (!source.contains(pattern)).then(|| format!("{label}:missing:{pattern}"))
        } else {
            source.contains(pattern).then(|| format!("{label}:{pattern}"))
        }
    })
    .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "phase-nine construction deletion proof failed because the replay corpus catalog or simplex registry drifted back into the certification tree instead of living in plain construction test support: {violations:?}"
    );
}

#[test]
fn phase_nine_construction_corpus_no_longer_teaches_deleted_compound_milestone_closeout_shelf() {
    let closeout_reports = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/compound_closeout_reports.rs"
    ));
    let violations = [(
        "worth-kernel.compound-closeout-tests",
        closeout_reports,
        "prepare_primitive_construction_compound_milestone_closeout_report",
    )]
    .into_iter()
    .filter_map(|(label, source, pattern)| {
        source
            .contains(pattern)
            .then(|| format!("{label}:{pattern}"))
    })
    .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "phase-nine construction deletion proof failed because the compound corpus band reintroduced the deleted milestone closeout shelf: {violations:?}"
    );
}

#[test]
fn phase_nine_construction_compound_band_no_longer_teaches_deleted_ordering_scenario_shelf() {
    let compound_lane_support = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/compound_lane_support.rs"
    ));
    let compound_parity_reports = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/compound_parity_reports.rs"
    ));
    let violations = [
        (
            "worth-kernel.compound-tests-lane-support",
            compound_lane_support,
            "PrimitiveConstructionCompoundOrderingScenarioRow",
        ),
        (
            "worth-kernel.compound-tests-lane-support",
            compound_lane_support,
            "scenario_row_for(",
        ),
        (
            "worth-kernel.compound-tests-lane-support",
            compound_lane_support,
            "scenario_rows(",
        ),
        (
            "worth-kernel.compound-tests-parity-reports",
            compound_parity_reports,
            "scenario_row_for(",
        ),
    ]
    .into_iter()
    .filter_map(|(label, source, pattern)| {
        source
            .contains(pattern)
            .then(|| format!("{label}:{pattern}"))
    })
    .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "phase-nine construction deletion proof failed because the compound ordering-scenario wrapper shelf came back: {violations:?}"
    );
}

#[test]
fn phase_nine_construction_compound_band_no_longer_teaches_deleted_specialized_report_shelf() {
    let compound_builder = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/compound_runtime/builder.rs"
    ));
    let compound_reports = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/compound_reports.rs"
    ));
    let compound_parity_reports = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/compound_parity_reports.rs"
    ));
    let violations = [
        (
            "worth-kernel.compound-builder",
            compound_builder,
            "prepare_primitive_construction_compound_motion_parity_report",
        ),
        (
            "worth-kernel.compound-builder",
            compound_builder,
            "prepare_primitive_construction_compound_grazing_boundary_report",
        ),
        (
            "worth-kernel.compound-builder",
            compound_builder,
            "prepare_primitive_construction_compound_exhaustion_witness_parity_report",
        ),
        (
            "worth-kernel.compound-tests-reports",
            compound_reports,
            "prepare_primitive_construction_compound_motion_parity_report",
        ),
        (
            "worth-kernel.compound-tests-reports",
            compound_reports,
            "prepare_primitive_construction_compound_grazing_boundary_report",
        ),
        (
            "worth-kernel.compound-tests-parity-reports",
            compound_parity_reports,
            "prepare_primitive_construction_compound_exhaustion_witness_parity_report",
        ),
    ]
    .into_iter()
    .filter_map(|(label, source, pattern)| {
        source
            .contains(pattern)
            .then(|| format!("{label}:{pattern}"))
    })
    .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "phase-nine construction deletion proof failed because the compound band reintroduced the deleted specialized motion/grazing/exhaustion report shelf: {violations:?}"
    );
}
