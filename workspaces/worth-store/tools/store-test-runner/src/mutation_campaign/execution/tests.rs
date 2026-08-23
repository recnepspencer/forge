use super::{
    actual_failing_predicate, build_command, compiler_diagnostics, executed_test_count,
    execution_class, nested_executable, test_binary, MutationExecutionClass,
};
use crate::mutation_campaign::catalog::{ControlledMutation, MutationTarget};
use crate::mutation_campaign::target_directory::MutationCampaignTarget;

#[test]
fn mutation_causality_requires_one_runtime_predicate_marker() {
    assert_eq!(
        actual_failing_predicate("panic: C5_PREDICATE:page-layout", 6).unwrap(),
        "page-layout"
    );
    assert!(actual_failing_predicate("unrelated panic", 6).is_err());
    assert!(
        actual_failing_predicate("C5_PREDICATE:page-layout C5_PREDICATE:batch-atomicity", 6,)
            .is_err()
    );
    assert_eq!(
        actual_failing_predicate("panic: C5_PREDICATE:local-physical-work-scheduler", 43,).unwrap(),
        "local-physical-work-scheduler"
    );
    assert_eq!(
        actual_failing_predicate(
            "panic: MUTANT_PREDICATE:stale-residency-generation-consumed",
            49,
        )
        .unwrap(),
        "stale-residency-generation-consumed"
    );
    assert!(actual_failing_predicate(
        "C5_PREDICATE:page-layout MUTANT_PREDICATE:foreign-dirty-frame-claimed",
        50,
    )
    .is_err());
}

#[test]
fn repeated_nested_process_marker_is_one_causal_predicate() {
    let output = "child C5_PREDICATE:current-truth\nparent MUTANT_PREDICATE:current-truth";
    assert_eq!(
        actual_failing_predicate(output, 8).unwrap(),
        "current-truth"
    );
}

#[test]
fn filtered_test_summaries_must_prove_a_test_executed() {
    assert_eq!(
        executed_test_count(
            "running 1 test\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 12 filtered out"
        ),
        Some(1)
    );
    assert_eq!(
        executed_test_count(
            "running 1 test\ntest result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 12 filtered out"
        ),
        Some(1)
    );
    assert_eq!(
        executed_test_count(
            "running 0 tests\ntest result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out"
        ),
        Some(0)
    );
    assert_eq!(executed_test_count("compiler output only"), None);
}

#[test]
fn cargo_json_binds_the_executed_binary_without_platform_text_parsing() {
    let output =
        br#"{"reason":"compiler-artifact","executable":"C:\\target\\debug\\deps\\proof.exe"}
{"reason":"build-finished","success":true}"#;
    assert_eq!(
        test_binary(output).unwrap(),
        std::path::PathBuf::from(r"C:\target\debug\deps\proof.exe")
    );
}

#[test]
fn nested_execution_binds_the_actual_child_binary_from_json() {
    let output =
        r#"CONTROLLED_MUTATION_EXECUTABLE "C:\\target\\physical_store_work_courtroom.exe""#;
    assert_eq!(
        nested_executable(output).unwrap(),
        std::path::PathBuf::from(r"C:\target\physical_store_work_courtroom.exe")
    );
    assert!(nested_executable("").is_err());
    assert!(nested_executable(&format!("{output}\n{output}")).is_err());
}

#[test]
fn execution_cost_class_is_closed_over_target_topology() {
    assert_eq!(
        execution_class(MutationTarget::NestedExecutableLibrary {
            features: "physical-work-evidence",
        }),
        MutationExecutionClass::NestedExecutableCold
    );
    for target in [
        MutationTarget::Library,
        MutationTarget::LibraryWithFeatures {
            features: "physical-work-evidence",
        },
        MutationTarget::Binary("proof"),
        MutationTarget::Integration("journey"),
    ] {
        assert_eq!(
            execution_class(target),
            MutationExecutionClass::IsolatedCampaign
        );
    }
    assert_eq!(
        execution_class(MutationTarget::Integration("phase_eight_process")),
        MutationExecutionClass::FreshProcessCold
    );
}

#[test]
fn cargo_json_preserves_compiler_diagnostics_ahead_of_trailing_artifacts() {
    let diagnostic = r#"{"reason":"compiler-message","message":{"message":"missing authority","rendered":"error[E0425]: cannot find value `authority`\n"}}"#;
    let mut output = format!("not-json\n{diagnostic}\n");
    for ordinal in 0..40 {
        output.push_str(&format!(
            "{{\"reason\":\"compiler-artifact\",\"target\":{{\"name\":\"artifact-{ordinal}\"}}}}\n"
        ));
    }
    assert_eq!(
        compiler_diagnostics(&output).unwrap(),
        "error[E0425]: cannot find value `authority`"
    );
}

#[test]
fn cargo_json_diagnostic_extraction_ignores_malformed_and_empty_messages() {
    let output = r#"not-json
{"reason":"compiler-message","message":{"message":"","rendered":""}}
{"reason":"build-finished","success":false}"#;
    assert!(compiler_diagnostics(output).is_none());
}

#[test]
fn mutation_execution_receives_the_campaign_target_instead_of_workspace_target() {
    let workspace = std::env::temp_dir().join(format!(
        "worth-store-mutation-command-test-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&workspace);
    std::fs::create_dir(&workspace).unwrap();
    let target = MutationCampaignTarget::allocate_at(&workspace.join("target")).unwrap();
    assert!(target.path().starts_with(workspace.join("target")));
    assert_ne!(target.path(), workspace.join("target").as_path());
    let path = target.path().to_owned();
    target.close().unwrap();
    assert!(!path.exists());
    std::fs::remove_dir_all(workspace).unwrap();
}

#[test]
fn mutation_command_binds_the_exclusive_campaign_target_environment() {
    let workspace = std::env::temp_dir().join(format!(
        "worth-store-mutation-command-environment-test-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&workspace);
    std::fs::create_dir(&workspace).unwrap();
    let target = MutationCampaignTarget::allocate_at(&workspace.join("target")).unwrap();
    let mutation = ControlledMutation {
        id: 156,
        predicate: "target-environment",
        source: "fixture.rs",
        needle: "before",
        replacement: "after",
        package: "store-test-runner",
        target: MutationTarget::Library,
        selector: "mutation_campaign::execution::tests::mutation_command_binds_the_exclusive_campaign_target_environment",
    };

    let command = build_command(&workspace, &mutation, &target);
    let target_environment = command
        .get_envs()
        .find(|(key, _)| *key == std::ffi::OsStr::new("CARGO_TARGET_DIR"))
        .and_then(|(_, value)| value)
        .unwrap_or_else(|| {
            panic!("MUTANT_PREDICATE:c8-mutation-target-isolation missing target environment")
        });

    if target_environment != target.path().as_os_str()
        || target_environment == workspace.join("target").as_os_str()
    {
        panic!(
            "MUTANT_PREDICATE:c8-mutation-target-isolation bound {:?}, expected {:?}",
            target_environment,
            target.path()
        );
    }
    let target_path = target.path().to_owned();
    target.close().unwrap();
    assert!(!target_path.exists());
    std::fs::remove_dir_all(workspace).unwrap();
}
