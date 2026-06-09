use forge_proof::TransitionOutcome;
use forge_query::facade::{
    ForgeQueryAspectMutationBuilder, ForgeQueryExistingEntityTarget,
    ForgeQueryExistingTruthTargetBinding, ForgeQueryRuntimeFacadeFamily,
    ForgeQueryRuntimeFamilySupport, ForgeQueryRuntimeSupportProfile, ForgeQueryWriteCommand,
};
use forge_server::{
    ForgeServerDirectMutationOutcome, ForgeServerQueryHandoffDenialCode,
    ForgeServerQueryHandoffInput, ForgeServerQueryHandoffOperation, ForgeServerQueryOperation,
    ForgeServerResponseInput, ForgeServerSuccessKind,
};
use std::sync::atomic::Ordering;

use crate::forge_native_assertions::{
    family_contract_digest, forge_native_session, operator_evidence_record,
    response_provenance_digest,
};
use crate::forge_native_runtime::{build_server, build_server_with_profiled_counting_workspace};
use crate::query_handoff_fixture::{request_input, resolve_request_context, success};

#[test]
fn direct_single_mutation_returns_provenance_bearing_result_boundary() {
    let server = build_server(true);
    let mutation = direct_mutation_success(forge_native_session(&server).direct().mutate(
        &ForgeServerQueryOperation::single_mutation("tasks.insert", insert_task("task-1")),
    ));

    assert_eq!(
        mutation
            .response_envelope()
            .success()
            .expect("success envelope")
            .payload()
            .kind(),
        ForgeServerSuccessKind::DirectMutation
    );
    let result = mutation.mutation_result();
    let receipt = result
        .single_receipt()
        .expect("single mutation should expose a write receipt");
    let inspection = result
        .single_inspection()
        .expect("single mutation should expose a write inspection");
    assert_eq!(result.result_digest(), receipt.commit_identity());
    assert_eq!(result.inspection_digest(), inspection.inspection_digest());
    assert_eq!(
        result.execution_provenance_digest(),
        receipt
            .execution_provenance_chain_digest()
            .expect("single receipt should preserve execution provenance")
    );
    assert_eq!(inspection.commit_identity(), receipt.commit_identity());
    assert_eq!(inspection.snapshot_token(), receipt.snapshot_token());
    assert!(mutation
        .canonical_digest()
        .contains(mutation.handoff_digest()));
}

#[test]
fn direct_batch_mutation_preserves_batch_receipt_and_inspection_digests() {
    let server = build_server(true);
    let mutation = direct_mutation_success(forge_native_session(&server).direct().mutate(
        &ForgeServerQueryOperation::batch_mutation(
            "tasks.seed",
            vec![insert_task("task-a"), insert_task("task-b")],
        ),
    ));

    let result = mutation.mutation_result();
    let receipt = result
        .batch_receipt()
        .expect("batch mutation should expose a batch receipt");
    let inspection = result
        .batch_inspection()
        .expect("batch mutation should expose a batch inspection");
    assert_eq!(receipt.write_count(), 2);
    assert_eq!(inspection.write_receipt_count(), 2);
    assert_eq!(result.result_digest(), receipt.batch_digest());
    assert_eq!(result.inspection_digest(), inspection.inspection_digest());
    assert_eq!(
        result.execution_provenance_digest(),
        receipt
            .execution_provenance()
            .expect("batch receipt should preserve execution provenance")
            .execution_provenance_chain_digest()
    );
}

#[test]
fn direct_mutation_denies_backend_verified_assertions_at_the_server_boundary() {
    let server = build_server(true);
    let denial = direct_mutation_denied(forge_native_session(&server).direct().mutate(
        &ForgeServerQueryOperation::single_mutation(
            "tasks.verify-existing",
            verify_existing_task("authority:task-1", "task-1"),
        ),
    ));

    assert_eq!(
        denial.code(),
        ForgeServerQueryHandoffDenialCode::DirectMutationAssertionDenied
    );
    assert!(denial.detail().contains("does not admit backend-verified"));
}

#[test]
fn direct_mutation_denies_before_write_when_write_family_is_unavailable() {
    let (server, attempted_writes) = build_server_with_profiled_counting_workspace(
        ForgeQueryRuntimeSupportProfile::scaffold_backend_profile().with_family_support(
            ForgeQueryRuntimeFamilySupport::unsupported(
                ForgeQueryRuntimeFacadeFamily::Write,
                "write is intentionally denied in this hostile mutation profile",
            ),
        ),
    );
    let denial = direct_mutation_denied(forge_native_session(&server).direct().mutate(
        &ForgeServerQueryOperation::single_mutation("tasks.insert", insert_task("task-1")),
    ));

    assert_eq!(
        denial.code(),
        ForgeServerQueryHandoffDenialCode::UnsupportedQueryFacadeFamily
    );
    assert!(denial
        .detail()
        .contains("query workspace does not admit `write` facade family"));
    assert_eq!(
        attempted_writes.load(Ordering::Relaxed),
        0,
        "unsupported write should deny direct mutation before any write attempt"
    );
    let denial_response =
        server
            .responses()
            .shape_with_defaults(ForgeServerResponseInput::query_handoff_denied(
                denial.clone(),
            ));
    let denial_evidence = operator_evidence_record(&server, denial_response);
    assert_eq!(
        denial_evidence
            .counter_receipt()
            .counter("response.query_handoff_denial.count")
            .expect("query handoff denial counter")
            .exact_value(),
        1
    );
    assert_eq!(
        denial_evidence
            .counter_receipt()
            .counter("response.query_mutation_success.count")
            .expect("mutation success counter")
            .exact_value(),
        0
    );
    assert_eq!(
        denial_evidence
            .counter_receipt()
            .counter("response.query_read_success.count")
            .expect("read success counter")
            .exact_value(),
        0
    );
}

#[test]
fn direct_mutation_denies_before_write_when_receipt_inspection_family_is_unavailable() {
    let (server, attempted_writes) = build_server_with_profiled_counting_workspace(
        ForgeQueryRuntimeSupportProfile::scaffold_backend_profile().with_family_support(
            ForgeQueryRuntimeFamilySupport::unsupported(
                ForgeQueryRuntimeFacadeFamily::Inspect,
                "inspect is intentionally denied in this hostile mutation profile",
            ),
        ),
    );
    let denial = direct_mutation_denied(forge_native_session(&server).direct().mutate(
        &ForgeServerQueryOperation::single_mutation("tasks.insert", insert_task("task-1")),
    ));

    assert_eq!(
        denial.code(),
        ForgeServerQueryHandoffDenialCode::UnsupportedQueryFacadeFamily
    );
    assert!(denial
        .detail()
        .contains("query workspace does not admit `inspect` facade family"));
    assert_eq!(
        attempted_writes.load(Ordering::Relaxed),
        0,
        "unsupported inspect should deny direct mutation before any write attempt"
    );
}

#[test]
fn direct_mutation_preserves_mutation_lane_support_and_operator_classification() {
    let server = build_server(true);
    let direct = direct_mutation_success(forge_native_session(&server).direct().mutate(
        &ForgeServerQueryOperation::single_mutation("tasks.insert", insert_task("task-1")),
    ));
    let admission = match server
        .middleware()
        .admit(forge_server::ForgeServerPipelineInput::new(
            resolve_request_context(
                &server,
                request_input(
                    forge_server::ForgeServerSurfaceFamily::CompatHttp,
                    forge_server::ForgeServerTransportClass::CompatHttp,
                ),
            ),
            forge_server::ForgeServerPipelineIntent::query_mutation("tasks.insert"),
        )) {
        TransitionOutcome::Success(admission) => admission,
        other => panic!("expected admitted mutation pipeline result, got {other:?}"),
    };
    let compat_handoff = success(server.query_handoff().prepare(
        ForgeServerQueryHandoffInput::new(
            admission,
            ForgeServerQueryHandoffOperation::query_mutation("tasks.insert"),
        ),
    ));
    let compat_response =
        server
            .responses()
            .shape_with_defaults(ForgeServerResponseInput::query_handoff_success(
                compat_handoff,
            ));
    let direct_evidence = operator_evidence_record(&server, direct.response_envelope().clone());

    assert_eq!(
        family_contract_digest(direct.support_posture()),
        family_contract_digest(
            compat_response
                .success()
                .expect("compat response should succeed")
                .payload()
                .support_posture()
        )
    );
    assert_eq!(
        response_provenance_digest(direct.response_envelope()),
        response_provenance_digest(&compat_response)
    );
    assert_eq!(
        direct_evidence.classification(),
        &forge_server::ForgeServerOperatorEvidenceClass::QueryMutationSucceeded
    );
    assert_eq!(
        direct_evidence
            .counter_receipt()
            .counter("response.query_mutation_success.count")
            .expect("mutation success counter")
            .exact_value(),
        1
    );
    assert_eq!(
        direct_evidence
            .counter_receipt()
            .counter("response.query_read_success.count")
            .expect("read success counter")
            .exact_value(),
        0
    );
}

fn insert_task(identity: &str) -> ForgeQueryWriteCommand {
    ForgeQueryAspectMutationBuilder::new()
        .aspect("identity.id", identity)
        .aspect("title.value", format!("Title for {identity}"))
        .build_insert("Task")
        .expect("insert command should build")
}

fn verify_existing_task(
    authoritative_identity: &str,
    resolved_entity_identity: &str,
) -> ForgeQueryWriteCommand {
    ForgeQueryAspectMutationBuilder::new()
        .aspect("title.value", "Expected title")
        .build_verify_existing(existing_binding(
            authoritative_identity,
            resolved_entity_identity,
        ))
        .expect("verify existing command should build")
}

fn existing_binding(
    authoritative_identity: &str,
    resolved_entity_identity: &str,
) -> ForgeQueryExistingTruthTargetBinding {
    ForgeQueryExistingTruthTargetBinding::from_entity_target(
        ForgeQueryExistingEntityTarget::new(authoritative_identity, resolved_entity_identity)
            .expect("existing entity target should build")
            .in_target_collection("Task")
            .expect("existing entity target collection should build"),
    )
    .expect("existing entity binding should build")
}

fn direct_mutation_success(
    outcome: ForgeServerDirectMutationOutcome,
) -> forge_server::ForgeServerDirectMutation {
    match outcome {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected successful direct mutation, got {other:?}"),
    }
}

fn direct_mutation_denied(
    outcome: ForgeServerDirectMutationOutcome,
) -> forge_server::ForgeServerQueryHandoffDenial {
    match outcome {
        TransitionOutcome::Denied(denial) => denial,
        other => panic!("expected denied direct mutation, got {other:?}"),
    }
}
