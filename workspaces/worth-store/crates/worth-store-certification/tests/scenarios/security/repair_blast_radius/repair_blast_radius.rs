use worth_foundational::{
    aspects, AspectContract, AspectKey, AspectValue, InternedString, ScalarAspectType,
};
use worth_proof::TransitionOutcome;
use worth_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StorePhysicalBoundaryWitness,
};
use worth_store_authority::{require_current_store_authority, StoreCurrentAuthorityWitness};
use worth_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};
use worth_store_offline_verifier::{
    OfflineRepairBlastRadiusObservation, OfflineRepairEvidenceKind,
};
use worth_store_operations::{
    RepairBlastRadiusDeclaration, RepairBlastRadiusDenial, RepairBlastRadiusPlan,
};
use worth_store_security::{
    classify_audit_record_as_security_scope_source, classify_iam_role_as_security_scope_source,
    classify_offline_verifier_evidence_as_security_scope_source,
    classify_operator_identity_as_security_scope_source, reject_non_store_security_scope_source,
    repair_blast_radius_authenticity, StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture,
    StoreRawSecurityScopeDeclaration, StoreRepairPhysicalRegionDeclaration,
    StoreSecurityScopeAdmissionDenial, StoreSecurityScopeAdmissionRebindRequired,
    StoreSecurityScopeDenialKind, StoreTenantScope,
};

#[test]
fn repair_read_plan_requires_admitted_store_blast_radius() {
    let authority = current_authority("cert.phase8.admitted");
    let declaration = RepairBlastRadiusDeclaration::native(
        &authority,
        StoreRepairPhysicalRegionDeclaration::raw("repair-region-a"),
        StoreKeyVersionPosture::Current,
        StoreCustodyPosture::InternalStoreCustody,
    )
    .expect("current Store blast radius should declare repair readiness");
    let region = declaration.physical_region().clone();
    let readiness = RepairBlastRadiusPlan::declare(declaration)
        .admit_with_store_blast_radius()
        .expect("current Store blast radius should admit repair readiness");
    let read_plan = readiness
        .prepare_repair_read(region)
        .expect("admitted region should prepare repair read");

    assert_eq!(
        read_plan.security_scope().key_scope(),
        StoreKeyScope::RepairScopeEnvelope
    );
    assert_eq!(
        read_plan.security_scope().tenant_scope(),
        StoreTenantScope::RepairBlastRadius
    );
    assert_eq!(read_plan.counters().repair_reads_prepared(), 1);
}

#[test]
fn correct_region_id_cannot_override_wrong_tenant_scope() {
    let authority = current_authority("cert.phase8.wrong-tenant");
    let raw = StoreRawSecurityScopeDeclaration::deserialized_unadmitted(
        authority.physical_witness(),
        StoreKeyScope::RepairScopeEnvelope,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::TenantPhysicalBoundary,
        Some(repair_blast_radius_authenticity()),
        Some(StoreCustodyPosture::InternalStoreCustody),
    );
    let observation = OfflineRepairBlastRadiusObservation::from_offline_repair_report(
        raw,
        StoreRepairPhysicalRegionDeclaration::raw("repair-region-a"),
        OfflineRepairEvidenceKind::RepairReadCloseout,
    )
    .expect("offline report should be raw evidence");
    let denial = RepairBlastRadiusDeclaration::from_offline_observation(&authority, observation)
        .expect_err("wrong tenant should deny before repair read");

    assert!(matches!(
        denial,
        RepairBlastRadiusDenial::SecurityScopeAdmissionDenied {
            source: StoreSecurityScopeAdmissionDenial::WrongTenantScope,
            ..
        }
    ));
}

#[test]
fn rebind_required_key_posture_preserves_rebind_topology() {
    let authority = current_authority("cert.phase8.rebind-key");
    let denial = RepairBlastRadiusDeclaration::native(
        &authority,
        StoreRepairPhysicalRegionDeclaration::raw("repair-region-a"),
        StoreKeyVersionPosture::RebindRequired,
        StoreCustodyPosture::InternalStoreCustody,
    )
    .expect_err("rebind-required key posture should deny before repair read");

    if let RepairBlastRadiusDenial::SecurityScopeAdmissionRebindRequired { source, counters } =
        denial
    {
        assert_eq!(
            source,
            StoreSecurityScopeAdmissionRebindRequired::KeyVersionRebindRequired(
                StoreKeyVersionPosture::RebindRequired
            )
        );
        assert_eq!(counters.key_rebind_required_rejections(), 1);
        assert_eq!(counters.repair_reads_prepared(), 0);
        assert_eq!(counters.repair_denied(), 1);
    } else {
        panic!("rebind-required repair admission must stay typed");
    }
}

#[test]
fn non_store_sources_cannot_mint_repair_readiness() {
    let operator = reject_non_store_security_scope_source(
        classify_operator_identity_as_security_scope_source(),
    );
    let iam = reject_non_store_security_scope_source(classify_iam_role_as_security_scope_source());
    let audit =
        reject_non_store_security_scope_source(classify_audit_record_as_security_scope_source());
    let offline = reject_non_store_security_scope_source(
        classify_offline_verifier_evidence_as_security_scope_source(),
    );

    assert_eq!(
        operator.kind(),
        StoreSecurityScopeDenialKind::OperatorIdentityIsNotRepairAuthority
    );
    assert_eq!(
        iam.kind(),
        StoreSecurityScopeDenialKind::IamRoleIsNotCustodyPosture
    );
    assert_eq!(
        audit.kind(),
        StoreSecurityScopeDenialKind::AuditRecordIsNotRepairAuthority
    );
    assert_eq!(
        offline.kind(),
        StoreSecurityScopeDenialKind::OfflineVerifierEvidenceIsNotRepairAuthority
    );
}

fn current_authority(label: &str) -> StoreCurrentAuthorityWitness {
    require_current_store_authority(boundary_fact(label, "current"))
}

fn boundary_fact(identity_key: &str, value: &str) -> StoreAspectBoundaryFact {
    let key = aspect_key(identity_key);
    let contract = scalar_string_contract(key.clone());
    let admitted_state = match aspects()
        .authoritative_state()
        .admit([validated_scalar_value(&contract, value)])
    {
        TransitionOutcome::Success(state) => state,
        outcome => panic!("state admission should succeed: {outcome:?}"),
    };

    StoreAspectBoundaryFact::from_admitted_state(
        StoreAspectIdentity::from_aspect_key(key),
        StoreAspectAuthorityInput::new(admitted_state, physical_witness()),
    )
    .expect("Store boundary fact should admit matching identity")
}

fn aspect_key(raw: &str) -> AspectKey {
    aspects().vocabulary().key(raw).unwrap()
}

fn scalar_string_contract(aspect_key: AspectKey) -> AspectContract {
    aspects()
        .contract()
        .for_key(aspect_key)
        .identified_by(aspects().vocabulary().identity(1))
        .at_revision(aspects().vocabulary().revision(1))
        .scalar(ScalarAspectType::String)
}

fn validated_scalar_value(
    contract: &AspectContract,
    raw_value: &str,
) -> worth_foundational::ContractValidatedAspectArtifact {
    match aspects()
        .validate()
        .against(contract)
        .value(AspectValue::String(InternedString::from(raw_value)))
    {
        TransitionOutcome::Success(value) => value,
        outcome => panic!("validation should succeed: {outcome:?}"),
    }
}

fn physical_witness() -> StorePhysicalBoundaryWitness {
    StorePhysicalBoundaryWitness::from_physical_authority(
        StorePhysicalAuthorityWitness::for_aspect_native_boundary(
            ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
        )
        .unwrap(),
    )
    .unwrap()
}
