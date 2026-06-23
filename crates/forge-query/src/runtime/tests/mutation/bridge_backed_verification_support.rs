use super::super::support::*;

fn task_relation_runtime() -> ForgeQueryRuntime {
    stateful_bridge_runtime_with_support(
        &["Task", "TaskRelation"],
        admitted_profile("direct_entity_identity")
            .with_bridge_backed_verification_support(
                "verify_existing",
                "direct_relation_identity",
                true,
                true,
                None,
            )
            .with_bridge_backed_verification_support(
                "probe_existing",
                "direct_relation_identity",
                true,
                true,
                None,
            )
            .with_bridge_backed_verification_support(
                "update_existing_verified",
                "direct_relation_identity",
                true,
                true,
                None,
            )
            .with_bridge_backed_verification_support(
                "delete_existing_verified",
                "direct_relation_identity",
                true,
                true,
                None,
            ),
    )
}

fn admitted_profile(target_binding_family: &str) -> ForgeQueryRuntimeSupportProfile {
    [
        "verify_existing",
        "probe_existing",
        "update_existing_verified",
        "delete_existing_verified",
    ]
    .into_iter()
    .fold(
        ForgeQueryRuntimeSupportProfile::bridge_backed(
            "test-subscription-activation",
            "test-preview-basis",
            "test-inspector-evidence",
        ),
        |profile, operation_family| {
            profile.with_bridge_backed_verification_support(
                operation_family,
                target_binding_family,
                true,
                true,
                None,
            )
        },
    )
}

fn verification_row<'a>(
    support: &'a ForgeQueryAuthoritativeMutationEvidenceSupport,
    operation_family: &str,
    target_binding_family: &str,
) -> &'a ForgeQueryBridgeBackedVerificationSupportRow {
    support
        .bridge_backed_verification_support_rows()
        .iter()
        .find(|row| {
            row.operation_family() == operation_family
                && row.target_binding_family() == target_binding_family
        })
        .expect("support row should exist")
}

#[test]
fn bridge_backed_entity_verification_rows_match_runtime_behavior() {
    let runtime =
        stateful_bridge_runtime_with_support(&["Task"], admitted_profile("direct_entity_identity"));
    let mut workspace = runtime
        .workspace("tasks.bridge-backed-entity-verification-support")
        .expect("workspace should open");
    let support = workspace.public_authoritative_mutation_evidence_support();
    let _: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view(
            "tasks.bridge-backed-entity-verification-support-table",
            |q| {
                q.from("Task")
                    .select([
                        crate::authoring::AspectFieldKey::from_authoring_parts("identity", "id")
                            .unwrap(),
                        crate::authoring::AspectFieldKey::from_authoring_parts("status", "value")
                            .unwrap(),
                    ])
                    .order_by(
                        crate::authoring::AspectFieldKey::from_authoring_parts("identity", "id")
                            .unwrap(),
                    )
                    .schema_basis("tasks-bridge-backed-entity-verification-support-table")
            },
        )
        .expect("entity live view should declare");

    let seed = workspace
        .insert("Task", |task| {
            task.set_aspect(
                test_aspect_touch("identity.id"),
                test_authored_string_aspect_value("task-1"),
            )
            .set_aspect(
                test_aspect_touch("title.value"),
                test_authored_string_aspect_value("Task one"),
            )
            .set_aspect(
                test_aspect_touch("status.value"),
                test_authored_string_aspect_value("open"),
            )
        })
        .expect("seed insert should execute");
    let binding = workspace
        .bind_existing_entity(
            ForgeQueryExistingEntityTarget::new(
                crate::runtime::ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::ForgeQueryExistingTruthBindingAuthorityLabel::new("authority:task-1").expect("existing-truth authority label")).expect("existing-truth authority identity"),
                seed.deltas()[0].entity_identity.clone(),
            )
            .expect("existing entity target should build")
            .in_target_collection("Task")
            .expect("existing entity target collection should build"),
        )
        .expect("binding should build");

    for operation_family in [
        "verify_existing",
        "probe_existing",
        "update_existing_verified",
        "delete_existing_verified",
    ] {
        let row = verification_row(&support, operation_family, "direct_entity_identity");
        assert_eq!(
            row.current_posture_status(),
            ForgeQueryBridgeBackedVerificationSupportStatus::Admitted
        );
    }

    workspace
        .verify_existing(binding.clone(), |task| {
            task.set_aspect(
                test_aspect_touch("status.value"),
                test_authored_string_aspect_value("open"),
            )
        })
        .expect("entity verify_existing should execute");
    workspace
        .probe_existing(binding.clone(), test_aspect_touches(["status.value"]))
        .expect("entity probe_existing should execute");
    workspace
        .update_existing_verified(
            binding.clone(),
            |verify| {
                verify.set_aspect(
                    test_aspect_touch("status.value"),
                    test_authored_string_aspect_value("open"),
                )
            },
            |update| {
                update.set_aspect(
                    test_aspect_touch("status.value"),
                    test_authored_string_aspect_value("closed"),
                )
            },
        )
        .expect("entity update_existing_verified should execute");
    workspace
        .delete_existing_verified(
            binding,
            |verify| {
                verify.set_aspect(
                    test_aspect_touch("status.value"),
                    test_authored_string_aspect_value("closed"),
                )
            },
            |delete| delete.touch(test_aspect_touch("status.value")),
        )
        .expect("entity delete_existing_verified should execute");
}

#[test]
fn bridge_backed_relation_verification_rows_match_runtime_behavior() {
    let mut workspace = task_relation_runtime()
        .workspace("tasks.bridge-backed-relation-verification-support")
        .expect("workspace should open");
    let support = workspace.public_authoritative_mutation_evidence_support();
    let _: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view(
            "tasks.bridge-backed-relation-verification-support-table",
            |q| {
                q.from("TaskRelation")
                    .select([
                        crate::authoring::AspectFieldKey::from_authoring_parts("identity", "id")
                            .unwrap(),
                        crate::authoring::AspectFieldKey::from_authoring_parts("status", "value")
                            .unwrap(),
                    ])
                    .order_by(
                        crate::authoring::AspectFieldKey::from_authoring_parts("identity", "id")
                            .unwrap(),
                    )
                    .schema_basis("tasks-bridge-backed-relation-verification-support-table")
            },
        )
        .expect("relation live view should declare");

    let seed = workspace
        .insert("TaskRelation", |relation| {
            relation
                .set_aspect(
                    test_aspect_touch("identity.id"),
                    test_authored_string_aspect_value("rel-1"),
                )
                .set_aspect(
                    test_aspect_touch("kind.value"),
                    test_authored_string_aspect_value("depends_on"),
                )
                .set_aspect(
                    test_aspect_touch("status.value"),
                    test_authored_string_aspect_value("active"),
                )
        })
        .expect("seed insert should execute");
    let binding = workspace
        .bind_existing_relation(
            ForgeQueryExistingRelationTarget::new(
                crate::runtime::ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::ForgeQueryExistingTruthBindingAuthorityLabel::new("authority:task-1").expect("existing-truth authority label")).expect("existing-truth authority identity"),
                seed.deltas()[0].entity_identity.clone(),
            )
            .expect("existing relation target should build")
            .in_target_collection("TaskRelation")
            .expect("existing relation target collection should build"),
        )
        .expect("relation binding should build");

    for operation_family in [
        "verify_existing",
        "probe_existing",
        "update_existing_verified",
        "delete_existing_verified",
    ] {
        let row = verification_row(&support, operation_family, "direct_relation_identity");
        assert_eq!(
            row.current_posture_status(),
            ForgeQueryBridgeBackedVerificationSupportStatus::Admitted
        );
    }

    workspace
        .verify_existing(binding.clone(), |relation| {
            relation.set_aspect(
                test_aspect_touch("status.value"),
                test_authored_string_aspect_value("active"),
            )
        })
        .expect("relation verify_existing should execute");
    workspace
        .probe_existing(binding.clone(), test_aspect_touches(["status.value"]))
        .expect("relation probe_existing should execute");
    workspace
        .update_existing_verified(
            binding.clone(),
            |verify| {
                verify.set_aspect(
                    test_aspect_touch("status.value"),
                    test_authored_string_aspect_value("active"),
                )
            },
            |update| {
                update.set_aspect(
                    test_aspect_touch("status.value"),
                    test_authored_string_aspect_value("retired"),
                )
            },
        )
        .expect("relation update_existing_verified should execute");
    workspace
        .delete_existing_verified(
            binding,
            |verify| {
                verify.set_aspect(
                    test_aspect_touch("status.value"),
                    test_authored_string_aspect_value("retired"),
                )
            },
            |delete| delete.touch(test_aspect_touch("status.value")),
        )
        .expect("relation delete_existing_verified should execute");
}

#[test]
fn primary_entity_bridge_backed_verification_rows_match_runtime_denials() {
    let mut workspace =
        bridge_runtime_with_support(ForgeQueryRuntimeSupportProfile::bridge_backed(
            "test-subscription-activation",
            "test-preview-basis",
            "test-inspector-evidence",
        ))
        .workspace("tasks.primary-entity-verification-support")
        .expect("workspace should open");
    let support = workspace.public_authoritative_mutation_evidence_support();
    let binding = workspace
        .bind_existing_entity(
            ForgeQueryExistingEntityTarget::new(crate::runtime::ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::ForgeQueryExistingTruthBindingAuthorityLabel::new("authority:task-1").expect("existing-truth authority label")).expect("existing-truth authority identity"), test_entity_identity("Task:1"))
                .expect("existing entity target should build")
                .in_target_collection("Task")
                .expect("existing entity target collection should build"),
        )
        .expect("binding should build");

    assert_eq!(
        verification_row(&support, "verify_existing", "direct_entity_identity")
            .denial_class_when_unsupported(),
        Some("backend_verification_unsupported")
    );
    assert_eq!(
        verification_row(&support, "probe_existing", "direct_entity_identity")
            .denial_class_when_unsupported(),
        Some("backend_probe_unsupported")
    );

    assert!(matches!(
        workspace.verify_existing(binding.clone(), |task| task.set_aspect(test_aspect_touch("status.value"), test_authored_string_aspect_value("open"))),
        Err(ForgeQueryRuntimeError::ExistingTruthAssertionDenied(denial))
            if denial.kind() == ForgeQueryExistingTruthAssertionDenialKind::BackendVerificationUnsupported
    ));
    assert!(matches!(
        workspace.probe_existing(binding.clone(), test_aspect_touches(["status.value"])),
        Err(ForgeQueryRuntimeError::ExistingTruthProbeDenied(denial))
            if denial.kind() == ForgeQueryExistingTruthProbeDenialKind::BackendProbeUnsupported
    ));
    assert!(matches!(
        workspace.update_existing_verified(
            binding.clone(),
            |verify| verify.set_aspect(test_aspect_touch("status.value"), test_authored_string_aspect_value("open")),
            |update| update.set_aspect(test_aspect_touch("status.value"), test_authored_string_aspect_value("closed")),
        ),
        Err(ForgeQueryRuntimeError::ExistingTruthAssertionDenied(denial))
            if denial.kind() == ForgeQueryExistingTruthAssertionDenialKind::BackendVerificationUnsupported
    ));
    assert!(matches!(
        workspace.delete_existing_verified(
            binding,
            |verify| verify.set_aspect(test_aspect_touch("status.value"), test_authored_string_aspect_value("closed")),
            |delete| delete.touch(test_aspect_touch("status.value")),
        ),
        Err(ForgeQueryRuntimeError::ExistingTruthAssertionDenied(denial))
            if denial.kind() == ForgeQueryExistingTruthAssertionDenialKind::BackendVerificationUnsupported
    ));
}

#[test]
fn primary_relation_bridge_backed_verification_rows_match_runtime_denials() {
    let mut workspace =
        bridge_runtime_with_support(ForgeQueryRuntimeSupportProfile::bridge_backed(
            "test-subscription-activation",
            "test-preview-basis",
            "test-inspector-evidence",
        ))
        .workspace("tasks.primary-relation-verification-support")
        .expect("workspace should open");
    let support = workspace.public_authoritative_mutation_evidence_support();
    let binding = workspace
        .bind_existing_relation(
            ForgeQueryExistingRelationTarget::new(
                crate::runtime::ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::ForgeQueryExistingTruthBindingAuthorityLabel::new("authority:task-1").expect("existing-truth authority label")).expect("existing-truth authority identity"),
                test_entity_identity("TaskRelation:1"),
            )
            .expect("existing relation target should build")
            .in_target_collection("TaskRelation")
            .expect("existing relation target collection should build"),
        )
        .expect("relation binding should build");

    assert_eq!(
        verification_row(&support, "verify_existing", "direct_relation_identity")
            .current_posture_status(),
        ForgeQueryBridgeBackedVerificationSupportStatus::Denied
    );
    assert_eq!(
        verification_row(&support, "probe_existing", "direct_relation_identity")
            .denial_class_when_unsupported(),
        Some("backend_probe_unsupported")
    );

    assert!(matches!(
        workspace.verify_existing(binding.clone(), |relation| relation.set_aspect(test_aspect_touch("kind.value"), test_authored_string_aspect_value("depends_on"))),
        Err(ForgeQueryRuntimeError::ExistingTruthAssertionDenied(denial))
            if denial.kind() == ForgeQueryExistingTruthAssertionDenialKind::BackendVerificationUnsupported
    ));
    assert!(matches!(
        workspace.probe_existing(binding.clone(), test_aspect_touches(["kind.value"])),
        Err(ForgeQueryRuntimeError::ExistingTruthProbeDenied(denial))
            if denial.kind() == ForgeQueryExistingTruthProbeDenialKind::BackendProbeUnsupported
    ));
    assert!(matches!(
        workspace.update_existing_verified(
            binding.clone(),
            |verify| verify.set_aspect(test_aspect_touch("kind.value"), test_authored_string_aspect_value("depends_on")),
            |update| update.set_aspect(test_aspect_touch("kind.value"), test_authored_string_aspect_value("blocked_by")),
        ),
        Err(ForgeQueryRuntimeError::ExistingTruthAssertionDenied(denial))
            if denial.kind() == ForgeQueryExistingTruthAssertionDenialKind::BackendVerificationUnsupported
    ));
    assert!(matches!(
        workspace.delete_existing_verified(
            binding,
            |verify| verify.set_aspect(test_aspect_touch("kind.value"), test_authored_string_aspect_value("depends_on")),
            |delete| delete.touch(test_aspect_touch("kind.value")),
        ),
        Err(ForgeQueryRuntimeError::ExistingTruthAssertionDenied(denial))
            if denial.kind() == ForgeQueryExistingTruthAssertionDenialKind::BackendVerificationUnsupported
    ));
}
