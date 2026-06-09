#[test]
fn phase_nine_construction_proof_band_no_longer_teaches_deleted_substrate_closeout_shelf() {
    let proof_mod = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/proof/mod.rs"
    ));
    let proof_closeout_tests = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/certification/proof_substrate_closeout_tests.rs"
    ));
    let violations = [
        (
            "worth-kernel.proof-mod",
            proof_mod,
            "mod substrate_closeout_report;",
        ),
        (
            "worth-kernel.proof-closeout-tests",
            proof_closeout_tests,
            "prepare_primitive_construction_proof_substrate_closeout_report",
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
        "phase-nine construction deletion proof failed because the proof band reintroduced the deleted substrate closeout shelf: {violations:?}"
    );
}

#[test]
fn phase_nine_construction_proof_band_no_longer_teaches_deleted_metadata_shelves() {
    let proof_mod = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/proof/mod.rs"
    ));
    let proof_substrate_tests = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/certification/proof_substrate_closeout_tests.rs"
    ));
    let proof_metadata_tests = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/certification/proof_metadata_surface_tests.rs"
    ));
    let violations = [
        (
            "worth-kernel.proof-mod",
            proof_mod,
            "mod truth_projection_matrix;",
        ),
        (
            "worth-kernel.proof-mod",
            proof_mod,
            "mod verified_artifact_surface_report;",
        ),
        (
            "worth-kernel.proof-substrate-tests",
            proof_substrate_tests,
            "prepare_primitive_construction_truth_projection_matrix",
        ),
        (
            "worth-kernel.proof-substrate-tests",
            proof_substrate_tests,
            "prepare_primitive_construction_verified_artifact_surface_report",
        ),
    ]
    .into_iter()
    .filter_map(|(label, source, pattern)| {
        source
            .contains(pattern)
            .then(|| format!("{label}:{pattern}"))
    })
    .collect::<Vec<_>>();

    assert!(
        proof_metadata_tests.contains("mod truth_projection_matrix;")
            && proof_metadata_tests.contains("mod verified_artifact_surface_report;")
            && proof_metadata_tests.contains("mod substrate_closeout_report;"),
        "proof metadata shelf guard drifted; the test must explicitly deny the deleted proof metadata wrappers"
    );

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "phase-nine construction deletion proof failed because the proof band reintroduced the deleted proof metadata shelves: {violations:?}"
    );
}

#[test]
fn phase_nine_construction_certification_no_longer_teaches_deleted_closeout_shelf() {
    let certification_mod = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/certification/mod.rs"
    ));
    let boundary_tests = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/certification/phase_five_boundary_closeout_tests.rs"
    ));
    let violations = [
        (
            "worth-kernel.certification-mod",
            certification_mod,
            "mod closeout;",
        ),
        (
            "worth-kernel.phase-five-boundary-tests",
            boundary_tests,
            "prepare_primitive_construction_phase_five_boundary_closeout_report",
        ),
        (
            "worth-kernel.phase-five-boundary-tests",
            boundary_tests,
            "PrimitiveConstructionPhaseFiveBoundaryCloseoutKind",
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
        "phase-nine construction deletion proof failed because the certification band reintroduced the deleted closeout shelf: {violations:?}"
    );
}

#[test]
fn phase_nine_construction_realization_band_no_longer_teaches_deleted_bundle_shelf() {
    let certification_mod = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/certification/mod.rs"
    ));
    let realization_tests = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/realization_reports.rs"
    ));
    let violations = [
        (
            "worth-kernel.certification-mod",
            certification_mod,
            "mod realization;",
        ),
        (
            "worth-kernel.realization-tests",
            realization_tests,
            "prepare_primitive_construction_realization_report_bundle",
        ),
    ]
    .into_iter()
    .filter_map(|(label, source, pattern)| {
        source
            .contains(pattern)
            .then(|| format!("{label}:{pattern}"))
    })
    .collect::<Vec<_>>();

    assert!(
        realization_tests.contains("prepare_realization_snapshot"),
        "realization shared-view proof drifted; the tests should build lower reports directly from one realization snapshot"
    );

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "phase-nine construction deletion proof failed because the realization certification band reintroduced the deleted bundle shelf: {violations:?}"
    );
}

#[test]
fn phase_nine_construction_certification_no_longer_teaches_deleted_branch_local_or_replay_parity_shelves(
) {
    let certification_mod = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/certification/mod.rs"
    ));
    let branch_preview_basis = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/branch_preview_basis.rs"
    ));
    let proof_ingredients = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/branch_basis_digest.rs"
    ));
    let construction_query_reports = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/construction_query_reports.rs"
    ));
    let compile_fail_contracts = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/certification/public_facade_contracts/compile_fail_contracts.rs"
    ));
    let violations = [
        (
            "worth-kernel.certification-mod",
            certification_mod,
            "mod parity;",
        ),
        (
            "worth-kernel.certification-mod",
            certification_mod,
            "mod query;",
        ),
        (
            "worth-kernel.certification-mod",
            certification_mod,
            "mod basis_preview_parity;",
        ),
        (
            "worth-kernel.branch-preview-basis-support",
            branch_preview_basis,
            "PrimitiveConstructionReplayParityReport",
        ),
        (
            "worth-kernel.branch-preview-basis-support",
            branch_preview_basis,
            "prepare_primitive_construction_replay_parity_report",
        ),
        (
            "worth-kernel.corpus-proof-ingredients",
            proof_ingredients,
            "PrimitiveConstructionReplayParityReport",
        ),
        (
            "worth-kernel.corpus-proof-ingredients",
            proof_ingredients,
            "prepare_primitive_construction_replay_parity_report",
        ),
        (
            "worth-kernel.construction-query-reports",
            construction_query_reports,
            "prepare_primitive_construction_replay_parity_report",
        ),
        (
            "worth-kernel.compile-fail-contracts",
            compile_fail_contracts,
            "public_branch_local_parity_report_constructor_not_exported.rs",
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
        "phase-nine construction deletion proof failed because the deleted branch-local or replay parity shelves came back: {violations:?}"
    );
}

#[test]
fn phase_nine_construction_certification_no_longer_teaches_deleted_rejection_locality_report_shelf()
{
    let certification_mod = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/certification/mod.rs"
    ));
    let construction_tests = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/construction.rs"
    ));
    let compile_fail_fixture = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/certification/public_facade_contracts/compile_fail/reports/public_misclassified_surface_exports_demoted.rs"
    ));
    let violations = [
        (
            "worth-kernel.certification-mod",
            certification_mod,
            "mod rejection_locality;",
        ),
        (
            "worth-kernel.construction-tests",
            construction_tests,
            "prepare_primitive_construction_rejection_locality_report",
        ),
        (
            "worth-kernel.compile-fail-fixture",
            compile_fail_fixture,
            "PrimitiveConstructionRejectionLocalityReport",
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
        "phase-nine construction deletion proof failed because the deleted rejection-locality report shelf came back: {violations:?}"
    );
}

#[test]
fn phase_nine_construction_realization_band_no_longer_teaches_deleted_report_file_shelves() {
    let certification_mod = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/certification/mod.rs"
    ));
    let realization_support_mod = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/realization/mod.rs"
    ));
    let violations = [
        (
            "worth-kernel.certification-mod",
            certification_mod,
            "mod realization;",
        ),
        (
            "worth-kernel.realization-support-mod",
            realization_support_mod,
            "mod strategy_report;",
        ),
        (
            "worth-kernel.realization-support-mod",
            realization_support_mod,
            "mod stability_class_report;",
        ),
        (
            "worth-kernel.realization-support-mod",
            realization_support_mod,
            "mod conditioning_witness_report;",
        ),
        (
            "worth-kernel.realization-support-mod",
            realization_support_mod,
            "mod exhaustion_report;",
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
        "phase-nine construction deletion proof failed because the deleted realization report file shelves came back instead of staying quarantined in hostile test support: {violations:?}"
    );
}
