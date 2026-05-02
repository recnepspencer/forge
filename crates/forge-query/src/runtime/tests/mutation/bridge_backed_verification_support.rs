use super::super::support::*;

fn task_relation_runtime() -> ForgeQueryRuntime {
    ForgeQueryRuntime::builder()
        .compatibility_in_memory_collections([
            ForgeQueryCollection::new(
                "Task",
                [
                    crate::memory_workspace::ForgeQueryAspect::new("identity.id", "identity.id"),
                    crate::memory_workspace::ForgeQueryAspect::new("title.value", "title.value"),
                    crate::memory_workspace::ForgeQueryAspect::new("status.value", "status.value"),
                ],
            ),
            ForgeQueryCollection::new(
                "TaskRelation",
                [
                    crate::memory_workspace::ForgeQueryAspect::new("identity.id", "identity.id"),
                    crate::memory_workspace::ForgeQueryAspect::new("kind.value", "kind.value"),
                    crate::memory_workspace::ForgeQueryAspect::new("status.value", "status.value"),
                ],
            ),
        ])
        .build()
        .expect("runtime should build")
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
fn compatibility_entity_bridge_backed_verification_rows_match_runtime_behavior() {
    let mut workspace = task_relation_runtime()
        .workspace("tasks.compatibility-entity-verification-support")
        .expect("workspace should open");
    let support = workspace.public_authoritative_mutation_evidence_support();
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view(
            "tasks.compatibility-entity-verification-support-table",
            |q| {
                q.from("Task")
                    .select(["identity.id", "status.value"])
                    .order_by("identity.id")
                    .schema_basis("tasks-compatibility-entity-verification-support-table")
            },
        )
        .expect("entity live view should declare");

    let seed = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-1")
                .aspect("title.value", "Task one")
                .aspect("status.value", "open")
        })
        .expect("seed insert should execute");
    let binding = workspace
        .bind_existing_entity(
            ForgeQueryExistingEntityTarget::new(
                "authority:task-1",
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
        .verify_existing(binding.clone(), |task| task.aspect("status.value", "open"))
        .expect("entity verify_existing should execute");
    workspace
        .probe_existing(binding.clone(), ["status.value"])
        .expect("entity probe_existing should execute");
    workspace
        .update_existing_verified(
            binding.clone(),
            |verify| verify.aspect("status.value", "open"),
            |update| update.aspect("status.value", "closed"),
        )
        .expect("entity update_existing_verified should execute");
    workspace
        .delete_existing_verified(
            binding,
            |verify| verify.aspect("status.value", "closed"),
            |delete| delete.touch("status.value"),
        )
        .expect("entity delete_existing_verified should execute");
}

#[test]
fn compatibility_relation_bridge_backed_verification_rows_match_runtime_behavior() {
    let mut workspace = task_relation_runtime()
        .workspace("tasks.compatibility-relation-verification-support")
        .expect("workspace should open");
    let support = workspace.public_authoritative_mutation_evidence_support();
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view(
            "tasks.compatibility-relation-verification-support-table",
            |q| {
                q.from("TaskRelation")
                    .select(["identity.id", "status.value"])
                    .order_by("identity.id")
                    .schema_basis("tasks-compatibility-relation-verification-support-table")
            },
        )
        .expect("relation live view should declare");

    let seed = workspace
        .insert("TaskRelation", |relation| {
            relation
                .aspect("identity.id", "rel-1")
                .aspect("kind.value", "depends_on")
                .aspect("status.value", "active")
        })
        .expect("seed insert should execute");
    let binding = workspace
        .bind_existing_relation(
            ForgeQueryExistingRelationTarget::new(
                "authority:rel-1",
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
            relation.aspect("status.value", "active")
        })
        .expect("relation verify_existing should execute");
    workspace
        .probe_existing(binding.clone(), ["status.value"])
        .expect("relation probe_existing should execute");
    workspace
        .update_existing_verified(
            binding.clone(),
            |verify| verify.aspect("status.value", "active"),
            |update| update.aspect("status.value", "retired"),
        )
        .expect("relation update_existing_verified should execute");
    workspace
        .delete_existing_verified(
            binding,
            |verify| verify.aspect("status.value", "retired"),
            |delete| delete.touch("status.value"),
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
            ForgeQueryExistingEntityTarget::new("authority:task-1", "Task:1")
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
        workspace.verify_existing(binding.clone(), |task| task.aspect("status.value", "open")),
        Err(ForgeQueryRuntimeError::ExistingTruthAssertionDenied(denial))
            if denial.kind() == ForgeQueryExistingTruthAssertionDenialKind::BackendVerificationUnsupported
    ));
    assert!(matches!(
        workspace.probe_existing(binding.clone(), ["status.value"]),
        Err(ForgeQueryRuntimeError::ExistingTruthProbeDenied(denial))
            if denial.kind() == ForgeQueryExistingTruthProbeDenialKind::BackendProbeUnsupported
    ));
    assert!(matches!(
        workspace.update_existing_verified(
            binding.clone(),
            |verify| verify.aspect("status.value", "open"),
            |update| update.aspect("status.value", "closed"),
        ),
        Err(ForgeQueryRuntimeError::ExistingTruthAssertionDenied(denial))
            if denial.kind() == ForgeQueryExistingTruthAssertionDenialKind::BackendVerificationUnsupported
    ));
    assert!(matches!(
        workspace.delete_existing_verified(
            binding,
            |verify| verify.aspect("status.value", "closed"),
            |delete| delete.touch("status.value"),
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
            ForgeQueryExistingRelationTarget::new("authority:rel-1", "TaskRelation:1")
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
        workspace.verify_existing(binding.clone(), |relation| relation.aspect("kind.value", "depends_on")),
        Err(ForgeQueryRuntimeError::ExistingTruthAssertionDenied(denial))
            if denial.kind() == ForgeQueryExistingTruthAssertionDenialKind::BackendVerificationUnsupported
    ));
    assert!(matches!(
        workspace.probe_existing(binding.clone(), ["kind.value"]),
        Err(ForgeQueryRuntimeError::ExistingTruthProbeDenied(denial))
            if denial.kind() == ForgeQueryExistingTruthProbeDenialKind::BackendProbeUnsupported
    ));
    assert!(matches!(
        workspace.update_existing_verified(
            binding.clone(),
            |verify| verify.aspect("kind.value", "depends_on"),
            |update| update.aspect("kind.value", "blocked_by"),
        ),
        Err(ForgeQueryRuntimeError::ExistingTruthAssertionDenied(denial))
            if denial.kind() == ForgeQueryExistingTruthAssertionDenialKind::BackendVerificationUnsupported
    ));
    assert!(matches!(
        workspace.delete_existing_verified(
            binding,
            |verify| verify.aspect("kind.value", "depends_on"),
            |delete| delete.touch("kind.value"),
        ),
        Err(ForgeQueryRuntimeError::ExistingTruthAssertionDenied(denial))
            if denial.kind() == ForgeQueryExistingTruthAssertionDenialKind::BackendVerificationUnsupported
    ));
}
