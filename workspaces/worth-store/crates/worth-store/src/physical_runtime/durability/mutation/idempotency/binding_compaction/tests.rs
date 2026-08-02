use super::decoding::PhysicalBindingCompactionRecordDecodeDenial;
use super::encoding::COMPACTION_RECORD_DOMAIN;
use super::*;
use crate::physical_runtime::durability::mutation::idempotency::persisted_binding::{
    PhysicalBindingDecodingContext, PhysicalPersistedBindingDecodeDenial,
};
use crate::physical_runtime::durability::mutation::idempotency::registry::{
    PhysicalMutationIdempotencyRegistryAdmission, PhysicalMutationUnresolvedBindingObservation,
};
use crate::physical_runtime::durability::mutation::idempotency::test_support::{
    fingerprint, fixture, mutation,
};
use crate::physical_runtime::{
    PhysicalMutationIdempotencyMaterial, PhysicalMutationProvenNoEffectCause,
};

#[test]
fn compaction_record_decoder_rejects_unknown_state_trailing_bytes_and_foreign_policy() {
    let mut fixture = fixture(4);
    let key = fixture
        .registry
        .issue_key(PhysicalMutationIdempotencyMaterial::new([91; 32]))
        .unwrap();
    let fingerprint = fingerprint(&fixture, 1);
    let mutation = mutation(&fixture, 1);
    assert!(matches!(
        fixture
            .registry
            .admit_unallocated(key, fingerprint, mutation),
        Ok(PhysicalMutationIdempotencyRegistryAdmission::Fresh(_))
    ));
    let encoded = encode_retained_record(
        fixture.registry.bindings.values().next().unwrap(),
        PhysicalNamespaceDurableCheckpointGeneration::from_namespace_durable_checkpoint(1),
    )
    .unwrap();
    let context =
        PhysicalBindingDecodingContext::new(fixture.store, fixture.policy, fixture.idempotency);
    assert!(DecodedPhysicalMutationBindingRecord::decode(&encoded, context).is_ok());

    let mut unknown_state = encoded.clone();
    unknown_state[8 + COMPACTION_RECORD_DOMAIN.len()] = u8::MAX;
    assert!(matches!(
        DecodedPhysicalMutationBindingRecord::decode(&unknown_state, context),
        Err(PhysicalBindingCompactionRecordDecodeDenial::UnknownState)
    ));

    let mut trailing = encoded.clone();
    trailing.push(0);
    assert!(matches!(
        DecodedPhysicalMutationBindingRecord::decode(&trailing, context),
        Err(PhysicalBindingCompactionRecordDecodeDenial::Persisted(
            PhysicalPersistedBindingDecodeDenial::TrailingBytes
        ))
    ));

    let foreign = PhysicalBindingDecodingContext::new(
        fixture.store,
        fixture.foreign_policy,
        fixture.idempotency,
    );
    assert!(matches!(
        DecodedPhysicalMutationBindingRecord::decode(&encoded, foreign),
        Err(PhysicalBindingCompactionRecordDecodeDenial::Persisted(
            PhysicalPersistedBindingDecodeDenial::ForeignPolicy
        ))
    ));
    fixture.media.close();
}

#[test]
fn persisted_terminal_fate_round_trips_through_its_closed_decoder_seam() {
    let mut fixture = fixture(4);
    let key = fixture
        .registry
        .issue_key(PhysicalMutationIdempotencyMaterial::new([92; 32]))
        .unwrap();
    let request_fingerprint = fingerprint(&fixture, 2);
    let request_mutation = mutation(&fixture, 2);
    assert!(fixture
        .registry
        .admit_unallocated(key.clone(), request_fingerprint, request_mutation)
        .is_ok());
    fixture
        .registry
        .cancel_before_group_seal(
            PhysicalMutationUnresolvedBindingObservation::new(
                key.identity(),
                request_fingerprint,
                request_mutation,
            ),
            PhysicalMutationProvenNoEffectCause::CancelledBeforeGroupSeal,
        )
        .unwrap();
    let encoded = encode_retained_record(
        fixture.registry.bindings.values().next().unwrap(),
        PhysicalNamespaceDurableCheckpointGeneration::from_namespace_durable_checkpoint(1),
    )
    .unwrap();
    let decoded = DecodedPhysicalMutationBindingRecord::decode(
        &encoded,
        PhysicalBindingDecodingContext::new(fixture.store, fixture.policy, fixture.idempotency),
    )
    .unwrap();
    assert!(matches!(
        decoded,
        DecodedPhysicalMutationBindingRecord::Terminal { basis, fate }
            if basis.key().identity() == key.identity()
                && fate.as_proven_no_effect().unwrap().request_fingerprint() == request_fingerprint
    ));
    fixture.media.close();
}
