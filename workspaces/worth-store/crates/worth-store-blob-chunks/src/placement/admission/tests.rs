use worth_store_budgets::CounterEvidenceStrength;
use worth_store_physical_backend::{
    BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis, BackendCapabilityKind,
    BackendCapabilitySupportPosture, BackendCapabilitySupportSet, BackendMediaAssumptionSet,
    BackendRebindTriggers, BackendTargetProfile, PhysicalBackendCapabilityAdmissionAuthority,
};
use worth_store_security::{
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreKeyVersionPosture, StoreSecurityScopeIdentity, StoreTenantScope,
};
use worth_store_tiering::{ColdPlacementState, ColdTierIoPosture};

use crate::lifecycle::generation_registry_test_support::{
    lifecycle_receipt_for_publication, root_publication,
};
use crate::{
    BlobAuthorityClassification, BlobPlacementAdmissionAuthority, BlobPlacementAdmissionDenial,
    BlobPlacementClass, BlobPlacementIntent,
};

use super::test_support::{
    admitted_backend, cold_posture, external_recovery, external_recovery_for_digest,
    external_recovery_for_digest_and_scope, residue_observation,
};

#[test]
fn inline_placement_requires_only_reachability_and_backend_capability() {
    let receipt = receipt("phase16-inline-placement");
    let reachability = receipt.reachability();
    let authority = BlobPlacementAdmissionAuthority::from_admitted_backend(admitted_backend());

    let constructor: fn() -> BlobPlacementIntent<'static> = BlobPlacementIntent::<'static>::inline;
    let intent = constructor();
    assert!(matches!(intent, BlobPlacementIntent::Inline));
    assert!(
        core::mem::size_of::<BlobPlacementIntent<'static>>() <= 4 * core::mem::size_of::<usize>(),
        "placement intent must borrow class evidence instead of carrying large proof values inline"
    );
    let inline = authority
        .admit(reachability, intent)
        .expect("inline placement should admit");

    assert_eq!(inline.stored_digest(), reachability.stored_digest());
    assert_eq!(inline.security_metadata(), reachability.security_metadata());
    assert_eq!(inline.non_claims().len(), 3);
    assert_eq!(inline.class(), BlobPlacementClass::Inline);
    assert_eq!(inline.counters().inline_reads(), 1);
    assert_eq!(inline.counters().external_reads(), 0);
    assert_eq!(inline.counters().cold_fetches(), 0);
    assert_eq!(inline.counters().strength(), CounterEvidenceStrength::Exact);
    assert_eq!(
        inline.counters().placement_class(),
        Some(BlobPlacementClass::Inline)
    );
}

#[test]
fn external_placement_requires_only_reachability_recoverability_and_backend_capability() {
    let receipt = receipt("phase16-external-placement");
    let reachability = receipt.reachability();
    let authority = BlobPlacementAdmissionAuthority::from_admitted_backend(admitted_backend());

    let recoverability = external_recovery(reachability);
    let intent = external_intent(&recoverability);
    match &intent {
        BlobPlacementIntent::External {
            recoverability: carried,
        } => assert!(core::ptr::eq(*carried, &recoverability)),
        _ => panic!("external constructor must borrow only external recoverability"),
    }
    let external = authority
        .admit(reachability, intent)
        .expect("external placement should admit");

    assert_eq!(external.stored_digest(), reachability.stored_digest());
    assert_eq!(
        external.security_metadata(),
        reachability.security_metadata()
    );
    assert_eq!(external.non_claims().len(), 3);
    assert_eq!(external.class(), BlobPlacementClass::External);
    assert_eq!(external.counters().inline_reads(), 0);
    assert_eq!(external.counters().external_reads(), 1);
    assert_eq!(external.counters().cold_fetches(), 0);
    assert_eq!(
        external.counters().strength(),
        CounterEvidenceStrength::Exact
    );
    assert_eq!(
        external.counters().placement_class(),
        Some(BlobPlacementClass::External)
    );
}

#[test]
fn cold_placement_requires_exact_cold_posture_state_and_backend_capability() {
    let receipt = receipt("phase16-cold-placement");
    let reachability = receipt.reachability();
    let authority = BlobPlacementAdmissionAuthority::from_admitted_backend(admitted_backend());

    let posture = cold_posture(reachability);
    let intent = cold_intent(&posture, ColdPlacementState::ColdAvailable);
    match &intent {
        BlobPlacementIntent::Cold {
            posture: carried,
            state,
        } => {
            assert!(core::ptr::eq(*carried, &posture));
            assert_eq!(*state, ColdPlacementState::ColdAvailable);
        }
        _ => panic!("cold constructor must mint only the cold intent variant"),
    }
    let cold = authority
        .admit(reachability, intent)
        .expect("cold placement should admit");

    assert_eq!(cold.stored_digest(), reachability.stored_digest());
    assert_eq!(cold.security_metadata(), reachability.security_metadata());
    assert_eq!(cold.non_claims().len(), 3);
    assert_eq!(cold.class(), BlobPlacementClass::Cold);
    assert_eq!(cold.counters().inline_reads(), 0);
    assert_eq!(cold.counters().external_reads(), 0);
    assert_eq!(cold.counters().cold_fetches(), 1);
    assert_eq!(cold.counters().strength(), CounterEvidenceStrength::Exact);
    assert_eq!(
        cold.counters().placement_class(),
        Some(BlobPlacementClass::Cold)
    );
}

#[test]
fn wrong_cold_posture_scope_denies_before_cold_placement_admission() {
    let receipt = receipt("phase16-wrong-cold-posture-scope");
    let reachability = receipt.reachability();
    let authority = BlobPlacementAdmissionAuthority::from_admitted_backend(admitted_backend());
    let posture = cold_posture_for_security_scope(mismatched_security_scope());

    match authority.admit(
        reachability,
        BlobPlacementIntent::cold(&posture, ColdPlacementState::ColdAvailable),
    ) {
        Err(BlobPlacementAdmissionDenial::ColdPostureScopeMismatch { counters }) => {
            assert_eq!(counters.inline_reads(), 0);
            assert_eq!(counters.external_reads(), 0);
            assert_eq!(counters.cold_fetches(), 0);
        }
        outcome => panic!("expected cold posture scope denial, got {outcome:?}"),
    }
}

#[test]
fn unsupported_backend_capability_denies_before_publication() {
    let receipt = receipt("phase16-unsupported-backend");
    let reachability = receipt.reachability();
    let authority =
        BlobPlacementAdmissionAuthority::from_admitted_backend(backend_without_direct_io());
    let recoverability = external_recovery(reachability);

    match authority.admit(reachability, BlobPlacementIntent::external(&recoverability)) {
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
    let recoverability = external_recovery_for_digest("s7:stored:unrelated-external-manifest");

    match authority.admit(reachability, BlobPlacementIntent::external(&recoverability)) {
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
    let recoverability =
        external_recovery_for_digest_and_scope(digest, mismatched_security_scope());

    match authority.admit(reachability, BlobPlacementIntent::external(&recoverability)) {
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
    let posture = cold_posture(reachability);

    assert_eq!(
        authority.admit(
            reachability,
            BlobPlacementIntent::cold(&posture, ColdPlacementState::ColdUnavailable)
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
        worth_store_physical_backend::BlobBackendResidueObservationKind::OrphanedPlacementResidue,
        "sidecar-path",
    );

    match authority.admit(
        reachability,
        BlobPlacementIntent::external_sidecar_without_store_authority(&sidecar),
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

fn cold_posture_for_security_scope(scope: StoreSecurityScopeIdentity) -> ColdTierIoPosture {
    worth_store_tiering::certification_test_support::cold_tier_io_posture_for_certification_test(
        scope,
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

fn physical_witness() -> worth_store_aspect_native::StorePhysicalBoundaryWitness {
    use worth_store_contracts::{
        StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
    };

    worth_store_aspect_native::StorePhysicalBoundaryWitness::from_physical_authority(
        StorePhysicalAuthorityWitness::for_aspect_native_boundary(
            ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
        )
        .expect("physical authority"),
    )
    .expect("physical boundary")
}

fn external_intent<'evidence>(
    recoverability: &'evidence worth_store_physical_backend::StoreExternalPlacementRecoverabilityEvidence,
) -> BlobPlacementIntent<'evidence> {
    BlobPlacementIntent::external(recoverability)
}

fn cold_intent<'evidence>(
    posture: &'evidence ColdTierIoPosture,
    state: ColdPlacementState,
) -> BlobPlacementIntent<'evidence> {
    BlobPlacementIntent::cold(posture, state)
}

fn backend_without_direct_io() -> worth_store_physical_backend::AdmittedBackendCapabilityWitness {
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
