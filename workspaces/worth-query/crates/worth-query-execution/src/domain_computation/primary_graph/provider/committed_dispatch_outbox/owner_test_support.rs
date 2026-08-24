use worth_foundational::facade::{
    AspectValue, BoundaryProtocolIdentity, BoundaryProtocolVersion, InternedString,
};
use worth_query_declaration::facade::application_schema::ApplicationExternalEffectProtocol;
use worth_query_installation::facade::InstalledExternalEffectContract;
use worth_relational::facade::mvcc::BranchBoundRelationalTransaction;
use worth_relational::facade::transactions::WorkerIntentBatch;

use super::super::WorthQueryPrimaryGraphProvider;
use crate::domain_computation::application_aftermath::{
    derive_external_effect_correlation_identity, dispatch_outbox_create_intent,
    ExternalEffectCorrelationBasis, WorthQueryDispatchOutboxRecord,
};
use crate::domain_computation::primary_graph::{
    primary_relational_branch_id, WorthQueryCommittedDispatchOutboxBinding,
};

pub(super) fn commit_record(
    provider: &WorthQueryPrimaryGraphProvider,
    identity: u64,
) -> (
    WorthQueryCommittedDispatchOutboxBinding,
    worth_relational::facade::history::RelationalCommitReceipt,
    u64,
) {
    let record = record_for(identity);
    let branch = primary_relational_branch_id();
    let (binding, commit, runtime_id) = provider.graph.with_runtime_mut(|runtime| {
        let mut transaction: BranchBoundRelationalTransaction ={
    let transaction_validation_input = runtime
                .admit_main_branch_basis()
                .expect("main branch binding");
    runtime
        .begin_branch_transaction(
            &transaction_validation_input,
            worth_relational::facade::mvcc::RelationalTransactionIntent::ordinary(),
        )
        .expect("owner-admitted transaction context")
};
        transaction.push_batch(
            WorkerIntentBatch::new("committed-outbox-owner-test").push(
                dispatch_outbox_create_intent(
                    Some(provider.graph.layout.provider_dispatch_outbox()),
                    Some(&record),
                )
                .unwrap(),
            ),
        );
        let committed = transaction.commit(runtime).unwrap();
        let binding = WorthQueryCommittedDispatchOutboxBinding::fixture_from_commit(
            provider.graph.layout.provider_dispatch_outbox(),
            Some(&record),
            &committed,
        )
        .unwrap()
        .unwrap();
        let commit = committed.outcome().commit.clone();
        let snapshot = crate::domain_computation::primary_graph::exact_basis_access::open_current_branch_snapshot(runtime, &branch)
            .unwrap();
        let runtime_id = snapshot.runtime_instance_id();
        runtime.snapshots().release_snapshot(&snapshot);
        (binding, commit, runtime_id)
    });
    (binding, commit, runtime_id)
}

pub(super) fn record_for(identity: u64) -> WorthQueryDispatchOutboxRecord {
    let correlation = derive_external_effect_correlation_identity(ExternalEffectCorrelationBasis {
        correlation_family:
            worth_query_installation::facade::WorthQueryExternalEffectCorrelationFamily::new(
                "owner-test",
            )
            .unwrap(),
        operation_slot: "notify",
        operation_version: 1,
        outcome_identity: identity,
        idempotency_key: &[identity as u8; 32],
        branch: "main",
    })
    .unwrap();
    WorthQueryDispatchOutboxRecord::from_installed_contract(
        correlation,
        &InstalledExternalEffectContract::Declared {
            correlation_family:
                worth_query_installation::facade::WorthQueryExternalEffectCorrelationFamily::new(
                    "owner-test",
                )
                .unwrap(),
            effect: "OwnerTestEffect".to_owned(),
            rust_payload_type: "tests::Payload".to_owned(),
            protocol: ApplicationExternalEffectProtocol::new(
                BoundaryProtocolIdentity::new("test.owner.payload"),
                BoundaryProtocolVersion::new(1),
            ),
            maximum_payload_bytes: 24,
        },
        vec![identity as u8; 8],
        identity,
    )
    .unwrap()
}

pub(super) fn string(value: String) -> AspectValue {
    AspectValue::String(InternedString::from(value))
}
