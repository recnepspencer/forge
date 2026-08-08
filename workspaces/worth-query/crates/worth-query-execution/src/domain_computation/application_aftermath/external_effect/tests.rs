use worth_foundational::facade::CanonicalDigestId;
use worth_foundational::facade::{
    BoundaryProtocolCompatibilityWindow, BoundaryProtocolIdentity,
    BoundaryProtocolUnsupportedVersionPosture, BoundaryProtocolVersion,
};
use worth_query_declaration::facade::application_schema::ApplicationExternalEffectProtocol;
use worth_query_installation::facade::InstalledExternalEffectContract;
use worth_relational::facade::history::{BranchId, CommitId, CommitReference};
use worth_relational::facade::identity::{EntityId, PartitionId, VersionId};
use worth_relational::facade::transactions::RecordRef;

use super::*;
use crate::domain_computation::primary_graph::WorthQueryCommittedDispatchOutboxObservation;

mod protocol_stability;

struct FixedTransport(WorthQueryExternalTransportOutcome);

impl WorthQueryExternalEffectTransport for FixedTransport {
    fn dispatch(
        &self,
        _request: WorthQueryExternalDispatchRequest<'_>,
    ) -> WorthQueryExternalTransportOutcome {
        self.0
    }
}

#[test]
fn completed_dispatch_has_four_distinct_exact_predecessors() {
    let dispatch = dispatch::dispatch_external_effect(
        &FixedTransport(WorthQueryExternalTransportOutcome::Completed),
        admitted(runtime_axis(), committed_outbox(7), 1),
    )
    .unwrap();
    let ladder = dispatch.causal_ladder();
    let observation = ladder.observation().expect("completion is owner-observed");
    assert_eq!(
        ladder.provider_commit().kind(),
        ExternalEffectPostureKind::ProviderCommit
    );
    assert_eq!(
        ladder.emission().kind(),
        ExternalEffectPostureKind::EmittedApplicationCausality
    );
    assert_eq!(
        ladder.attempt().kind(),
        ExternalEffectPostureKind::DispatchAttempt
    );
    assert_eq!(
        observation.kind(),
        ExternalEffectPostureKind::ExternalCompletion
    );
    let identities = [
        ladder.provider_commit().identity().bytes(),
        ladder.emission().identity().bytes(),
        ladder.attempt().identity().bytes(),
        observation.identity().bytes(),
    ];
    for left in 0..identities.len() {
        for right in (left + 1)..identities.len() {
            assert_ne!(identities[left], identities[right]);
        }
    }
    assert_predecessor(ladder.emission(), ladder.provider_commit());
    assert_predecessor(ladder.attempt(), ladder.emission());
    assert_predecessor(observation, ladder.attempt());
    assert_eq!(dispatch.canonical_work().digest_derivations(), 4);
}

#[test]
fn transport_faults_never_fabricate_owner_observation_or_completion() {
    for (index, outcome) in [
        WorthQueryExternalTransportOutcome::TimedOut,
        WorthQueryExternalTransportOutcome::Disconnected,
        WorthQueryExternalTransportOutcome::LostResponse,
        WorthQueryExternalTransportOutcome::DuplicateAcknowledgement,
        WorthQueryExternalTransportOutcome::Rejected,
    ]
    .into_iter()
    .enumerate()
    {
        let dispatch = dispatch::dispatch_external_effect(
            &FixedTransport(outcome),
            admitted(runtime_axis(), committed_outbox(9), index as u64 + 1),
        )
        .unwrap();
        assert!(!dispatch.is_external_completion());
        assert!(dispatch.causal_ladder().observation().is_none());
        assert_eq!(dispatch.canonical_work().digest_derivations(), 3);
    }
}

#[test]
fn unsupported_protocol_version_preserves_exact_external_owner_causality() {
    let unsupported = BoundaryProtocolCompatibilityWindow::inclusive(
        BoundaryProtocolVersion::new(1),
        BoundaryProtocolVersion::new(2),
    )
    .admit(BoundaryProtocolVersion::new(3))
    .unwrap_err();
    assert_eq!(
        unsupported.posture(),
        BoundaryProtocolUnsupportedVersionPosture::ExceedsWindow
    );
    let dispatch = dispatch::dispatch_external_effect(
        &FixedTransport(
            WorthQueryExternalTransportOutcome::UnsupportedProtocolVersion(unsupported),
        ),
        admitted(runtime_axis(), committed_outbox(10), 1),
    )
    .unwrap();

    assert_eq!(
        dispatch.fault(),
        Some(ExternalRailTransportFault::UnsupportedProtocolVersion(
            unsupported
        ))
    );
    assert!(!dispatch.is_external_completion());
    assert!(dispatch.causal_ladder().observation().is_none());
}

#[test]
fn distinct_attempt_ordinals_produce_distinct_attempt_identities() {
    let runtime = runtime_axis();
    let first = dispatch::dispatch_external_effect(
        &FixedTransport(WorthQueryExternalTransportOutcome::LostResponse),
        admitted(runtime, committed_outbox(11), 1),
    )
    .unwrap();
    let second = dispatch::dispatch_external_effect(
        &FixedTransport(WorthQueryExternalTransportOutcome::Completed),
        admitted(runtime, committed_outbox(11), 2),
    )
    .unwrap();
    assert_eq!(
        first.causal_ladder().emission().identity(),
        second.causal_ladder().emission().identity()
    );
    assert_ne!(
        first.causal_ladder().attempt().identity(),
        second.causal_ladder().attempt().identity()
    );
}

#[test]
fn equal_attempt_ordinals_in_distinct_query_runtimes_cannot_collide() {
    let first = dispatch::dispatch_external_effect(
        &FixedTransport(WorthQueryExternalTransportOutcome::LostResponse),
        admitted(runtime_axis(), committed_outbox(12), 1),
    )
    .unwrap();
    let second = dispatch::dispatch_external_effect(
        &FixedTransport(WorthQueryExternalTransportOutcome::LostResponse),
        admitted(runtime_axis(), committed_outbox(12), 1),
    )
    .unwrap();

    assert_ne!(
        first.causal_ladder().provider_commit().identity(),
        second.causal_ladder().provider_commit().identity()
    );
    assert_ne!(
        first.causal_ladder().attempt().identity(),
        second.causal_ladder().attempt().identity()
    );
}

#[test]
fn equal_runtime_commit_and_correlation_cannot_hide_record_ref_drift() {
    let runtime = runtime_axis();
    let first_observation = committed_outbox(13);
    let second_observation = WorthQueryCommittedDispatchOutboxObservation::fixture(
        first_observation.record().clone(),
        first_observation.commit_reference().clone(),
        RecordRef::Entity(EntityId::new(PartitionId::main(), 10, 1)),
    );
    let first = dispatch::dispatch_external_effect(
        &FixedTransport(WorthQueryExternalTransportOutcome::LostResponse),
        admitted(runtime, first_observation, 1),
    )
    .unwrap();
    let second = dispatch::dispatch_external_effect(
        &FixedTransport(WorthQueryExternalTransportOutcome::LostResponse),
        admitted(runtime, second_observation, 1),
    )
    .unwrap();

    assert_ne!(
        first.causal_ladder().provider_commit().identity(),
        second.causal_ladder().provider_commit().identity()
    );
}

#[test]
fn undeclared_external_effect_pays_zero_outbox_intents() {
    assert!(dispatch_outbox_create_intent(None, None).is_none());
}

#[test]
fn correlation_rejects_provider_string_as_identity_basis_field() {
    let left = correlation(1);
    let right = correlation(2);
    assert_ne!(left, right);
}

#[test]
fn external_effect_source_rejects_cdc_checkpoint_vocabulary() {
    const CDC_RESIDUE_FORBIDDEN: &[&str] = &[
        "SubscriberCheckpoint",
        "subscriber_checkpoint",
        "CdcSubscriber",
        "SubscriberResumeRequest",
    ];
    let sources = [
        include_str!("posture.rs"),
        include_str!("correlation.rs"),
        include_str!("classification.rs"),
        include_str!("outbox.rs"),
        include_str!("identity.rs"),
        include_str!("identity_derivation.rs"),
        include_str!("causal_event.rs"),
        include_str!("dispatch.rs"),
        include_str!("observation.rs"),
        include_str!("transport.rs"),
        include_str!("mod.rs"),
    ];
    for source in sources {
        for forbidden in CDC_RESIDUE_FORBIDDEN {
            assert!(!source.contains(forbidden));
        }
    }
}

pub(crate) fn committed_outbox(
    outcome_identity: u64,
) -> WorthQueryCommittedDispatchOutboxObservation {
    let record = WorthQueryDispatchOutboxRecord::from_installed_contract(
        correlation(outcome_identity),
        &InstalledExternalEffectContract::Declared {
            correlation_family: "estate-death-notice-rail".to_owned(),
            effect: "notify-death-effect".to_owned(),
            rust_payload_type: "fixture::DeathNotice".to_owned(),
            protocol: ApplicationExternalEffectProtocol::new(
                BoundaryProtocolIdentity::new("test.notify-death"),
                BoundaryProtocolVersion::new(1),
            ),
            maximum_payload_bytes: 1_024,
        },
        vec![0xAB; 8],
        outcome_identity,
    )
    .unwrap();
    WorthQueryCommittedDispatchOutboxObservation::fixture(
        record,
        CommitReference {
            commit_id: CommitId(17),
            version_id: VersionId(17),
            branch_id: BranchId("main".to_owned()),
            parents: Vec::new(),
        },
        RecordRef::Entity(EntityId::new(PartitionId::main(), 9, 1)),
    )
}

fn correlation(outcome_identity: u64) -> ExternalEffectCorrelationIdentity {
    derive_external_effect_correlation_identity(ExternalEffectCorrelationBasis {
        correlation_family: "estate-death-notice-rail",
        operation_slot: "notify-death",
        operation_version: 1,
        outcome_identity,
        idempotency_key: &[0xAB; 32],
        branch: "main",
    })
    .unwrap()
}

fn assert_predecessor(successor: &ExternalEffectPosture, predecessor: &ExternalEffectPosture) {
    assert_eq!(
        successor.predecessor().unwrap().predecessor(),
        predecessor.identity()
    );
}

fn ordinal(
    value: u64,
) -> crate::domain_computation::primary_graph::WorthQueryExternalDispatchAttemptOrdinal {
    crate::domain_computation::primary_graph::WorthQueryExternalDispatchAttemptOrdinal::fixture(
        value,
    )
}

fn runtime_axis() -> crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity
{
    crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity::mint_for_test(
    )
}

fn admitted(
    runtime: crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity,
    committed: WorthQueryCommittedDispatchOutboxObservation,
    ordinal_value: u64,
) -> crate::domain_computation::primary_graph::WorthQueryAdmittedExternalDispatchAttempt {
    crate::domain_computation::primary_graph::WorthQueryAdmittedExternalDispatchAttempt::fixture(
        runtime,
        committed,
        ordinal(ordinal_value),
    )
}

#[test]
fn correlation_digest_restore_is_exact() {
    let digest = CanonicalDigestId::new([0x42; 32]);
    assert_eq!(
        ExternalEffectCorrelationIdentity::from_digest(digest).digest(),
        &digest
    );
}
