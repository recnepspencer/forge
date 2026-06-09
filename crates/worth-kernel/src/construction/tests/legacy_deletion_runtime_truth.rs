#[test]
fn phase_nine_construction_parity_band_no_longer_teaches_bespoke_prepared_fact_carrier() {
    let branch_preview_basis = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/branch_preview_basis.rs"
    ));
    let proof_ingredients = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/branch_basis_digest.rs"
    ));
    let violations = [
        (
            "worth-kernel.branch-preview-basis-support",
            branch_preview_basis,
            "PrimitiveConstructionParityPreparedFacts",
        ),
        (
            "worth-kernel.branch-preview-basis-support",
            branch_preview_basis,
            "from_admitted_runtime_truth",
        ),
        (
            "worth-kernel.branch-preview-basis-support",
            branch_preview_basis,
            "from_rejected_runtime_truth",
        ),
        (
            "worth-kernel.corpus-proof-ingredients",
            proof_ingredients,
            "PrimitiveConstructionParityPreparedFacts",
        ),
        (
            "worth-kernel.corpus-proof-ingredients",
            proof_ingredients,
            "from_admitted_runtime_truth",
        ),
        (
            "worth-kernel.corpus-proof-ingredients",
            proof_ingredients,
            "from_rejected_runtime_truth",
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
        "phase-nine construction deletion proof failed because replay parity reintroduced a bespoke prepared-facts carrier instead of reading through shared certification runtime truth: {violations:?}"
    );
}

#[test]
fn phase_nine_construction_query_parity_band_no_longer_teaches_duplicated_runtime_truth_fields() {
    let projection_source = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/projection_consumption.rs"
    ));
    let duplicated_field_patterns = [
        "realization_strategy: Option<PrimitiveRealizationStrategy>",
        "attempted_realization_strategies: Vec<PrimitiveRealizationStrategy>",
        "stability_class: Option<PrimitiveStabilityClass>",
        "feature_conditioning_class: Option<PrimitiveFeatureConditioningClass>",
        "support_normal_class: Option<PrimitiveSupportNormalClass>",
        "normalization_disposition: Option<PrimitiveNormalizationDisposition>",
    ];
    let violations = duplicated_field_patterns
        .iter()
        .filter(|pattern| projection_source.contains(**pattern))
        .map(|pattern| format!("worth-kernel.query-projection-parity:{pattern}"))
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "phase-nine construction deletion proof failed because query parity reintroduced duplicated admitted runtime-truth field bags instead of reading through canonical owners: {violations:?}"
    );
}

#[test]
fn phase_nine_construction_family_coverage_shelf_no_longer_lives_in_certification_tree() {
    let certification_mod = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/certification/mod.rs"
    ));
    let construction_tests = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/construction.rs"
    ));
    let family_coverage_support = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/family_coverage.rs"
    ));
    let violations = [
        (
            "worth-kernel.certification-mod",
            certification_mod,
            "mod family_coverage;",
        ),
        (
            "worth-kernel.construction-tests",
            construction_tests,
            "certification::family_coverage",
        ),
        (
            "worth-kernel.family-coverage-support",
            family_coverage_support,
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
        "phase-nine construction deletion proof failed because the family-coverage shelf drifted back into the certification tree instead of staying in hostile test support: {violations:?}"
    );
}

#[test]
fn phase_nine_construction_corpus_execution_no_longer_teaches_bespoke_runtime_truth_bag() {
    let execution_source = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/branch_basis_digest.rs"
    ));
    let bespoke_field_patterns = [
        "admitted: bool",
        "outcome_digest: String",
        "birth_truth_digest: Option<String>",
        "realization_strategy: Option<PrimitiveRealizationStrategy>",
        "attempted_realization_strategies: Vec<PrimitiveRealizationStrategy>",
        "stability_class: Option<PrimitiveStabilityClass>",
        "feature_conditioning_class: Option<PrimitiveFeatureConditioningClass>",
        "support_normal_class: Option<PrimitiveSupportNormalClass>",
        "normalization_disposition: Option<PrimitiveNormalizationDisposition>",
        "topology_fact_breadth: Option<usize>",
        "placement_facts: Option<PrimitiveConstructionBirthPlacementFacts>",
        "exhaustion_reason: Option<PrimitiveRealizationExhaustionReason>",
        "rejection_class: Option<PrimitiveConstructionRejectionClass>",
        "rejection_locality: Option<PrimitiveConstructionRejectionLocality>",
        "blocking_boundary: Option<PrimitiveConstructionBlockingBoundary>",
        "replay_digest: String",
    ];
    let violations = bespoke_field_patterns
        .into_iter()
        .filter(|pattern| execution_source.contains(pattern))
        .map(|pattern| format!("worth-kernel.corpus-execution:{pattern}"))
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "phase-nine construction deletion proof failed because corpus execution reintroduced a bespoke runtime-truth bag instead of carrying canonical replay/runtime artifacts: {violations:?}"
    );
}

#[test]
fn phase_nine_construction_replay_siege_rows_no_longer_teach_copied_runtime_truth_fields() {
    let replay_row_source = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/corpus_replay_row.rs"
    ));
    let copied_field_patterns = [
        "PrimitiveConstructionCorpusOutcomeDisposition",
        "pub fn outcome_disposition(&self)",
        "direct_construction_digest: String",
        "pub fn direct_construction_digest(&self) -> &str",
        "replay_digest: String",
        "outcome_disposition: PrimitiveConstructionCorpusOutcomeDisposition",
        "pub fn birth_attachment_breadth(&self) -> usize",
        "pub fn certification_breadth(&self) -> usize",
        "pub fn birth_digest(&self) -> Option<&str>",
        "pub fn realization_strategy(&self) -> Option<PrimitiveRealizationStrategy>",
        "pub fn attempted_realization_strategies(&self) -> &[PrimitiveRealizationStrategy]",
        "pub fn attempted_realization_strategy_count(&self) -> usize",
        "pub fn stability_class(&self) -> Option<PrimitiveStabilityClass>",
        "pub fn feature_conditioning_class(&self) -> Option<PrimitiveFeatureConditioningClass>",
        "pub fn support_normal_class(&self) -> Option<PrimitiveSupportNormalClass>",
        "pub fn normalization_disposition(&self) -> Option<PrimitiveNormalizationDisposition>",
        "pub fn exhaustion_reason(&self) -> Option<PrimitiveRealizationExhaustionReason>",
        "pub fn rejection_class(&self) -> Option<PrimitiveConstructionRejectionClass>",
        "pub fn rejection_locality(&self) -> Option<PrimitiveConstructionRejectionLocality>",
        "pub fn blocking_boundary(&self) -> Option<PrimitiveConstructionBlockingBoundary>",
        "pub fn construction_breadth(&self) -> usize",
        "birth_digest: Option<String>",
        "realization_strategy: Option<PrimitiveRealizationStrategy>",
        "attempted_realization_strategies: Vec<PrimitiveRealizationStrategy>",
        "stability_class: Option<PrimitiveStabilityClass>",
        "feature_conditioning_class: Option<PrimitiveFeatureConditioningClass>",
        "support_normal_class: Option<PrimitiveSupportNormalClass>",
        "normalization_disposition: Option<PrimitiveNormalizationDisposition>",
        "exhaustion_reason: Option<PrimitiveRealizationExhaustionReason>",
        "rejection_class: Option<PrimitiveConstructionRejectionClass>",
        "rejection_locality: Option<PrimitiveConstructionRejectionLocality>",
        "blocking_boundary: Option<PrimitiveConstructionBlockingBoundary>",
    ];
    let violations = copied_field_patterns
        .into_iter()
        .filter(|pattern| replay_row_source.contains(pattern))
        .map(|pattern| format!("worth-kernel.corpus-replay-siege-row:{pattern}"))
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "phase-nine construction deletion proof failed because replay-siege rows reintroduced a copied runtime-truth field bag instead of carrying canonical runtime truth directly: {violations:?}"
    );
}

#[test]
fn phase_nine_construction_compound_rows_no_longer_teach_copied_runtime_truth_fields() {
    let compound_rows_source = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/compound_runtime/rows/siege_row.rs"
    ));
    let copied_field_patterns = [
        "direct_digest: String",
        "replay_digest: String",
        "outcome_digest: String",
        "branch_basis_digest: String",
        "query_surface_digest: Option<String>",
        "birth_attachment_breadth: usize",
        "certification_breadth: usize",
        "runtime_truth: Option<PrimitiveConstructionCertificationRuntimeTruth>",
        "pub fn birth_attachment_breadth(&self) -> usize",
        "pub fn certification_breadth(&self) -> usize",
        "pub fn outcome_digest(&self) -> &str",
        "pub fn branch_basis_digest(&self) -> &str",
        "pub fn query_surface_digest(&self) -> Option<&str>",
        "realization_strategy: Option<PrimitiveRealizationStrategy>",
        "attempted_realization_strategies: Vec<PrimitiveRealizationStrategy>",
        "stability_class: Option<PrimitiveStabilityClass>",
        "feature_conditioning_class: Option<PrimitiveFeatureConditioningClass>",
        "support_normal_class: Option<PrimitiveSupportNormalClass>",
        "normalization_disposition: Option<PrimitiveNormalizationDisposition>",
        "exhaustion_reason: Option<PrimitiveRealizationExhaustionReason>",
        "rejection_class: Option<PrimitiveConstructionRejectionClass>",
        "rejection_locality: Option<PrimitiveConstructionRejectionLocality>",
        "blocking_boundary: Option<PrimitiveConstructionBlockingBoundary>",
        "pub fn realization_strategy(&self) -> Option<PrimitiveRealizationStrategy>",
        "pub fn attempted_realization_strategies(&self) -> &[PrimitiveRealizationStrategy]",
        "pub fn stability_class(&self) -> Option<PrimitiveStabilityClass>",
        "pub fn exhaustion_reason(&self) -> Option<PrimitiveRealizationExhaustionReason>",
        "pub fn rejection_class(&self) -> Option<PrimitiveConstructionRejectionClass>",
        "pub fn rejection_locality(&self) -> Option<PrimitiveConstructionRejectionLocality>",
        "pub fn blocking_boundary(&self) -> Option<PrimitiveConstructionBlockingBoundary>",
        "motion_kind: Option<PrimitiveConstructionCompoundMotionKind>",
        "motion_digest: Option<String>",
        "grazing_kind: Option<PrimitiveConstructionCompoundGrazingKind>",
        "grazing_digest: Option<String>",
        "construction_breadth: usize",
        "pub fn motion_kind(&self) -> Option<PrimitiveConstructionCompoundMotionKind>",
        "pub fn motion_digest(&self) -> Option<&str>",
        "pub fn grazing_kind(&self) -> Option<PrimitiveConstructionCompoundGrazingKind>",
        "pub fn grazing_digest(&self) -> Option<&str>",
        "row_digest: String",
        "pub fn row_digest(&self) -> &str",
    ];
    let violations = copied_field_patterns
        .into_iter()
        .filter(|pattern| compound_rows_source.contains(pattern))
        .map(|pattern| format!("worth-kernel.compound-rows:{pattern}"))
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "phase-nine construction deletion proof failed because compound rows reintroduced a copied runtime-truth field bag instead of reading through canonical execution proof ingredients: {violations:?}"
    );
}
