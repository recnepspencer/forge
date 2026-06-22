#[test]
fn phase_nine_construction_compound_band_no_longer_teaches_deleted_authoring_lane_wrapper_shelf() {
    let compound_mod = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/compound_runtime/mod.rs"
    ));
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
            "worth-kernel.compound-mod",
            compound_mod,
            "mod authoring_lane;",
        ),
        (
            "worth-kernel.compound-builder",
            compound_builder,
            "PrimitiveConstructionCompoundAuthoringLane",
        ),
        (
            "worth-kernel.compound-tests-reports",
            compound_reports,
            "PrimitiveConstructionCompoundAuthoringLane",
        ),
        (
            "worth-kernel.compound-tests-parity-reports",
            compound_parity_reports,
            "PrimitiveConstructionCompoundAuthoringLane",
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
        "phase-nine construction deletion proof failed because the compound-side authoring-lane wrapper shelf came back: {violations:?}"
    );
}

#[test]
fn phase_nine_construction_compound_rows_no_longer_teach_fake_three_breadth_taxonomy() {
    let compound_row_builder = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/compound_runtime/row_builder.rs"
    ));
    let compound_row = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/compound_runtime/rows/siege_row.rs"
    ));
    let violations = [
        (
            "worth-kernel.compound-row-builder",
            compound_row_builder,
            "topology_fact_breadth,\n            topology_fact_breadth,\n            topology_fact_breadth,",
        ),
        (
            "worth-kernel.compound-row-builder",
            compound_row_builder,
            "0,\n            0,\n            0,",
        ),
        (
            "worth-kernel.compound-row-builder",
            compound_row_builder,
            "rows.iter().map(|row| row.birth_attachment_breadth()).sum(),",
        ),
        (
            "worth-kernel.compound-row-builder",
            compound_row_builder,
            "rows.iter().map(|row| row.certification_breadth()).sum(),",
        ),
        (
            "worth-kernel.compound-row",
            compound_row,
            "birth_attachment_breadth: usize",
        ),
        (
            "worth-kernel.compound-row",
            compound_row,
            "certification_breadth: usize",
        ),
        (
            "worth-kernel.compound-row",
            compound_row,
            "pub fn birth_attachment_breadth(&self) -> usize",
        ),
        (
            "worth-kernel.compound-row",
            compound_row,
            "pub fn certification_breadth(&self) -> usize",
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
        "phase-nine construction deletion proof failed because compound rows reintroduced a fake three-breadth taxonomy instead of one canonical breadth owner: {violations:?}"
    );
}

#[test]
fn phase_nine_construction_compound_live_band_no_longer_teaches_synthetic_mixed_batch_row() {
    let compound_row_builder = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/compound_runtime/row_builder.rs"
    ));
    let compound_schema = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/compound_runtime/schema.rs"
    ));
    let compound_registry = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/compound_runtime/parity/registry.rs"
    ));
    let violations = [
        (
            "worth-kernel.compound-row-builder",
            compound_row_builder,
            "mixed_topology_batch_row(",
        ),
        (
            "worth-kernel.compound-row-builder",
            compound_row_builder,
            "mixed_topology_class_batch",
        ),
        (
            "worth-kernel.compound-schema",
            compound_schema,
            "MixedTopologyClassBatch",
        ),
        (
            "worth-kernel.compound-schema",
            compound_schema,
            "MixedBatch",
        ),
        (
            "worth-kernel.compound-schema",
            compound_schema,
            "MixedTopologyBatch",
        ),
        (
            "worth-kernel.compound-parity-registry",
            compound_registry,
            "mixed_topology_class_batch",
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
        "phase-nine construction deletion proof failed because the compound band reintroduced a synthetic mixed-topology batch row instead of sticking to direct scenario runtime truth: {violations:?}"
    );
}

#[test]
fn phase_nine_construction_compound_parity_band_no_longer_teaches_bundle_wrapper_shelf() {
    let compound_builder = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/compound_runtime/builder.rs"
    ));
    let compound_parity_mod = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/compound_runtime/parity/mod.rs"
    ));
    let compound_parity_reports = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/compound_parity_reports.rs"
    ));
    let violations = [
        (
            "worth-kernel.compound-builder",
            compound_builder,
            "PrimitiveConstructionCompoundParityReportBundle",
        ),
        (
            "worth-kernel.compound-builder",
            compound_builder,
            "PrimitiveConstructionCompoundParityReport",
        ),
        (
            "worth-kernel.compound-builder",
            compound_builder,
            "PrimitiveConstructionCompoundParityVerificationFailure",
        ),
        (
            "worth-kernel.compound-builder",
            compound_builder,
            "prepare_primitive_construction_compound_parity_report(",
        ),
        (
            "worth-kernel.compound-builder",
            compound_builder,
            "verify_bundle(PrimitiveConstructionCompoundParityReportBundle::new(",
        ),
        (
            "worth-kernel.compound-parity-mod",
            compound_parity_mod,
            "PrimitiveConstructionCompoundParityReportBundle",
        ),
        (
            "worth-kernel.compound-parity-mod",
            compound_parity_mod,
            "verify_bundle",
        ),
        (
            "worth-kernel.compound-parity-mod",
            compound_parity_mod,
            "PrimitiveConstructionCompoundParityReport",
        ),
        (
            "worth-kernel.compound-parity-mod",
            compound_parity_mod,
            "PrimitiveConstructionCompoundParityVerificationFailure",
        ),
        (
            "worth-kernel.compound-parity-mod",
            compound_parity_mod,
            "PrimitiveConstructionCompoundParityVerificationMismatch",
        ),
        (
            "worth-kernel.compound-parity-mod",
            compound_parity_mod,
            "mod bundle_verified;",
        ),
        (
            "worth-kernel.compound-tests-parity-reports",
            compound_parity_reports,
            "PrimitiveConstructionCompoundParityReportBundle",
        ),
        (
            "worth-kernel.compound-tests-parity-reports",
            compound_parity_reports,
            "prepare_primitive_construction_compound_parity_report(",
        ),
        (
            "worth-kernel.compound-tests-parity-reports",
            compound_parity_reports,
            "verify_bundle(",
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
        "phase-nine construction deletion proof failed because the compound parity bundle wrapper shelf came back: {violations:?}"
    );
}

#[test]
fn phase_nine_construction_compound_live_band_no_longer_teaches_specialized_row_helper_shelf() {
    let compound_builder = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/compound_runtime/builder.rs"
    ));
    let compound_mod = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/compound_runtime/mod.rs"
    ));
    let violations = [
        (
            "worth-kernel.compound-builder",
            compound_builder,
            "build_motion_parity_rows_from_siege(",
        ),
        (
            "worth-kernel.compound-builder",
            compound_builder,
            "build_grazing_boundary_rows_from_siege(",
        ),
        (
            "worth-kernel.compound-builder",
            compound_builder,
            "build_exhaustion_witness_parity_rows_from_siege(",
        ),
        (
            "worth-kernel.compound-builder",
            compound_builder,
            "derive_specialized_rows(",
        ),
        (
            "worth-kernel.compound-builder",
            compound_builder,
            "require_specialized_row_field(",
        ),
        ("worth-kernel.compound-mod", compound_mod, "mod lanes;"),
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
        "phase-nine construction deletion proof failed because live compound certification reintroduced the specialized-row helper shelf instead of keeping it in explicit test support: {violations:?}"
    );
}
