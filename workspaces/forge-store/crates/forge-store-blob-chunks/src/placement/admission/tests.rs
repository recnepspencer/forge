use forge_store_budgets::CounterEvidenceStrength;
use forge_store_io_scheduler::IoSchedulerIsolationAdmission;
use forge_store_physical_backend::{
    BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis, BackendCapabilityKind,
    BackendCapabilitySupportPosture, BackendCapabilitySupportSet, BackendMediaAssumptionSet,
    BackendRebindTriggers, BackendTargetProfile, PhysicalBackendCapabilityAdmissionAuthority,
};
use forge_store_security::{
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreKeyVersionPosture, StoreSecurityScopeIdentity, StoreTenantScope,
};
use forge_store_tiering::{admit_tier_placement_io, ColdPlacementState, TierPlacementIoAdmission};

use crate::lifecycle::generation_registry_test_support::{
    lifecycle_receipt_for_publication, root_publication,
};
use crate::{
    BlobAuthorityClassification, BlobPlacementAdmissionAuthority, BlobPlacementAdmissionDenial,
    BlobPlacementClass, BlobPlacementIntent,
};

use super::test_support::{
    admitted_backend, external_recovery, external_recovery_for_digest,
    external_recovery_for_digest_and_scope, readiness, residue_observation,
};

#[test]
fn placement_classes_preserve_blob_facts_with_distinct_counters() {
    let receipt = receipt("phase16-parity");
    let reachability = receipt.reachability();
    let authority = BlobPlacementAdmissionAuthority::from_admitted_backend(admitted_backend());

    let inline = authority
        .admit(
            reachability,
            BlobPlacementIntent::inline(readiness(reachability)),
        )
        .expect("inline placement should admit");
    let external = authority
        .admit(
            reachability,
            BlobPlacementIntent::external(readiness(reachability), external_recovery(reachability)),
        )
        .expect("external placement should admit");
    let cold = authority
        .admit(
            reachability,
            BlobPlacementIntent::cold(readiness(reachability), ColdPlacementState::ColdAvailable),
        )
        .expect("cold placement should admit");

    for placement in [&inline, &external, &cold] {
        assert_eq!(placement.stored_digest(), reachability.stored_digest());
        assert_eq!(
            placement.security_metadata(),
            reachability.security_metadata()
        );
        assert_eq!(placement.non_claims().len(), 3);
    }
    assert_eq!(inline.class(), BlobPlacementClass::Inline);
    assert_eq!(external.class(), BlobPlacementClass::External);
    assert_eq!(cold.class(), BlobPlacementClass::Cold);
    assert_eq!(inline.counters().inline_reads(), 1);
    assert_eq!(inline.counters().strength(), CounterEvidenceStrength::Exact);
    assert_eq!(
        inline.counters().placement_class(),
        Some(BlobPlacementClass::Inline)
    );
    assert_eq!(external.counters().external_reads(), 1);
    assert_eq!(
        external.counters().strength(),
        CounterEvidenceStrength::Exact
    );
    assert_eq!(
        external.counters().placement_class(),
        Some(BlobPlacementClass::External)
    );
    assert_eq!(cold.counters().cold_fetches(), 1);
    assert_eq!(cold.counters().strength(), CounterEvidenceStrength::Exact);
    assert_eq!(
        cold.counters().placement_class(),
        Some(BlobPlacementClass::Cold)
    );
}

#[test]
fn copied_readiness_seed_security_scope_denies_before_placement_admission() {
    let receipt = receipt("phase16-copied-readiness-seed");
    let reachability = receipt.reachability();
    let authority = BlobPlacementAdmissionAuthority::from_admitted_backend(admitted_backend());

    match authority.admit(
        reachability,
        BlobPlacementIntent::inline(readiness_for_security_scope(mismatched_security_scope())),
    ) {
        Err(BlobPlacementAdmissionDenial::PlacementReadinessBasisMismatch { counters }) => {
            assert_eq!(counters.inline_reads(), 0);
            assert_eq!(counters.external_reads(), 0);
            assert_eq!(counters.cold_fetches(), 0);
        }
        outcome => panic!("expected copied readiness basis denial, got {outcome:?}"),
    }
}

#[test]
fn unsupported_backend_capability_denies_before_publication() {
    let receipt = receipt("phase16-unsupported-backend");
    let reachability = receipt.reachability();
    let authority =
        BlobPlacementAdmissionAuthority::from_admitted_backend(backend_without_direct_io());

    match authority.admit(
        reachability,
        BlobPlacementIntent::external(readiness(reachability), external_recovery(reachability)),
    ) {
        Err(BlobPlacementAdmissionDenial::BackendCapability { counters, .. }) => {
            assert_eq!(counters.external_reads(), 0);
        }
        outcome => panic!("expected backend capability denial, got {outcome:?}"),
    }
}

#[test]
fn unrelated_external_recovery_denies_before_placement_admission() {
    let source = receipt("phase16-external-source");
    let reachability = source.reachability();
    let authority = BlobPlacementAdmissionAuthority::from_admitted_backend(admitted_backend());

    match authority.admit(
        reachability,
        BlobPlacementIntent::external(
            readiness(reachability),
            external_recovery_for_digest("s7:stored:unrelated-external-manifest"),
        ),
    ) {
        Err(BlobPlacementAdmissionDenial::ExternalPlacementRecoverabilityBasisMismatch {
            counters,
        }) => assert_eq!(counters.external_reads(), 1),
        outcome => panic!("expected external recoverability mismatch denial, got {outcome:?}"),
    }
}

#[test]
fn same_digest_external_recovery_with_wrong_security_scope_denies() {
    let source = receipt("phase16-external-same-digest-wrong-scope");
    let reachability = source.reachability();
    let digest = reachability.stored_digest().digest().as_str();
    let authority = BlobPlacementAdmissionAuthority::from_admitted_backend(admitted_backend());

    match authority.admit(
        reachability,
        BlobPlacementIntent::external(
            readiness(reachability),
            external_recovery_for_digest_and_scope(digest, mismatched_security_scope()),
        ),
    ) {
        Err(BlobPlacementAdmissionDenial::ExternalPlacementRecoverabilityBasisMismatch {
            counters,
        }) => assert_eq!(counters.external_reads(), 1),
        outcome => {
            panic!("expected external recoverability scope mismatch denial, got {outcome:?}")
        }
    }
}

#[test]
fn unavailable_cold_chunks_deny_with_cold_state() {
    let receipt = receipt("phase16-cold-unavailable");
    let reachability = receipt.reachability();
    let authority = BlobPlacementAdmissionAuthority::from_admitted_backend(admitted_backend());

    assert_eq!(
        authority.admit(
            reachability,
            BlobPlacementIntent::cold(readiness(reachability), ColdPlacementState::ColdUnavailable)
        ),
        Err(BlobPlacementAdmissionDenial::ColdChunkUnavailable {
            state: ColdPlacementState::ColdUnavailable,
            counters: crate::BlobPlacementCounterSnapshot::for_class(BlobPlacementClass::Cold)
                .record_unavailable_cold_chunk()
                .record_tier_move_protected_denial()
        })
    );
}

#[test]
fn external_sidecar_without_store_authority_denies() {
    let receipt = receipt("phase16-sidecar-denial");
    let reachability = receipt.reachability();
    let authority = BlobPlacementAdmissionAuthority::from_admitted_backend(admitted_backend());
    let sidecar = residue_observation(
        forge_store_physical_backend::BlobBackendResidueObservationKind::OrphanedPlacementResidue,
        "sidecar-path",
    );

    match authority.admit(
        reachability,
        BlobPlacementIntent::external_sidecar_without_store_authority(
            readiness(reachability),
            sidecar,
        ),
    ) {
        Err(BlobPlacementAdmissionDenial::ExternalSidecarWithoutStoreAuthority {
            counters,
            ..
        }) => assert_eq!(counters.external_reads(), 1),
        outcome => panic!("expected sidecar authority denial, got {outcome:?}"),
    }
}

fn receipt(case: &str) -> crate::LifecycleReceipt {
    let (publication, stored_digest) = root_publication(case);
    lifecycle_receipt_for_publication(
        case,
        publication.chunk_tree_root().clone(),
        publication.logical_content_digest().clone(),
        stored_digest,
        BlobAuthorityClassification::StoreOwnedPhysicalBlob,
    )
}

fn readiness_for_security_scope(scope: StoreSecurityScopeIdentity) -> TierPlacementIoAdmission {
    let cold = forge_store_tiering::certification_test_support::cold_tier_io_posture_for_certification_test(scope);
    admit_tier_placement_io(
        IoSchedulerIsolationAdmission::for_certification_test(),
        cold,
    )
}

fn mismatched_security_scope() -> StoreSecurityScopeIdentity {
    StoreSecurityScopeIdentity::from_physical_security_scope(
        physical_witness(),
        StoreKeyScope::BlobChunkEnvelope,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::MultiTenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedBlobChunk,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    )
}

fn physical_witness() -> forge_store_aspect_native::StorePhysicalBoundaryWitness {
    use forge_store_contracts::{
        StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
    };

    forge_store_aspect_native::StorePhysicalBoundaryWitness::from_physical_authority(
        StorePhysicalAuthorityWitness::for_aspect_native_boundary(
            ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
        )
        .expect("physical authority"),
    )
    .expect("physical boundary")
}

fn backend_without_direct_io() -> forge_store_physical_backend::AdmittedBackendCapabilityWitness {
    PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(BackendCapabilityAdmissionRequest::new(
            BackendTargetProfile::PosixFileFsyncDirSync,
            BackendCapabilityEvidenceBasis::certified_backend_profile(),
            BackendCapabilitySupportSet::all_supported().with_posture(
                BackendCapabilityKind::DirectIo,
                BackendCapabilitySupportPosture::Unsupported,
            ),
            BackendMediaAssumptionSet::platform_file_defaults()
                .with_direct_io_alignment()
                .with_sector_atomicity()
                .with_page_cache_policy()
                .with_async_ordering()
                .with_flush_ordering()
                .with_fdatasync_durability()
                .with_cold_tier_io_posture(),
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
        ))
        .expect("backend should admit")
}
