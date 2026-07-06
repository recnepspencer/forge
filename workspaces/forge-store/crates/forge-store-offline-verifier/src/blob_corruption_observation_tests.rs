use forge_foundational::{aspects, AspectValue, InternedString, ScalarAspectType};
use forge_proof::TransitionOutcome;
use forge_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StorePhysicalBoundaryWitness,
};
use forge_store_authority::require_current_store_authority;
use forge_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};
use forge_store_security::{
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreKeyVersionPosture, StoreRawSecurityScopeDeclaration, StoreTenantScope,
};

use crate::{
    OfflineBlobCorruptionEvidenceKind, OfflineBlobCorruptionObservation,
    OfflineBlobCorruptionObservationDenial, OfflineBlobDamageCaseHint,
};

#[test]
fn offline_blob_corruption_observation_accepts_only_raw_unadmitted_reports() {
    let authority = current_authority("offline.blob-corruption");
    let physical_witness = authority.physical_witness();
    let raw = StoreRawSecurityScopeDeclaration::deserialized_unadmitted(
        physical_witness,
        StoreKeyScope::BlobChunkEnvelope,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::ImportReadmissionBoundary,
        Some(StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedBlobChunk,
        )),
        Some(StoreCustodyPosture::ImportedUnreadmitted),
    );

    let observed = OfflineBlobCorruptionObservation::from_offline_corruption_report(
        raw,
        OfflineBlobCorruptionEvidenceKind::ColdFetch,
    )
    .expect("offline corruption report should observe as raw evidence");
    assert_eq!(observed.raw_declaration(), raw);
    assert_eq!(
        observed.evidence_kind(),
        OfflineBlobCorruptionEvidenceKind::ColdFetch
    );

    let native = StoreRawSecurityScopeDeclaration::native(
        physical_witness,
        StoreKeyScope::BlobChunkEnvelope,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::ImportReadmissionBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedBlobChunk,
        ),
        StoreCustodyPosture::Readmitted,
    );
    assert_eq!(
        OfflineBlobCorruptionObservation::from_offline_corruption_report(
            native,
            OfflineBlobCorruptionEvidenceKind::QuarantineReport,
        ),
        Err(OfflineBlobCorruptionObservationDenial::NotRawReportInput)
    );

    let classified = OfflineBlobCorruptionObservation::admit_and_classify_offline_corruption_report(
        raw,
        OfflineBlobCorruptionEvidenceKind::Import,
    )
    .expect("import report should classify");
    assert_eq!(
        classified.damage_case_hint(),
        OfflineBlobDamageCaseHint::CrossScopeImport
    );
}

fn current_authority(identity_key: &str) -> forge_store_authority::StoreCurrentAuthorityWitness {
    require_current_store_authority(boundary_fact(identity_key, "current"))
}

fn boundary_fact(identity_key: &str, value: &str) -> StoreAspectBoundaryFact {
    let key = aspects()
        .vocabulary()
        .key(identity_key)
        .expect("aspect key");
    let contract = aspects()
        .contract()
        .for_key(key.clone())
        .identified_by(aspects().vocabulary().identity(1))
        .at_revision(aspects().vocabulary().revision(1))
        .scalar(ScalarAspectType::String);
    let value = aspects()
        .validate()
        .against(&contract)
        .value(AspectValue::String(InternedString::from(value)));
    let value = match value {
        TransitionOutcome::Success(value) => value,
        outcome => panic!("validation should succeed: {outcome:?}"),
    };
    let state = match aspects().authoritative_state().admit([value]) {
        TransitionOutcome::Success(state) => state,
        outcome => panic!("state admission should succeed: {outcome:?}"),
    };
    let physical = StorePhysicalBoundaryWitness::from_physical_authority(
        StorePhysicalAuthorityWitness::for_aspect_native_boundary(
            ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
        )
        .unwrap(),
    )
    .unwrap();
    StoreAspectBoundaryFact::from_admitted_state(
        StoreAspectIdentity::from_aspect_key(key),
        StoreAspectAuthorityInput::new(state, physical),
    )
    .unwrap()
}
