use super::*;

#[test]
fn existing_truth_cluster_unsupported_backend_denials_remain_typed_and_distinct() {
    let runtime = bridge_runtime_with_support(WorthQueryRuntimeSupportProfile::bridge_backed(
        "test-subscription-activation",
        "test-preview-basis",
        "test-inspector-evidence",
    ));
    let mut workspace = runtime
        .workspace("tasks.existing-truth-cluster-unsupported")
        .expect("workspace should open");
    let binding = workspace
        .bind_existing_entity(
            WorthQueryExistingEntityTarget::new(crate::runtime::WorthQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::WorthQueryExistingTruthBindingAuthorityLabel::new("authority:task-assert").expect("existing-truth authority label")).expect("existing-truth authority identity"), test_entity_identity("Task:1"))
                .expect("existing entity target should build")
                .in_target_collection("Task")
                .expect("existing entity target collection should build"),
        )
        .expect("binding should build");

    let probe_error = workspace
        .probe_existing(binding.clone(), test_aspect_touches(["title.value"]))
        .expect_err("unsupported backend probe should deny");
    match probe_error {
        WorthQueryRuntimeError::ExistingTruthProbeDenied(denial) => {
            assert_eq!(
                denial.kind(),
                WorthQueryExistingTruthProbeDenialKind::BackendProbeUnsupported
            );
        }
        other => panic!("expected typed probe denial, got {other:?}"),
    }

    let verify_error = workspace
        .verify_existing(binding.clone(), |task| {
            task.set_aspect(
                test_aspect_touch("title.value"),
                test_authored_string_aspect_value("Seed title"),
            )
        })
        .expect_err("unsupported backend verification should deny");
    match verify_error {
        WorthQueryRuntimeError::ExistingTruthAssertionDenied(denial) => {
            assert_eq!(
                denial.kind(),
                WorthQueryExistingTruthAssertionDenialKind::BackendVerificationUnsupported
            );
        }
        other => panic!("expected typed assertion denial, got {other:?}"),
    }

    let update_error = workspace
        .update_existing_verified(
            binding.clone(),
            |task| {
                task.set_aspect(
                    test_aspect_touch("title.value"),
                    test_authored_string_aspect_value("Seed title"),
                )
            },
            |task| {
                task.set_aspect(
                    test_aspect_touch("title.value"),
                    test_authored_string_aspect_value("Updated title"),
                )
            },
        )
        .expect_err("unsupported backend verified update should deny");
    match update_error {
        WorthQueryRuntimeError::ExistingTruthAssertionDenied(denial) => {
            assert_eq!(
                denial.kind(),
                WorthQueryExistingTruthAssertionDenialKind::BackendVerificationUnsupported
            );
        }
        other => panic!("expected typed assertion denial, got {other:?}"),
    }

    let delete_error = workspace
        .delete_existing_verified(
            binding,
            |task| {
                task.set_aspect(
                    test_aspect_touch("title.value"),
                    test_authored_string_aspect_value("Seed title"),
                )
            },
            |delete| delete.touch(test_aspect_touch("title.value")),
        )
        .expect_err("unsupported backend verified delete should deny");
    match delete_error {
        WorthQueryRuntimeError::ExistingTruthAssertionDenied(denial) => {
            assert_eq!(
                denial.kind(),
                WorthQueryExistingTruthAssertionDenialKind::BackendVerificationUnsupported
            );
        }
        other => panic!("expected typed assertion denial, got {other:?}"),
    }
}
