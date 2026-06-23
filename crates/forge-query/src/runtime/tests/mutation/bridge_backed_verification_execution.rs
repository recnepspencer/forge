use super::super::support::*;
use crate::memory_workspace::ForgeQueryEntityIdentity;
use forge_runtime_bridge::facade::RelationalBridgeRecordIdentityParts;

fn relational_test_entity_identity(table_id: u32, record_id: u64) -> ForgeQueryEntityIdentity {
    ForgeQueryEntityIdentity::from_relational_record(RelationalBridgeRecordIdentityParts::entity(
        table_id, record_id, 0,
    ))
}

fn support_row<'a>(
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
        .expect("bridge-backed verification support row should exist")
}

fn admitted_primary_profile(target_binding_family: &str) -> ForgeQueryRuntimeSupportProfile {
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

#[test]
fn primary_bridge_backed_entity_verification_family_executes_when_profile_and_adapter_admit_it() {
    let binding = ForgeQueryExistingEntityTarget::new(
        crate::runtime::ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(
            crate::runtime::ForgeQueryExistingTruthBindingAuthorityLabel::new("authority:task-1")
                .expect("existing-truth authority label"),
        )
        .expect("existing-truth authority identity"),
        relational_test_entity_identity(1, 1),
    )
    .expect("existing entity target should build")
    .in_target_collection("Task")
    .expect("existing entity target collection should build");
    let binding = ForgeQueryExistingTruthTargetBinding::from_entity_target(binding)
        .expect("entity binding should build");
    let runtime = bridge_runtime_with_support_and_existing_truth_verification(
        admitted_primary_profile("direct_entity_identity"),
        TestExistingTruthVerificationAdapter::default()
            .with_value(&binding, "status.value", json!("open"))
            .with_value(&binding, "title.value", json!("Seed title")),
    );
    let mut workspace = runtime
        .workspace("tasks.primary-bridge-backed-entity-verification")
        .expect("workspace should open");
    let support = workspace.public_authoritative_mutation_evidence_support();

    for operation_family in [
        "verify_existing",
        "probe_existing",
        "update_existing_verified",
        "delete_existing_verified",
    ] {
        let row = support_row(&support, operation_family, "direct_entity_identity");
        assert_eq!(
            row.current_posture_status(),
            ForgeQueryBridgeBackedVerificationSupportStatus::Admitted
        );
        assert!(row.primary_bridge_backed_runtime_supported());
        assert_eq!(row.denial_class_when_unsupported(), None);
    }

    let verify_receipt = workspace
        .verify_existing(binding.clone(), |task| task.aspect("status.value", "open"))
        .expect("entity verify_existing should execute");
    assert_eq!(
        verify_receipt.mutation_family(),
        ForgeQueryMutationFamily::Assertion
    );
    assert_eq!(
        verify_receipt
            .existing_truth_assertion_evidence()
            .expect("verify receipt should retain assertion evidence")
            .mode(),
        ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
    );
    let verify_assumptions = verify_receipt
        .verified_assumption_set()
        .expect("verify receipt should retain verified assumption set");
    assert_eq!(
        verify_assumptions.binding_digest(),
        verify_receipt
            .existing_truth_binding_evidence()
            .expect("verify receipt should retain binding evidence")
            .binding_digest()
            .as_str()
    );
    assert_eq!(
        verify_assumptions.asserted_aspect_paths(),
        &["status.value".to_string()]
    );
    assert_eq!(
        verify_assumptions
            .verification_read_set_breadth()
            .counter_snapshot(),
        "target_bindings=1;asserted_aspects=1;distinct_asserted_aspect_paths=1;cleared_assertions=0"
    );
    assert_eq!(
        verify_assumptions.assumption_snapshot_digest(),
        verify_assumptions
            .assumption_snapshot_evidence_digest()
            .as_str()
    );
    assert_eq!(
        verify_assumptions.verified_precondition_digest(),
        verify_assumptions
            .verified_precondition_evidence_digest()
            .as_str()
    );

    let probe = workspace
        .probe_existing(binding.clone(), ["status.value", "title.value"])
        .expect("entity probe_existing should execute");
    assert_eq!(
        probe
            .field("status.value")
            .expect("status field should exist")
            .external_value_json(),
        "\"open\""
    );

    let update_receipt = workspace
        .update_existing_verified(
            binding.clone(),
            |task| task.aspect("status.value", "open"),
            |task| task.aspect("status.value", "closed"),
        )
        .expect("entity update_existing_verified should execute");
    assert_eq!(
        update_receipt.mutation_family(),
        ForgeQueryMutationFamily::Update
    );
    assert_eq!(
        update_receipt
            .existing_truth_binding_evidence()
            .expect("update receipt should retain binding evidence")
            .family(),
        ForgeQueryExistingTruthBindingFamily::DirectEntityIdentity
    );
    assert_eq!(
        update_receipt
            .verification_read_set_breadth()
            .expect("verified update should retain read-set breadth")
            .counter_snapshot(),
        "target_bindings=1;asserted_aspects=1;distinct_asserted_aspect_paths=1;cleared_assertions=0"
    );

    let delete_receipt = workspace
        .delete_existing_verified(
            binding,
            |task| task.aspect("title.value", "Seed title"),
            |delete| delete.touch("title.value"),
        )
        .expect("entity delete_existing_verified should execute");
    assert_eq!(
        delete_receipt.mutation_family(),
        ForgeQueryMutationFamily::Delete
    );
    assert_eq!(
        delete_receipt
            .existing_truth_assertion_evidence()
            .expect("delete receipt should retain assertion evidence")
            .mode(),
        ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
    );
    let delete_assumptions = delete_receipt
        .verified_assumption_set()
        .expect("verified delete should retain assumption set");
    assert_eq!(
        delete_assumptions.verified_precondition_digest(),
        delete_assumptions
            .verified_precondition_evidence_digest()
            .as_str()
    );
}

#[test]
fn primary_bridge_backed_relation_verification_family_executes_when_profile_and_adapter_admit_it() {
    let binding = ForgeQueryExistingRelationTarget::new(
        crate::runtime::ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(
            crate::runtime::ForgeQueryExistingTruthBindingAuthorityLabel::new("authority:rel-1")
                .expect("existing-truth authority label"),
        )
        .expect("existing-truth authority identity"),
        relational_test_entity_identity(2, 1),
    )
    .expect("existing relation target should build")
    .in_target_collection("TaskRelation")
    .expect("existing relation target collection should build");
    let binding = ForgeQueryExistingTruthTargetBinding::from_relation_target(binding)
        .expect("relation binding should build");
    let runtime = bridge_runtime_with_support_and_existing_truth_verification(
        admitted_primary_profile("direct_relation_identity"),
        TestExistingTruthVerificationAdapter::default()
            .with_value(&binding, "kind.value", json!("depends_on"))
            .with_value(&binding, "status.value", json!("active")),
    );
    let mut workspace = runtime
        .workspace("tasks.primary-bridge-backed-relation-verification")
        .expect("workspace should open");
    let support = workspace.public_authoritative_mutation_evidence_support();

    assert_eq!(
        support_row(&support, "probe_existing", "direct_relation_identity")
            .current_posture_status(),
        ForgeQueryBridgeBackedVerificationSupportStatus::Admitted
    );

    let probe = workspace
        .probe_existing(binding.clone(), ["kind.value", "status.value"])
        .expect("relation probe_existing should execute");
    assert_eq!(
        probe
            .field("kind.value")
            .expect("kind field should exist")
            .external_value_json(),
        "\"depends_on\""
    );

    let update_receipt = workspace
        .update_existing_verified(
            binding.clone(),
            |relation| relation.aspect("status.value", "active"),
            |relation| relation.aspect("status.value", "retired"),
        )
        .expect("relation update_existing_verified should execute");
    assert_eq!(
        update_receipt
            .existing_truth_binding_evidence()
            .expect("relation update should retain binding evidence")
            .family(),
        ForgeQueryExistingTruthBindingFamily::DirectRelationIdentity
    );
    let update_assumptions = update_receipt
        .verified_assumption_set()
        .expect("relation update should retain assumption set");
    assert_eq!(
        update_assumptions.asserted_aspect_paths(),
        &["status.value".to_string()]
    );
    assert_eq!(
        update_assumptions
            .verification_read_set_breadth()
            .counter_snapshot(),
        "target_bindings=1;asserted_aspects=1;distinct_asserted_aspect_paths=1;cleared_assertions=0"
    );
    assert_eq!(
        update_assumptions.verified_precondition_digest(),
        update_assumptions
            .verified_precondition_evidence_digest()
            .as_str()
    );
    match workspace
        .inspect(&update_receipt)
        .expect("relation update should inspect")
    {
        ForgeQueryInspection::WriteReceipt(inspection) => {
            assert_eq!(
                inspection
                    .verified_assumption_set()
                    .expect("inspection should retain assumption set")
                    .verified_precondition_digest(),
                update_assumptions.verified_precondition_digest()
            );
            assert_eq!(
                inspection
                    .verification_read_set_breadth()
                    .expect("inspection should retain read-set breadth")
                    .counter_snapshot(),
                update_assumptions
                    .verification_read_set_breadth()
                    .counter_snapshot()
            );
        }
        other => panic!("expected write receipt inspection, got {other:?}"),
    }

    let delete_receipt = workspace
        .delete_existing_verified(
            binding,
            |relation| relation.aspect("kind.value", "depends_on"),
            |delete| delete.touch("kind.value"),
        )
        .expect("relation delete_existing_verified should execute");
    assert_eq!(
        delete_receipt
            .existing_truth_assertion_evidence()
            .expect("relation delete should retain assertion evidence")
            .mode(),
        ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
    );
}
