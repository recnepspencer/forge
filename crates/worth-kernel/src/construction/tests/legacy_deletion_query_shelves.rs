#[test]
fn phase_nine_construction_query_band_no_longer_teaches_deleted_scan_report_shelves() {
    let certification_mod = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/certification/mod.rs"
    ));
    let phase_five_boundary = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/certification/phase_five_boundary_closeout_tests.rs"
    ));
    let certification_bucket = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/certification/public_facade_contracts/compile_fail/reports/public_certification_bucket_not_exported.rs"
    ));
    let violations = [
        (
            "worth-kernel.certification-mod",
            certification_mod,
            "mod query;",
        ),
        (
            "worth-kernel.certification-mod",
            certification_mod,
            "mod existing_truth_binding;",
        ),
        (
            "worth-kernel.certification-mod",
            certification_mod,
            "mod no_local_runtime_workaround_audit;",
        ),
        (
            "worth-kernel.phase-five-boundary",
            phase_five_boundary,
            "prepare_primitive_construction_query_no_local_runtime_workaround_audit",
        ),
        (
            "worth-kernel.certification-bucket",
            certification_bucket,
            "PrimitiveConstructionQueryNoLocalRuntimeWorkaroundAudit",
        ),
        (
            "worth-kernel.certification-bucket",
            certification_bucket,
            "PrimitiveConstructionQueryExistingTruthBindingReport",
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
        "phase-nine construction deletion proof failed because the deleted query scan-report shelves came back: {violations:?}"
    );
}

#[test]
fn phase_nine_construction_query_band_no_longer_teaches_deleted_inspection_parity_shelf() {
    let certification_mod = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/certification/mod.rs"
    ));
    let construction_query_reports = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/construction_query_reports.rs"
    ));
    let simplex_ladder = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/corpus_simplex_ladder.rs"
    ));
    let compound_row_builder = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/compound_runtime/row_builder.rs"
    ));
    let violations = [
        (
            "worth-kernel.certification-mod",
            certification_mod,
            "mod query;",
        ),
        (
            "worth-kernel.certification-mod",
            certification_mod,
            "mod inspection_parity;",
        ),
        (
            "worth-kernel.construction-query-reports",
            construction_query_reports,
            "prepare_primitive_construction_query_inspection_parity_report",
        ),
        (
            "worth-kernel.simplex-ladder-tests",
            simplex_ladder,
            "prepare_primitive_construction_query_inspection_parity_report",
        ),
        (
            "worth-kernel.compound-row-builder",
            compound_row_builder,
            "prepare_primitive_construction_query_inspection_parity_report",
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
        "phase-nine construction deletion proof failed because the deleted inspection-parity shelf came back beside the surviving projection-consumption proof lane: {violations:?}"
    );
}

#[test]
fn phase_nine_construction_query_band_no_longer_teaches_deleted_graph_composition_parity_shelf() {
    let certification_mod = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/certification/mod.rs"
    ));
    let boundary_tests = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/boundary.rs"
    ));
    let boundary_sources = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/boundary_phase_five/sources.rs"
    ));
    let phase_five_boundary = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/certification/phase_five_boundary_closeout_tests.rs"
    ));
    let violations = [
        (
            "worth-kernel.certification-mod",
            certification_mod,
            "mod query;",
        ),
        (
            "worth-kernel.certification-mod",
            certification_mod,
            "mod graph_composition_parity;",
        ),
        (
            "worth-kernel.boundary-tests",
            boundary_tests,
            "graph_composition_parity.rs",
        ),
        (
            "worth-kernel.boundary-sources",
            boundary_sources,
            "graph_composition_parity.rs",
        ),
        (
            "worth-kernel.phase-five-boundary",
            phase_five_boundary,
            "graph_composition_parity.rs",
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
        "phase-nine construction deletion proof failed because the deleted graph-composition parity shelf came back instead of leaving ComposeGraph proof on the direct runtime/result surfaces: {violations:?}"
    );
}

#[test]
fn phase_nine_construction_query_band_no_longer_teaches_projection_consumption_report_wrapper() {
    let certification_mod = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/certification/mod.rs"
    ));
    let projection_support = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/projection_consumption.rs"
    ));
    let construction_query_reports = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/construction_query_reports.rs"
    ));
    let simplex_ladder = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/corpus_simplex_ladder.rs"
    ));
    let compound_row_builder = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/compound_runtime/row_builder.rs"
    ));
    let violations = [
        (
            "worth-kernel.certification-mod",
            certification_mod,
            "mod query;",
        ),
        (
            "worth-kernel.projection-support",
            projection_support,
            "PrimitiveConstructionQueryProjectionConsumptionParityReport",
        ),
        (
            "worth-kernel.projection-support",
            projection_support,
            "prepare_primitive_construction_query_projection_consumption_parity_report",
        ),
        (
            "worth-kernel.construction-query-reports",
            construction_query_reports,
            "prepare_primitive_construction_query_projection_consumption_parity_report",
        ),
        (
            "worth-kernel.simplex-ladder-tests",
            simplex_ladder,
            "prepare_primitive_construction_query_projection_consumption_parity_report",
        ),
        (
            "worth-kernel.compound-row-builder",
            compound_row_builder,
            "prepare_primitive_construction_query_projection_consumption_parity_report",
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
        "phase-nine construction deletion proof failed because the deleted projection-consumption report wrapper came back instead of leaving only direct runtime truth plus the query-surface digest helper: {violations:?}"
    );
}
