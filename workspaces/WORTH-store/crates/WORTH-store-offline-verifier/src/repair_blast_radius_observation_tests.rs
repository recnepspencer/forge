use worth_foundational::{aspects, AspectValue, InternedString, ScalarAspectType};
use worth_proof::TransitionOutcome;
use worth_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StorePhysicalBoundaryWitness,
};
use worth_store_authority::require_current_store_authority;
use worth_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};
use worth_store_security::{
    repair_blast_radius_authenticity, StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture,
    StoreRawSecurityScopeDeclaration, StoreRepairPhysicalRegionDeclaration, StoreTenantScope,
};

use crate::{
    OfflineRepairBlastRadiusObservation, OfflineRepairBlastRadiusObservationDenial,
    OfflineRepairEvidenceKind,
};

#[test]
fn offline_repair_observation_is_raw_evidence_not_readiness() {
    let authority = require_current_store_authority(boundary_fact("offline.repair", "current"));
    let raw = StoreRawSecurityScopeDeclaration::deserialized_unadmitted(
        authority.physical_witness(),
        StoreKeyScope::RepairScopeEnvelope,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::RepairBlastRadius,
        Some(repair_blast_radius_authenticity()),
        Some(StoreCustodyPosture::InternalStoreCustody),
    );
    let observation = OfflineRepairBlastRadiusObservation::from_offline_repair_report(
        raw,
        StoreRepairPhysicalRegionDeclaration::raw("repair-region-a"),
        OfflineRepairEvidenceKind::RepairReadCloseout,
    )
    .expect("offline verifier should observe raw repair report evidence");

    assert_eq!(observation.raw_declaration(), raw);
    assert_eq!(observation.physical_region().region_id(), "repair-region-a");
    assert_eq!(
        observation.evidence_kind(),
        OfflineRepairEvidenceKind::RepairReadCloseout
    );

    let native = StoreRawSecurityScopeDeclaration::native(
        authority.physical_witness(),
        StoreKeyScope::RepairScopeEnvelope,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::RepairBlastRadius,
        repair_blast_radius_authenticity(),
        StoreCustodyPosture::InternalStoreCustody,
    );
    assert_eq!(
        OfflineRepairBlastRadiusObservation::from_offline_repair_report(
            native,
            StoreRepairPhysicalRegionDeclaration::raw("repair-region-a"),
            OfflineRepairEvidenceKind::SupportTruth,
        ),
        Err(OfflineRepairBlastRadiusObservationDenial::NotRawReportInput)
    );
}

fn boundary_fact(identity_key: &str, value: &str) -> StoreAspectBoundaryFact {
    let key = aspects().vocabulary().key(identity_key).unwrap();
    let contract = aspects()
        .contract()
        .for_key(key.clone())
        .identified_by(aspects().vocabulary().identity(1))
        .at_revision(aspects().vocabulary().revision(1))
        .scalar(ScalarAspectType::String);
    let value = match aspects()
        .validate()
        .against(&contract)
        .value(AspectValue::String(InternedString::from(value)))
    {
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
