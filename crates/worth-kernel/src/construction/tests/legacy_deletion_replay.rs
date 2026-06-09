#[test]
fn phase_nine_construction_corpus_replay_band_no_longer_teaches_deleted_authoring_order_row_shelf()
{
    let certification_mod = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/certification/mod.rs"
    ));
    let replay_generation = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/corpus_replay_generation.rs"
    ));
    let violations = [
        (
            "worth-kernel.certification-mod",
            certification_mod,
            "mod corpus;",
        ),
        (
            "worth-kernel.corpus-replay-generation",
            replay_generation,
            "PrimitiveConstructionCorpusAuthoringOrderRow",
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
        "phase-nine construction deletion proof failed because the deleted corpus authoring-order row wrapper shelf came back: {violations:?}"
    );
}

#[test]
fn phase_nine_construction_corpus_replay_report_no_longer_teaches_stored_summary_field_bag() {
    let certification_mod = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/certification/mod.rs"
    ));
    let replay_generation = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/corpus_replay_generation.rs"
    ));
    let violations = [
        (
            "worth-kernel.certification-mod",
            certification_mod,
            "PrimitiveConstructionCorpusReplaySiegeReport",
        ),
        (
            "worth-kernel.corpus-replay-generation",
            replay_generation,
            "struct PrimitiveConstructionCorpusReplaySiegeReport",
        ),
        (
            "worth-kernel.corpus-replay-generation",
            replay_generation,
            "prepare_primitive_construction_corpus_replay_siege(",
        ),
        (
            "worth-kernel.corpus-replay-generation",
            replay_generation,
            "PrimitiveConstructionCorpusReplaySiegeReport::new(",
        ),
        (
            "worth-kernel.corpus-replay-generation",
            replay_generation,
            "accepted_count: usize",
        ),
        (
            "worth-kernel.corpus-replay-generation",
            replay_generation,
            "rejected_count: usize",
        ),
        (
            "worth-kernel.corpus-replay-generation",
            replay_generation,
            "required_scenario_coverage_verified: bool",
        ),
        (
            "worth-kernel.corpus-replay-generation",
            replay_generation,
            "row_digest_uniqueness_verified: bool",
        ),
        (
            "worth-kernel.corpus-replay-generation",
            replay_generation,
            "authoring_order_lane_coverage_verified: bool",
        ),
        (
            "worth-kernel.corpus-replay-generation",
            replay_generation,
            "authoring_order_digest_uniqueness_verified: bool",
        ),
        (
            "worth-kernel.corpus-replay-generation",
            replay_generation,
            "authoring_order_matrix_stability_verified: bool",
        ),
        (
            "worth-kernel.corpus-replay-generation",
            replay_generation,
            "report_digest: String",
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
        "phase-nine construction deletion proof failed because the replay-siege report reintroduced a stored summary-field bag instead of deriving those facts from canonical rows and lanes: {violations:?}"
    );
}

#[test]
fn phase_nine_construction_corpus_replay_band_no_longer_teaches_deleted_authoring_lane_wrapper_shelf(
) {
    let certification_mod = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/certification/mod.rs"
    ));
    let corpus_ordering = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/corpus_ordering.rs"
    ));
    let tests_support_mod = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/mod.rs"
    ));
    let violations = [
        (
            "worth-kernel.certification-mod",
            certification_mod,
            "mod corpus;",
        ),
        (
            "worth-kernel.corpus-ordering-support",
            corpus_ordering,
            "PrimitiveConstructionCorpusAuthoringLane",
        ),
        (
            "worth-kernel.tests-support-mod",
            tests_support_mod,
            "pub(crate) mod corpus_ordering;",
        ),
    ]
    .into_iter()
    .filter_map(|(label, source, pattern)| {
        if label == "worth-kernel.tests-support-mod" {
            (!source.contains(pattern)).then(|| format!("{label}:missing:{pattern}"))
        } else {
            source
                .contains(pattern)
                .then(|| format!("{label}:{pattern}"))
        }
    })
    .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "phase-nine construction deletion proof failed because the replay-side authoring-lane wrapper shelf came back: {violations:?}"
    );
}

#[test]
fn phase_nine_construction_corpus_replay_rows_no_longer_teach_cached_breadth_field_bag() {
    let replay_generation = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/corpus_replay_generation.rs"
    ));
    let replay_siege_row = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/corpus_replay_row.rs"
    ));
    let violations = [
        (
            "worth-kernel.corpus-replay-generation",
            replay_generation,
            "topology_fact_breadth,\n            topology_fact_breadth,\n            topology_fact_breadth,",
        ),
        (
            "worth-kernel.corpus-replay-generation",
            replay_generation,
            "0,\n            0,\n            0,",
        ),
        (
            "worth-kernel.corpus-replay-siege-row",
            replay_siege_row,
            "construction_breadth: usize",
        ),
        (
            "worth-kernel.corpus-replay-siege-row",
            replay_siege_row,
            "birth_attachment_breadth: usize",
        ),
        (
            "worth-kernel.corpus-replay-siege-row",
            replay_siege_row,
            "certification_breadth: usize",
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
        "phase-nine construction deletion proof failed because corpus replay rows reintroduced a cached breadth field bag instead of deriving breadth from canonical runtime truth: {violations:?}"
    );
}

#[test]
fn phase_nine_construction_corpus_replay_live_band_no_longer_teaches_cached_digest_bags() {
    let replay_siege_row = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/corpus_replay_row.rs"
    ));
    let violations = [
        (
            "worth-kernel.corpus-replay-siege-row",
            replay_siege_row,
            "row_digest: String",
        ),
        (
            "worth-kernel.corpus-replay-siege-row",
            replay_siege_row,
            "pub fn row_digest(&self) -> &str",
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
        "phase-nine construction deletion proof failed because the live replay band reintroduced cached row or lane digest bags instead of leaving replay digest rederivation in hostile test support: {violations:?}"
    );
}

#[test]
fn phase_nine_construction_replay_and_compound_builders_no_longer_self_compare_runtime_truth() {
    let replay_generation = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/corpus_replay_generation.rs"
    ));
    let compound_builder = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/compound_runtime/builder.rs"
    ));
    let compound_row_builder = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/compound_runtime/row_builder.rs"
    ));
    let violations = [
        (
            "worth-kernel.corpus-replay-generation",
            replay_generation,
            "ReplayParityDrift",
        ),
        (
            "worth-kernel.corpus-replay-generation",
            replay_generation,
            "prepare_primitive_construction_certification_runtime_truth(request.clone())",
        ),
        (
            "worth-kernel.corpus-replay-generation",
            replay_generation,
            "let replay_truth = prepare_primitive_construction_certification_runtime_truth(request);",
        ),
        (
            "worth-kernel.compound-builder",
            compound_builder,
            "ReplayParityDrift",
        ),
        (
            "worth-kernel.compound-row-builder",
            compound_row_builder,
            "prepare_primitive_construction_certification_runtime_truth(request.clone())",
        ),
        (
            "worth-kernel.compound-row-builder",
            compound_row_builder,
            "let replay_truth = prepare_primitive_construction_certification_runtime_truth(request);",
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
        "phase-nine construction deletion proof failed because replay/compound builders reintroduced production self-comparison of canonical runtime truth instead of using one owner computation: {violations:?}"
    );
}

#[test]
fn phase_nine_construction_corpus_replay_hostile_support_no_longer_lives_in_certification_tests_tree(
) {
    let replay_tests_mod = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/mod.rs"
    ));
    let replay_siege = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/corpus_replay_siege.rs"
    ));
    let tests_support_mod = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/mod.rs"
    ));
    let violations = [
        (
            "worth-kernel.corpus-tests-mod",
            replay_tests_mod,
            "mod corpus_replay_siege;",
        ),
        (
            "worth-kernel.corpus-tests-mod",
            replay_tests_mod,
            "mod corpus_replay_siege_rejections;",
        ),
        (
            "worth-kernel.corpus-tests-mod",
            replay_tests_mod,
            "mod corpus_realization_classes;",
        ),
        (
            "worth-kernel.corpus-tests-mod",
            replay_tests_mod,
            "mod corpus_family_boundaries;",
        ),
        (
            "worth-kernel.corpus-tests-mod",
            replay_tests_mod,
            "mod corpus_simplex_ladder;",
        ),
        (
            "worth-kernel.corpus-tests-replay-siege",
            replay_siege,
            "super::support::",
        ),
        (
            "worth-kernel.tests-support-mod",
            tests_support_mod,
            "pub(crate) mod corpus_replay_view;",
        ),
        (
            "worth-kernel.tests-support-mod",
            tests_support_mod,
            "pub(crate) mod corpus_replay_digest;",
        ),
        (
            "worth-kernel.tests-support-mod",
            tests_support_mod,
            "pub(crate) mod corpus_ordering;",
        ),
        (
            "worth-kernel.tests-support-mod",
            tests_support_mod,
            "pub(crate) mod corpus_replay_generation;",
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
        if label == "worth-kernel.corpus-tests-mod" || label == "worth-kernel.tests-support-mod" {
            (!source.contains(pattern)).then(|| format!("{label}:missing:{pattern}"))
        } else {
            source
                .contains(pattern)
                .then(|| format!("{label}:{pattern}"))
        }
    })
    .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "phase-nine construction deletion proof failed because replay hostile support drifted back into the certification test tree instead of staying in plain construction test support: {violations:?}"
    );
}
