use worth_proof::TransitionOutcome;
use worth_signal::facade::{
    AspectVersion, ClockAdvanceOrdinal, ClockDomain, ClockTick, TemporalPreviousValueReference,
    TemporalWakeId, WakeOrdinal,
};
use serde_json::json;

use super::*;

pub(crate) fn admitted_temporal_basis(
    truth_basis: BridgeTemporalTruthViewBasis,
) -> crate::facade::AdmittedBridgeTemporalBasis {
    admitted_temporal_basis_with_wake(truth_basis, 11, 7, 5)
}

pub(crate) fn admitted_temporal_basis_with_wake(
    truth_basis: BridgeTemporalTruthViewBasis,
    wake_id: u64,
    wake_ready_ordinal: u64,
    wake_tick: u64,
) -> crate::facade::AdmittedBridgeTemporalBasis {
    let signal_basis = BridgeTemporalSignalBasis::new(
        truth_basis.branch_identity().clone(),
        ClockDomain::MonotonicExecution,
        ClockTick::new(wake_tick),
        ClockAdvanceOrdinal::new(3),
        None,
    );
    let wake = BridgeTemporalWakeEvidence::new(
        TemporalWakeId::new(wake_id),
        WakeOrdinal::new(wake_ready_ordinal),
        ClockTick::new(wake_tick),
    );
    match AdmittedBridgeTemporalBasis::admit(truth_basis, signal_basis, Some(wake)) {
        TransitionOutcome::Success(admitted) => admitted,
        outcome => panic!("expected admitted temporal basis, got {outcome:?}"),
    }
}

pub(crate) fn committed_patch(
    branch_identity: TruthBranchIdentity,
    snapshot_identity: crate::facade::TruthSnapshotIdentity,
    commit_identity: TruthCommitIdentity,
    patch_identity: TruthPatchIdentity,
) -> BridgeCommittedPatchEnvelope {
    BridgeCommittedPatchEnvelope::new(
        BridgeCommittedPatchEnvelopeIdentity::new_with_metadata(
            crate::facade::BridgeProducerMetadata::bridge_harness_fixture(),
            commit_identity,
            patch_identity,
            snapshot_identity,
            branch_identity,
        ),
        vec![BridgeCommittedPatchItem::with_target(
            "entity-1",
            crate::facade::BridgeCommittedPatchTarget::entity_field_path(
                worth_foundational::facade::AspectLocator::new(
                    worth_foundational::facade::LocatorAuthority::Authoritative,
                    worth_foundational::facade::AspectKey::new("profile")
                        .expect("valid committed patch aspect key"),
                ),
                worth_foundational::facade::CanonicalFieldPath::single(
                    worth_foundational::facade::FieldKey::new("name".to_owned())
                        .expect("valid committed patch field key"),
                ),
            ),
        )],
    )
    .expect("committed patch envelope should construct")
}

pub(crate) fn retained_previous_value_reference(
    wake_id: u64,
    node: &str,
    tick: u64,
) -> TemporalPreviousValueReference {
    serde_json::from_value(json!({
        "revision": 1,
        "branch_id": 1,
        "access_wake_id": wake_id,
        "node": node,
        "captured_at_tick": tick,
        "aspect_version": AspectVersion::zero(),
        "output_identity": null
    }))
    .expect("previous value reference fixture should deserialize")
}
