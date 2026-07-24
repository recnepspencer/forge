use worth_proof::TransitionOutcome;
use worth_query::facade::domain;

use super::installed_operation_fixture::{
    artifact_move_workspace, artifact_workspace_without_support, bind_artifact_workflow,
    move_intent,
};

#[test]
fn artifact_package_requires_explicit_runtime_version_support() {
    let error = match artifact_workspace_without_support("artifact-support-denial") {
        Ok(_) => panic!("artifact contract without runtime version support installed"),
        Err(error) => error,
    };
    assert!(error.message().contains("UnsupportedArtifactVersion"));
    assert!(error
        .message()
        .contains("WORTH.tests.artifact-workflow.candidates:1:1"));
}

#[test]
fn foreign_provider_is_denied_and_its_resource_is_disposed_exactly_once() {
    let (mut workspace, probe) = artifact_move_workspace("artifact-provider-denial").unwrap();
    let outcome = bind_artifact_workflow(&workspace)
        .reexecute(move_intent("reject-provider"), &mut workspace);

    assert!(matches!(
        outcome,
        TransitionOutcome::Denied(_) | TransitionOutcome::Failed(_)
    ));
    assert_eq!(
        probe.denials(),
        vec![domain::WorthQueryArtifactDenialKind::ProviderFamilyMismatch]
    );
    assert_eq!(probe.allocations(), 1);
    assert_eq!(probe.projection_calls(), 1);
    assert_eq!(probe.disposals(), 1);
}
