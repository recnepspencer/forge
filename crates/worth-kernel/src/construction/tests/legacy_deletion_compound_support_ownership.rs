#[test]
fn phase_nine_construction_compound_live_band_no_longer_teaches_motion_or_grazing_digest_helpers() {
    let compound_row_builder = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/compound_runtime/row_builder.rs"
    ));
    let violations = [
        (
            "worth-kernel.compound-row-builder",
            compound_row_builder,
            "pub(super) fn requested_motion_digest(",
        ),
        (
            "worth-kernel.compound-row-builder",
            compound_row_builder,
            "pub(super) fn grazing_digest(",
        ),
        (
            "worth-kernel.compound-row-builder",
            compound_row_builder,
            "fn frame_normal(",
        ),
        (
            "worth-kernel.compound-row-builder",
            compound_row_builder,
            "fn admitted_angle_between(",
        ),
        (
            "worth-kernel.compound-row-builder",
            compound_row_builder,
            "fn admitted_distance(",
        ),
        (
            "worth-kernel.compound-row-builder",
            compound_row_builder,
            "fn numeric_error(",
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
        "phase-nine construction deletion proof failed because the live compound row-builder reintroduced motion/grazing digest helper shelves instead of keeping them in explicit hostile test support: {violations:?}"
    );
}

#[test]
fn phase_nine_construction_compound_hostile_support_no_longer_lives_in_certification_tests_tree() {
    let compound_tests_mod = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/mod.rs"
    ));
    let compound_mod = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/compound_runtime/mod.rs"
    ));
    let support_mod = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/tests/support/mod.rs"
    ));
    let violations = [
        ("worth-kernel.compound-mod", compound_mod, "mod support;"),
        (
            "worth-kernel.compound-tests-mod",
            compound_tests_mod,
            "mod compound_reports;",
        ),
        (
            "worth-kernel.compound-tests-mod",
            compound_tests_mod,
            "mod compound_parity_reports;",
        ),
        (
            "worth-kernel.compound-tests-mod",
            compound_tests_mod,
            "mod compound_closeout_reports;",
        ),
        (
            "worth-kernel.construction-test-support-mod",
            support_mod,
            "pub(crate) mod compound_corpus;",
        ),
        (
            "worth-kernel.construction-test-support-mod",
            support_mod,
            "pub(crate) mod compound_lowering;",
        ),
        (
            "worth-kernel.construction-test-support-mod",
            support_mod,
            "pub(crate) mod compound_row_support;",
        ),
        (
            "worth-kernel.construction-test-support-mod",
            support_mod,
            "pub(crate) mod compound_lane_support;",
        ),
        (
            "worth-kernel.construction-test-support-mod",
            support_mod,
            "pub(crate) mod compound_parity_support;",
        ),
        (
            "worth-kernel.construction-test-support-mod",
            support_mod,
            "pub(crate) mod compound_parity_view;",
        ),
        (
            "worth-kernel.construction-test-support-mod",
            support_mod,
            "pub(crate) mod compound_required_inventory;",
        ),
        (
            "worth-kernel.construction-test-support-mod",
            support_mod,
            "pub(crate) mod compound_specialized_rows;",
        ),
    ]
    .into_iter()
    .filter_map(|(label, source, pattern)| {
        if label == "worth-kernel.compound-tests-mod"
            || label == "worth-kernel.construction-test-support-mod"
        {
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
        "phase-nine construction deletion proof failed because compound hostile support drifted back into the certification tests tree or lost its plain construction test-support owner: {violations:?}"
    );
}
