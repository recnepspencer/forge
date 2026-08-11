//! Owner-local proofs for commit-to-receipt outbox resolution.

use worth_foundational::facade::{BoundaryProtocolIdentity, BoundaryProtocolVersion};
use worth_query_declaration::facade::application_schema::ApplicationExternalEffectProtocol;
use worth_query_installation::facade::InstalledExternalEffectContract;
use worth_relational::facade::transactions::{RecordRef, TransactionOptions, WorkerIntentBatch};

use super::{
    WorthQueryCommittedDispatchOutboxBindingDenial, WorthQueryCommittedDispatchOutboxResolution,
};
use crate::domain_computation::application_aftermath::{
    bind_dispatch_outbox_create_intent, derive_external_effect_correlation_identity,
    ExternalEffectCorrelationBasis, WorthQueryDispatchOutboxRecord,
    WorthQueryPendingDispatchOutbox,
};
use crate::domain_computation::primary_graph::tests::fixture::installed_authorization_world;

#[test]
fn real_commit_mapping_denial_reaches_receipt_projection() {
    let (requested, committed_pending, committed) = commit_only_other_outbox();
    let committed_resolution = WorthQueryCommittedDispatchOutboxResolution::from_commit(
        Some(&committed_pending),
        &committed,
    );
    let seal = committed_resolution
        .seal_for_receipt()
        .expect("the committed create resolves");
    let binding = seal
        .binding()
        .expect("the committed create has one binding");
    assert_eq!(binding.record(), committed_pending.record());
    assert_eq!(
        binding.record_ref(),
        &RecordRef::Entity(
            committed
                .created_entity(committed_pending.created_entity())
                .expect("the exact committed create has an owner-minted mapping")
        )
    );

    let denied =
        WorthQueryCommittedDispatchOutboxResolution::from_commit(Some(&requested), &committed);
    assert_eq!(
        denied.seal_for_receipt().map(|seal| seal.into_binding()),
        Err(WorthQueryCommittedDispatchOutboxBindingDenial::CreatedEntityMissing)
    );
}

#[test]
fn real_commit_with_no_requested_outbox_projects_honest_absence() {
    let (_, _, committed) = commit_only_other_outbox();
    let absent = WorthQueryCommittedDispatchOutboxResolution::from_commit(None, &committed);
    assert_eq!(
        absent
            .seal_for_receipt()
            .expect("honest absence is a successful receipt projection")
            .into_binding(),
        None
    );
}

fn commit_only_other_outbox() -> (
    WorthQueryPendingDispatchOutbox,
    WorthQueryPendingDispatchOutbox,
    worth_relational::facade::transactions::CommitResult,
) {
    let world = installed_authorization_world(true);
    let provider = &world.application.primary_provider;
    let (_, requested) = bind_dispatch_outbox_create_intent(
        Some(provider.graph.layout.provider_dispatch_outbox()),
        Some(&record(1)),
    )
    .expect("requested outbox create binds");
    let (committed_intent, committed_pending) = bind_dispatch_outbox_create_intent(
        Some(provider.graph.layout.provider_dispatch_outbox()),
        Some(&record(2)),
    )
    .expect("committed outbox create binds");
    let committed = provider.graph.with_runtime_mut(|runtime| {
        let mut transaction = runtime.begin_transaction(TransactionOptions::default());
        transaction.push_batch(
            WorkerIntentBatch::new("outbox-resolution-owner-proof").push(committed_intent),
        );
        transaction.commit().expect("the other outbox commits")
    });
    (requested, committed_pending, committed)
}

fn record(identity: u64) -> WorthQueryDispatchOutboxRecord {
    let correlation = derive_external_effect_correlation_identity(ExternalEffectCorrelationBasis {
        correlation_family: "receipt-resolution-test",
        operation_slot: "notify",
        operation_version: 1,
        outcome_identity: identity,
        idempotency_key: &[identity as u8; 32],
        branch: "main",
    })
    .expect("test correlation derives");
    WorthQueryDispatchOutboxRecord::from_installed_contract(
        correlation,
        &InstalledExternalEffectContract::Declared {
            correlation_family: "receipt-resolution-test".to_owned(),
            effect: "ReceiptResolutionEffect".to_owned(),
            rust_payload_type: "tests::ReceiptResolutionPayload".to_owned(),
            protocol: ApplicationExternalEffectProtocol::new(
                BoundaryProtocolIdentity::new("test.receipt-resolution.payload"),
                BoundaryProtocolVersion::new(1),
            ),
            maximum_payload_bytes: 24,
        },
        vec![identity as u8; 8],
        identity,
    )
    .expect("declared test contract produces an outbox record")
}
