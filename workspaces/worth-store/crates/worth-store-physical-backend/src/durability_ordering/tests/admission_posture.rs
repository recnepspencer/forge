use crate::{
    BackendCapabilityEvidenceBasis, BackendCapabilityKind, BackendCapabilitySupportPosture,
    BackendCapabilitySupportSet, BackendMediaAssumptionSet, CapabilityEvidenceClass,
    StoreDurabilityAdmission, StoreDurabilityAdmissionOutcome, StoreDurabilityDenialKind,
    StoreDurabilityRequirement, StoreDurabilityState, WalDurabilityBarrier,
    WalDurabilityBarrierSet,
};

use super::super::test_support::witness;

#[test]
fn external_guarantee_cannot_satisfy_certified_durability_api() {
    let witness = witness(
        BackendCapabilityEvidenceBasis::externally_guaranteed(1),
        BackendCapabilitySupportSet::buffered_durable_only(),
        BackendMediaAssumptionSet::platform_file_defaults(),
    );

    let outcome = StoreDurabilityAdmission::admit_checked(
        StoreDurabilityRequirement::wal_ordering_barrier(WalDurabilityBarrierSet::of(
            WalDurabilityBarrier::WalFileFsync,
        )),
        &witness,
    );

    let StoreDurabilityAdmissionOutcome::Denied(denial) = outcome else {
        panic!("external guarantee must not admit certified durability");
    };
    assert_eq!(
        denial.kind(),
        StoreDurabilityDenialKind::ExternallyGuaranteedCannotSatisfyCertifiedApi
    );
    assert_eq!(
        denial.actual_evidence(),
        CapabilityEvidenceClass::ExternallyGuaranteed
    );
}

#[test]
fn unsupported_unknown_stale_and_rebind_postures_remain_visible() {
    let cases = [
        (
            BackendCapabilitySupportPosture::Unsupported,
            StoreDurabilityState::DurabilityUnsupported,
            StoreDurabilityDenialKind::UnsupportedDurabilityCapability,
        ),
        (
            BackendCapabilitySupportPosture::Unknown,
            StoreDurabilityState::DurabilityUnknown,
            StoreDurabilityDenialKind::UnknownDurabilityPosture,
        ),
        (
            BackendCapabilitySupportPosture::Stale,
            StoreDurabilityState::Stale,
            StoreDurabilityDenialKind::StaleDurabilityPosture,
        ),
        (
            BackendCapabilitySupportPosture::RebindRequired,
            StoreDurabilityState::RebindRequired,
            StoreDurabilityDenialKind::RebindRequired,
        ),
    ];

    for (posture, state, kind) in cases {
        let support = BackendCapabilitySupportSet::buffered_durable_only()
            .with_posture(BackendCapabilityKind::DirectorySync, posture);
        let witness = witness(
            BackendCapabilityEvidenceBasis::certified_backend_profile(),
            support,
            BackendMediaAssumptionSet::platform_file_defaults(),
        );
        let denial = StoreDurabilityAdmission::admit(
            StoreDurabilityRequirement::checkpoint_publication(WalDurabilityBarrierSet::of(
                WalDurabilityBarrier::WalFileFsync,
            )),
            &witness,
        )
        .unwrap_err();

        assert_eq!(denial.state(), state);
        assert_eq!(denial.kind(), kind);
        assert_eq!(
            denial.capability(),
            Some(BackendCapabilityKind::DirectorySync)
        );
    }
}
