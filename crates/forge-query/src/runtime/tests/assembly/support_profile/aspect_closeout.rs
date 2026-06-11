use super::super::super::support::*;

const ASPECT_API_CLOSEOUT_DOC: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../_docs/forge-query/aspect-api-finalization-closeout.md"
));

#[test]
fn runtime_public_aspect_api_finalization_closeout_answers_substrate_handoff_questions() {
    let workspace = stateful_bridge_task_runtime()
        .workspace("task.aspect-api-closeout")
        .expect("task runtime should open a named workspace");
    let closeout = workspace.public_aspect_api_finalization_closeout();
    let report = workspace.public_mutation_surface_report();
    let matrix = workspace.public_support_matrix();
    let naming = ForgeQueryRuntime::public_api_naming_contract();

    assert_eq!(
        closeout.backend_posture(),
        ForgeQueryRuntimeBackendPosture::Primary
    );
    assert_eq!(
        closeout.support_matrix_digest(),
        matrix.matrix_digest().as_str()
    );
    assert_eq!(closeout.mutation_surface_digest(), report.report_digest());
    assert_eq!(closeout.naming_contract_digest(), naming.contract_digest());
    assert!(closeout
        .preferred_stable_surfaces()
        .iter()
        .any(|row| row == "workspace.insert(...)"));
    assert!(closeout
        .lower_level_stable_surfaces()
        .iter()
        .any(|row| row == "workspace.write(...)=>workspace.insert/update/delete/batch"));
    assert!(closeout
        .support_gated_surfaces()
        .iter()
        .any(|row| row == "workspace.intent(...)"));
}

#[test]
fn runtime_public_aspect_api_finalization_closeout_document_matches_certified_contract() {
    let workspace = stateful_bridge_task_runtime()
        .workspace("task.aspect-api-closeout-doc")
        .expect("task runtime should open a named workspace");
    let closeout = workspace.public_aspect_api_finalization_closeout();

    for required in [
        "`workspace.insert(...)`",
        "`workspace.update(...)`",
        "`workspace.delete(...)`",
        "`workspace.batch(...)`",
        "`workspace.write(...)`",
        "`workspace.intent(...)`",
        "`workspace.public_mutation_surface_report()`",
    ] {
        assert!(
            ASPECT_API_CLOSEOUT_DOC.contains(required),
            "closeout doc must include `{required}`"
        );
    }
    assert!(
        !ASPECT_API_CLOSEOUT_DOC.contains("public_mutation_api_compatibility_report"),
        "closeout doc must not teach the deleted compatibility report name"
    );

    for line in closeout.safe_to_build_now() {
        assert!(ASPECT_API_CLOSEOUT_DOC.contains(line));
    }
    for line in closeout.must_not_assume_yet() {
        assert!(ASPECT_API_CLOSEOUT_DOC.contains(line));
    }
    for line in closeout.migration_guidance() {
        assert!(ASPECT_API_CLOSEOUT_DOC.contains(line));
    }
    for command in closeout.required_verification_commands() {
        assert!(ASPECT_API_CLOSEOUT_DOC.contains(command));
    }
    assert!(
        ASPECT_API_CLOSEOUT_DOC.contains("JSON may still exist as an internal lowering adapter")
    );
}
