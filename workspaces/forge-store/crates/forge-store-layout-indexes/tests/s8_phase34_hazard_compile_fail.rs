mod compile_fail_support;

use compile_fail_support::assert_compile_fails_in_ui_dir;
use forge_store_layout_indexes::layout_certification::S8HazardProofTarget;
use forge_store_layout_indexes::layout_closeout::layout_closeout;

#[test]
fn every_compile_fail_hazard_executes_its_declared_harness_fixture() {
    let handoff = layout_closeout()
        .admit_s9_layout_handoff()
        .expect("canonical S.9 layout handoff");
    let mut targets = Vec::new();

    for row in handoff.inventory().rows() {
        let S8HazardProofTarget::CompileFail(target) = row.proof_target() else {
            continue;
        };
        assert!(
            !targets.contains(&target),
            "compile-fail target reused: {target:?}"
        );
        targets.push(target);
        let fixture_name = std::path::Path::new(target.fixture())
            .file_name()
            .and_then(|name| name.to_str())
            .expect("fixture path has a file name");
        assert_compile_fails_in_ui_dir(
            target.harness().ui_dir(),
            fixture_name,
            target.expected_stderr(),
            target.extern_crates(),
        );
    }

    assert_eq!(targets.len(), 5, "every compile-fail hazard has one target");
}

#[test]
fn lsm_compaction_authority_cannot_be_minted_or_admitted_from_claimed_rows() {
    assert_compile_fails_in_ui_dir(
        "phase34",
        "caller_cannot_mint_transition_fact.rs",
        &["private associated function", "new"],
        &[],
    );
    assert_compile_fails_in_ui_dir(
        "phase34",
        "raw_denial_cannot_mint_transition_fact.rs",
        &["no method named `production_transition`"],
        &[],
    );
    assert_compile_fails_in_ui_dir(
        "phase34",
        "caller_cannot_wrap_raw_denial_as_owner_outcome.rs",
        &["tuple variant `Denied` is private", "non_exhaustive"],
        &[],
    );
    assert_compile_fails_in_ui_dir(
        "phase34",
        "caller_cannot_construct_transitioned_result.rs",
        &["private field", "inner"],
        &[],
    );
    assert_compile_fails_in_ui_dir(
        "phase34",
        "lsm_compaction_product_fields_are_private.rs",
        &["private field"],
        &["forge_store_wal"],
    );
    assert_compile_fails_in_ui_dir(
        "phase34",
        "caller_cannot_admit_lsm_output_rows.rs",
        &["no method named `admit_compaction_product`"],
        &[],
    );
    assert_compile_fails_in_ui_dir(
        "phase34",
        "caller_cannot_mint_lsm_compaction_plan.rs",
        &["no `S8LsmCompactionPlan`"],
        &[],
    );
    assert_compile_fails_in_ui_dir(
        "phase34",
        "caller_cannot_execute_seeded_lsm_compaction.rs",
        &["cannot find function `execute_baseline_lsm_compaction`"],
        &["forge_store_wal"],
    );
    assert_compile_fails_in_ui_dir(
        "phase34",
        "caller_cannot_seed_lsm_execution_witness.rs",
        &["no function or associated item named `seeded`"],
        &["forge_store_wal"],
    );
    assert_compile_fails_in_ui_dir(
        "phase34",
        "caller_cannot_claim_raw_wal_as_executed.rs",
        &["no function or associated item named `from_executed_wal`"],
        &["forge_store_wal"],
    );
    assert_compile_fails_in_ui_dir(
        "phase34",
        "caller_cannot_construct_raw_lsm_request.rs",
        &["struct `BaselineLsmExecutionRequest` is private"],
        &["forge_store_wal"],
    );
    assert_compile_fails_in_ui_dir(
        "phase34",
        "caller_cannot_assemble_lsm_durable_inputs.rs",
        &["private field"],
        &["forge_store_wal"],
    );
    assert_compile_fails_in_ui_dir(
        "phase34",
        "caller_cannot_admit_loose_lsm_input_bag.rs",
        &["no method named `admit_baseline_lsm_inputs`"],
        &["forge_store_wal"],
    );
    assert_compile_fails_in_ui_dir(
        "phase34",
        "caller_cannot_relabel_durable_lsm_record_scope.rs",
        &["no method named `admit_baseline_lsm_record`"],
        &["forge_store_wal", "forge_store_security"],
    );
    assert_compile_fails_in_ui_dir(
        "phase34",
        "physical_cutover_cannot_publish_without_lsm_receipt.rs",
        &["no function or associated item named `publish`"],
        &["forge_store_physical_isolation"],
    );
    assert_compile_fails_in_ui_dir(
        "phase34",
        "caller_cannot_supply_durability_observation.rs",
        &["unresolved imports"],
        &["forge_store_physical_backend"],
    );
    assert_compile_fails_in_ui_dir(
        "phase34",
        "caller_cannot_select_lsm_compaction_records.rs",
        &["this method takes 2 arguments but 3 arguments were supplied"],
        &["forge_store_wal"],
    );
    assert_compile_fails_in_ui_dir(
        "phase34",
        "caller_cannot_open_detached_lsm_index.rs",
        &["this method takes 1 argument but 0 arguments were supplied"],
        &["forge_store_wal"],
    );
}
