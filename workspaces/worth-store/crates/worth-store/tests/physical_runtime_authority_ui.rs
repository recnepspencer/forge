#[path = "physical_runtime_authority/bounded_physical_record_access_examples.rs"]
#[allow(
    dead_code,
    reason = "the same file is also a standalone trybuild binary"
)]
mod bounded_physical_record_access_examples;

#[path = "physical_runtime_authority_ui/document_examples.rs"]
mod document_examples;

use document_examples::assert_bounded_physical_record_access_examples_are_compile_bound;

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
    durability_policy_cases(&cases);
    record_chunk_view_cases(&cases);
}

fn durability_policy_cases(cases: &trybuild::TestCases) {
    cases.pass("tests/physical_runtime_authority/physical_wal_append_examples.rs");
    cases.compile_fail(
        "tests/physical_runtime_authority/incomplete_durability_policy_cannot_admit.rs",
    );
    cases.compile_fail(
        "tests/physical_runtime_authority/raw_backend_profile_cannot_admit_durability.rs",
    );
    cases.compile_fail(
        "tests/physical_runtime_authority/physical_durability_basis_cannot_be_duplicated.rs",
    );
    cases.compile_fail("tests/physical_runtime_authority/admitted_durability_policy_is_sealed.rs");
    cases.compile_fail(
        "tests/physical_runtime_authority/durability_policy_cannot_be_omitted_from_open.rs",
    );
    cases.compile_fail(
        "tests/physical_runtime_authority/physical_mutation_preparation_authority_is_sealed.rs",
    );
}

#[test]
fn phase_ten_c8_recovery_handoff_is_compile_sealed() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail(
        "tests/physical_runtime_authority/c8_recovery_handoff_constructor_is_sealed.rs",
    );
    cases.compile_fail("tests/physical_runtime_authority/c8_recovery_handoff_is_linear.rs");
    cases.compile_fail(
        "tests/physical_runtime_authority/c8_recovery_handoff_rejects_report_conversion.rs",
    );
    cases.compile_fail(
        "tests/physical_runtime_authority/c8_recovery_operation_fact_cannot_mint_handoff.rs",
    );
}

#[test]
fn phase_four_progression_requires_exact_predecessor_authority() {
    let cases = trybuild::TestCases::new();
    cases.pass("tests/physical_runtime_authority/physical_data_progression_examples.rs");
    cases.compile_fail(
        "tests/physical_runtime_authority/wal_durable_authority_requires_completed_barrier.rs",
    );
    cases.compile_fail("tests/physical_runtime_authority/physical_data_progression_is_sealed.rs");
}

#[test]
fn phase_six_checkpoint_authority_is_compile_bound() {
    let cases = trybuild::TestCases::new();
    cases
        .compile_fail("tests/physical_runtime_authority/checkpoint_capture_authority_is_sealed.rs");
}

#[test]
fn phase_seven_root_publication_ownership_is_compile_bound() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/physical_runtime_authority/root_publication_plans_are_linear.rs");

    let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let projection = std::fs::read_to_string(
        crate_root.join("src/physical_runtime/record_serving/planning/prepared_root_projection.rs"),
    )
    .unwrap();
    let planning = std::fs::read_to_string(crate_root.join(
        "src/physical_runtime/record_serving/publication/root_candidate/planning_members.rs",
    ))
    .unwrap();
    let submission = std::fs::read_to_string(
        crate_root.join("src/physical_runtime/record_serving/publication/director/submission.rs"),
    )
    .unwrap();
    let certification = std::fs::read_to_string(crate_root.join(
        "src/physical_runtime/record_serving/publication/director/certification_submission.rs",
    ))
    .unwrap();

    assert!(
        !projection.contains("#[derive(Clone)]"),
        "prepared root projection must remain linear"
    );
    assert!(
        !planning.contains("duplicate_candidate_projections"),
        "root planning must not retain a projection duplication lane"
    );
    assert!(
        !submission.contains("    pub fn continue_root_publication_candidate")
            && certification.contains("pub fn continue_root_publication_candidate")
            && certification
                .contains("candidate: crate::physical_runtime::RootPublicationCandidatePlan"),
        "candidate continuation must consume the exact plan only through certification"
    );
}

#[test]
fn phase_eight_ordinary_mutation_outcomes_are_compile_bound() {
    let cases = trybuild::TestCases::new();
    cases.pass("tests/physical_runtime_authority/ordinary_mutation_outcomes_are_supported.rs");
    cases.pass("tests/physical_runtime_authority/physical_durability_guide_examples.rs");
    cases.compile_fail(
        "tests/physical_runtime_authority/ordinary_mutation_phase_driving_is_absent.rs",
    );
    cases.compile_fail(
        "tests/physical_runtime_authority/noncompleted_mutation_cannot_acknowledge.rs",
    );
    cases.compile_fail(
        "tests/physical_runtime_authority/mutation_evidence_cannot_reenter_authority.rs",
    );
}

#[test]
fn phase_seven_root_candidate_failure_typestate_is_contract_bound() {
    let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let writes = std::fs::read_to_string(
        crate_root.join("src/physical_runtime/record_serving/publication/root_candidate_writes.rs"),
    )
    .unwrap();
    let execution = std::fs::read_to_string(crate_root.join(
        "src/physical_runtime/record_serving/publication/director/root_candidate_execution.rs",
    ))
    .unwrap();

    assert_root_candidate_failure_contract(&writes, &execution);
    let mutants = [
        (
            "partial candidate effect became retryable",
            writes.replace(
                "completed_artifacts.is_empty() && proves_no_effect(&cause)",
                "proves_no_effect(&cause)",
            ),
            execution.clone(),
        ),
        (
            "root writer discarded the recoverable frame",
            writes.replace(
                ".write_new_candidate_recoverable(",
                ".write_new_candidate(",
            ),
            execution.clone(),
        ),
        (
            "retryable candidate failure poisoned Store health",
            writes.clone(),
            execution.replacen(
                "let candidate = candidate_basis.restore_proven_no_effect(plan);",
                "candidate_basis.require_inspection();\n                runtime.health.revoke();\n                let candidate = candidate_basis.restore_proven_no_effect(plan);",
                1,
            ),
        ),
        (
            "possible candidate effect stopped revoking Store health",
            writes.clone(),
            execution.replacen("runtime.health.revoke();", "", 1),
        ),
    ];
    for (name, writes, execution) in mutants {
        assert!(
            !root_candidate_failure_contract(&writes, &execution),
            "controlled mutant survived: {name}"
        );
    }
}

#[test]
fn phase_seven_completion_observation_is_settlement_bound() {
    let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let observation = std::fs::read_to_string(
        crate_root.join("src/physical_runtime/record_serving/publication/append_observation.rs"),
    )
    .unwrap();
    let projection = std::fs::read_to_string(
        crate_root.join("src/physical_runtime/record_serving/planning/prepared_root_projection.rs"),
    )
    .unwrap();
    let planning = std::fs::read_to_string(crate_root.join(
        "src/physical_runtime/record_serving/publication/root_candidate/planning_members.rs",
    ))
    .unwrap();
    let data = std::fs::read_to_string(
        crate_root.join("src/physical_runtime/record_serving/publication/durable_data_plan.rs"),
    )
    .unwrap();

    assert!(completion_observation_contract(
        &observation,
        &projection,
        &planning,
        &data
    ));
    assert_completion_observation_mutants_rejected(&observation, &projection, &planning, &data);
}

fn assert_completion_observation_mutants_rejected(
    observation: &str,
    projection: &str,
    planning: &str,
    data: &str,
) {
    let mutants = [
        (
            "completion escaped exact settlement",
            planning.replacen(
                "projection.settle_data_observation(settled_basis.data_effects().len());",
                "",
                1,
            ),
            data.to_owned(),
            observation.to_owned(),
        ),
        (
            "settlement used an ambient transfer count",
            planning.replacen("settled_basis.data_effects().len()", "0", 1),
            data.to_owned(),
            observation.to_owned(),
        ),
        (
            "extent source copies became invisible",
            planning.to_owned(),
            data.replacen("observation.observe_copy(count);", "", 1),
            observation.to_owned(),
        ),
        (
            "settled completion stopped proving completed bytes",
            planning.to_owned(),
            data.to_owned(),
            observation.replacen("self.completed_bytes = self.logical_bytes;", "", 2),
        ),
    ];
    for (name, planning, data, observation) in mutants {
        assert!(
            !completion_observation_contract(&observation, projection, &planning, &data),
            "controlled mutant survived: {name}"
        );
    }
}

fn completion_observation_contract(
    observation: &str,
    projection: &str,
    planning: &str,
    data: &str,
) -> bool {
    let observation = normalized_rust(observation);
    let projection = normalized_rust(projection);
    let planning = normalized_rust(planning);
    let data = normalized_rust(data);
    observation.contains("fnsettle_data_effects")
        && observation.contains(
            "self.completed_bytes=self.logical_bytes;self.transfer_count=u64::try_from(effect_count).unwrap_or(u64::MAX);",
        )
        && projection.contains("self.observation.settle_data_effects(effect_count)")
        && planning.contains(
            "projection.settle_data_observation(settled_basis.data_effects().len());letcompletion=projection.completion_projection();",
        )
        && data.contains("observation.observe_copy(record_bytes.len())")
        && data.contains("observation.observe_copy(count)")
        && data.contains("observation.observe_scratch(bytes.len())")
        && data.contains("observation.observe_transfer(bytes.len())")
}

fn assert_root_candidate_failure_contract(writes: &str, execution: &str) {
    assert!(
        root_candidate_failure_contract(writes, execution),
        "root candidate failure typestate contract drifted"
    );
}

fn root_candidate_failure_contract(writes: &str, execution: &str) -> bool {
    let writes = normalized_rust(writes);
    let execution = normalized_rust(execution);
    writes.contains("RetryableNoEffect{")
        && writes.contains("InspectionRequired{")
        && writes.contains("completed_artifacts.is_empty()&&proves_no_effect(&cause)")
        && writes.contains(
            ".write_new_candidate_recoverable(RecordPublicationStage::ManifestSynchronization,residency,frame,artifact,)?",
        )
        && execution.contains("CandidateWriteNotStarted{")
        && execution.contains("candidate_basis.restore_proven_no_effect(plan)")
        && !execution.contains(
            "RetryableNoEffect{plan,failed_artifact,cause})=>{candidate_basis.require_inspection();runtime.health.revoke();",
        )
        && execution.contains(
            "InspectionRequired{plan,completed_artifacts,failed_artifact,cause})=>{candidate_basis.require_inspection();runtime.health.revoke();",
        )
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
