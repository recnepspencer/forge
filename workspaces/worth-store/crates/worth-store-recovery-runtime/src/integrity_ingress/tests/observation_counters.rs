use worth_store_physical_format::store_namespace::{
    ProposedStoreIdentity, StoreNamespaceIdentityRecord, StoreNamespaceVersion,
};
use worth_store_physical_format::PhysicalRecordFormatDeclaration;
use worth_store_physical_integrity::{
    IndeterminatePhysicalIntegrityCause, IndeterminatePhysicalIntegrityPosture,
    PhysicalArtifactScope, PhysicalBlastRadius, PhysicalByteRange, PhysicalDamageCause,
    PhysicalDamageLocalization, PhysicalIntegrityRejection, PhysicalIntegrityVersionAxis,
    UnknownPhysicalIntegrityCause, UnknownPhysicalIntegrityPosture,
    UnsupportedPhysicalIntegrityVersion,
};

use super::super::{
    RecoveryIntegrityIngressCounters, RecoveryIntegrityIngressObservation,
    RecoveryIntegrityIngressObservationOutcome, RecoveryIntegrityIngressRejection,
};

#[test]
fn counters_preserve_every_validator_and_binding_rejection_class() {
    let scope = scope();
    let damage = PhysicalIntegrityRejection::Damaged(PhysicalDamageLocalization::new(
        scope,
        PhysicalDamageCause::ChecksumMismatch,
        scope.byte_range(),
        None,
        PhysicalBlastRadius::CompleteArtifact,
    ));
    let unsupported =
        PhysicalIntegrityRejection::Unsupported(UnsupportedPhysicalIntegrityVersion::new(
            scope,
            PhysicalIntegrityVersionAxis::EnvelopeSchema,
            u32::MAX,
        ));
    let unknown = PhysicalIntegrityRejection::Unknown(UnknownPhysicalIntegrityPosture::new(
        scope,
        UnknownPhysicalIntegrityCause::UnrecognizedArtifact,
    ));
    let indeterminate =
        PhysicalIntegrityRejection::Indeterminate(IndeterminatePhysicalIntegrityPosture::new(
            scope,
            IndeterminatePhysicalIntegrityCause::StableRangeNotProven,
            Some(scope.byte_range()),
        ));
    let rejections = [
        RecoveryIntegrityIngressRejection::Integrity(damage),
        RecoveryIntegrityIngressRejection::Integrity(unsupported),
        RecoveryIntegrityIngressRejection::Integrity(unknown),
        RecoveryIntegrityIngressRejection::Integrity(indeterminate),
        RecoveryIntegrityIngressRejection::Absent,
        RecoveryIntegrityIngressRejection::ConflictingDuplication {
            observed_sources: 2,
        },
        RecoveryIntegrityIngressRejection::ScopeMismatch,
    ];
    let mut counters = RecoveryIntegrityIngressCounters::default();
    for rejection in rejections {
        let observation = RecoveryIntegrityIngressObservation::rejected(scope, rejection);
        assert_eq!(
            observation.outcome(),
            RecoveryIntegrityIngressObservationOutcome::Rejected(rejection)
        );
        assert_eq!(observation.scope(), scope);
        counters.record(observation);
    }
    assert_eq!(counters.attempted, 7);
    assert_eq!(counters.admitted, 0);
    assert_eq!(counters.rejected_damaged, 1);
    assert_eq!(counters.rejected_unsupported, 1);
    assert_eq!(counters.rejected_unknown, 1);
    assert_eq!(counters.rejected_indeterminate, 1);
    assert_eq!(counters.rejected_absent, 1);
    assert_eq!(counters.rejected_conflicting, 1);
    assert_eq!(counters.rejected_source_binding, 1);
    assert_eq!(counters.owner_projection_entries, 0);
}

fn scope() -> PhysicalArtifactScope {
    let store = StoreNamespaceIdentityRecord::new(
        StoreNamespaceVersion::CURRENT,
        ProposedStoreIdentity::from_nonzero_bytes([0x51; 16]).unwrap(),
    )
    .published_identity();
    PhysicalArtifactScope::bootstrap_catalog(
        store,
        PhysicalRecordFormatDeclaration::builder().admit().unwrap(),
        PhysicalByteRange::new(0, 82).unwrap(),
    )
}
