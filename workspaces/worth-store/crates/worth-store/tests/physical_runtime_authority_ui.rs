#[path = "physical_runtime_authority/bounded_physical_record_access_examples.rs"]
#[allow(
    dead_code,
    reason = "the same file is also a standalone trybuild binary"
)]
mod bounded_physical_record_access_examples;

#[test]
fn external_consumers_cannot_forge_or_duplicate_runtime_authority() {
    assert_bounded_physical_record_access_examples_are_compile_bound();
    bounded_physical_record_access_examples::run_configuration_examples();
    let cases = trybuild::TestCases::new();
    cases.pass("tests/physical_runtime_authority/supported_admission.rs");
    cases.pass("tests/physical_runtime_authority/supported_physical_work.rs");
    cases.pass("tests/physical_runtime_authority/admitted_residency_policy_supported.rs");
    cases.pass("tests/physical_runtime_authority/responsibility_named_store_facade_supported.rs");
    cases.compile_fail(
        "tests/physical_runtime_authority/runtime_duplication_and_reconstruction_are_sealed.rs",
    );
    cases.compile_fail(
        "tests/physical_runtime_authority/internal_composition_construction_is_sealed.rs",
    );
    cases.compile_fail("tests/physical_runtime_authority/internal_runtime_topology_is_sealed.rs");
    cases.compile_fail("tests/physical_runtime_authority/non_authority_values_cannot_admit.rs");
    cases.compile_fail(
        "tests/physical_runtime_authority/wrong_phase_and_physical_operations_are_absent.rs",
    );
    cases.compile_fail("tests/physical_runtime_authority/maximal_feature_profile_cannot_admit.rs");
    cases.pass(
        "tests/physical_runtime_authority/independent_scan_and_mutation_capabilities_compile.rs",
    );
    cases.compile_fail("tests/physical_runtime_authority/frame_view_cannot_outlive_lease.rs");
    cases.compile_fail("tests/physical_runtime_authority/lower_clean_authority_is_required.rs");
    cases.compile_fail(
        "tests/physical_runtime_authority/physical_receipt_construction_is_sealed.rs",
    );
    cases.compile_fail("tests/physical_runtime_authority/physical_work_identity_is_sealed.rs");
    cases.compile_fail("tests/physical_runtime_authority/physical_work_progression_is_sealed.rs");
    cases.compile_fail(
        "tests/physical_runtime_authority/untyped_physical_work_basis_is_rejected.rs",
    );
    cases.compile_fail(
        "tests/physical_runtime_authority/borrowed_physical_work_submission_is_rejected.rs",
    );
    cases.compile_fail(
        "tests/physical_runtime_authority/legacy_mutation_and_writeback_routes_are_absent.rs",
    );
    cases.compile_fail(
        "tests/physical_runtime_authority/residency_writeback_internals_are_sealed.rs",
    );
    cases
        .compile_fail("tests/physical_runtime_authority/raw_residency_policy_cannot_enter_open.rs");
    cases.compile_fail("tests/physical_runtime_authority/admitted_residency_policy_is_sealed.rs");
    record_chunk_view_cases(&cases);
}

fn record_chunk_view_cases(cases: &trybuild::TestCases) {
    cases.pass("tests/physical_runtime_authority/bounded_physical_record_access_examples.rs");
    cases.pass("tests/physical_runtime_authority/record_chunk_views_supported.rs");
    cases.compile_fail(
        "tests/physical_runtime_authority/record_chunk_view_cannot_escape_session.rs",
    );
    cases.compile_fail(
        "tests/physical_runtime_authority/record_chunk_view_blocks_session_progress.rs",
    );
    cases.compile_fail("tests/physical_runtime_authority/record_chunk_view_blocks_session_drop.rs");
    cases.compile_fail(
        "tests/physical_runtime_authority/record_chunk_bytes_retain_session_borrow.rs",
    );
    cases.compile_fail(
        "tests/physical_runtime_authority/record_chunk_view_construction_is_sealed.rs",
    );
    cases.compile_fail(
        "tests/physical_runtime_authority/record_chunk_view_exposes_no_pool_authority.rs",
    );
    cases
        .compile_fail("tests/physical_runtime_authority/opened_physical_record_alias_is_absent.rs");
}

fn assert_bounded_physical_record_access_examples_are_compile_bound() {
    let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let document = std::fs::read_to_string(
        crate_root.join("../../../../_docs/worth-store/bounded-physical-record-access.md"),
    )
    .unwrap();
    let specimen = std::fs::read_to_string(
        crate_root
            .join("tests/physical_runtime_authority/bounded_physical_record_access_examples.rs"),
    )
    .unwrap();
    let specimen = normalized_rust(&specimen);
    let blocks = document
        .split("```rust")
        .skip(1)
        .map(|tail| tail.split("```").next().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(
        blocks.len(),
        4,
        "every public Rust block must be inventoried"
    );
    for block in blocks {
        let normalized = normalized_rust(block);
        assert!(
            specimen.contains(&normalized),
            "a public Rust block drifted from its compiler specimen:\n{block}",
        );
    }
}

fn normalized_rust(source: &str) -> String {
    let mut normalized = String::new();
    for (index, segment) in source.split('"').enumerate() {
        if index > 0 {
            normalized.push('"');
        }
        if index % 2 == 0 {
            normalized.extend(
                segment
                    .chars()
                    .filter(|character| !character.is_whitespace()),
            );
        } else {
            normalized.push_str(segment);
        }
    }
    normalized.replace(",}", "}")
}
