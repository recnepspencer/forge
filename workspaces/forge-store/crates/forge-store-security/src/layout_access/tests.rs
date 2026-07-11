use forge_foundational::{
    aspects, AspectContract, AspectKey, AspectValue, InternedString, ScalarAspectType,
};
use forge_proof::TransitionOutcome;
use forge_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StorePhysicalBoundaryWitness,
};
use forge_store_authority::{require_current_store_authority, StoreCurrentAuthorityWitness};
use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};

use crate::{
    classify_app_org_id_as_security_scope_source, classify_iam_role_as_security_scope_source,
    classify_identity_provider_claim_as_security_scope_source,
    classify_kms_key_id_as_security_scope_source,
    classify_operator_identity_as_security_scope_source,
    layout_access::{
        phase27_lookup_rule::SecurityCustodyLookupAccessShape,
        repair_blast_radius_family::RepairBlastRadiusAuthorityPosture,
    },
    security_scope_test_authority::{
        admitted_tenant_page_export_prepared_scope_for_layout_access_test,
        admitted_tenant_page_security_scope_for_layout_access_test,
    },
    AdmittedAuthenticityLayoutRule, AdmittedCustodyLayoutRule, AdmittedKeyScopeLayoutRule,
    AdmittedRepairBlastRadiusLayoutRule, AdmittedTenantScopeLayoutRule,
    StoreAuthenticityRequirementClass, StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture,
    StoreRepairPhysicalRegionDeclaration, StoreRepairPhysicalRegionWitness, StoreTenantScope,
};

#[test]
fn security_layout_reports_preserve_s51_posture_on_real_admission_path() {
    let admitted = admitted_tenant_page_export_prepared_scope_for_layout_access_test();

    let tenant = admitted.admit_tenant_scope_layout(&tenant_rule());
    let key = admitted.admit_key_scope_layout(&key_rule());
    let authenticity = admitted.admit_authenticity_layout(&authenticity_rule());
    let custody = admitted.admit_custody_layout(&custody_rule());

    assert_eq!(
        tenant.tenant_scope(),
        StoreTenantScope::TenantPhysicalBoundary
    );
    assert_eq!(key.key_scope(), StoreKeyScope::PageEnvelope);
    assert_eq!(
        authenticity.authenticity_requirement().class(),
        Some(StoreAuthenticityRequirementClass::AuthenticatedFrame)
    );
    assert_eq!(
        custody.custody_posture(),
        StoreCustodyPosture::ExportPrepared
    );
    assert_eq!(
        tenant.family_id(),
        DurableArtifactFamilyId::SecurityCustodyLookup
    );
    assert_eq!(
        tenant.declared_access_shape(),
        SecurityCustodyLookupAccessShape::PointLookup
    );
    assert_eq!(tenant.exact_counters().requests(), 1);
}

#[test]
fn security_layout_reports_deny_identity_provider_and_adjacent_strings_as_authority() {
    let admitted = admitted_tenant_page_security_scope_for_layout_access_test();
    let tenant = admitted.admit_tenant_scope_layout(&tenant_rule());
    let key = admitted.admit_key_scope_layout(&key_rule());
    let custody = admitted.admit_custody_layout(&custody_rule());
    let repair = admitted_repair_region().admit_repair_blast_radius_layout(&repair_rule());

    assert_eq!(
        tenant
            .deny_authority_source(classify_identity_provider_claim_as_security_scope_source())
            .kind(),
        crate::StoreSecurityScopeDenialKind::JwtSubjectIsNotTenantScope
    );
    assert_eq!(
        tenant
            .deny_authority_source(classify_app_org_id_as_security_scope_source())
            .kind(),
        crate::StoreSecurityScopeDenialKind::ApplicationOrgIdIsNotTenantScope
    );
    assert_eq!(
        key.deny_authority_source(classify_kms_key_id_as_security_scope_source())
            .kind(),
        crate::StoreSecurityScopeDenialKind::KmsKeyIdIsNotKeyScope
    );
    assert_eq!(
        custody
            .deny_authority_source(classify_iam_role_as_security_scope_source())
            .kind(),
        crate::StoreSecurityScopeDenialKind::IamRoleIsNotCustodyPosture
    );
    assert_eq!(
        repair
            .deny_authority_source(classify_operator_identity_as_security_scope_source())
            .kind(),
        crate::StoreSecurityScopeDenialKind::OperatorIdentityIsNotRepairAuthority
    );
}

#[test]
fn repair_blast_radius_layout_stays_readiness_only() {
    let report = admitted_repair_region().admit_repair_blast_radius_layout(&repair_rule());

    assert_eq!(
        report.authority_posture(),
        RepairBlastRadiusAuthorityPosture::ReadinessOnly
    );
    assert_eq!(
        report.security_boundary().tenant_scope(),
        StoreTenantScope::RepairBlastRadius
    );
    assert_eq!(
        report.security_boundary().key_version_posture(),
        StoreKeyVersionPosture::Current
    );
    assert_eq!(
        report.declared_access_shape(),
        SecurityCustodyLookupAccessShape::PointLookup
    );
    assert_eq!(report.exact_counters().requests(), 1);
}

const fn tenant_rule() -> AdmittedTenantScopeLayoutRule {
    AdmittedTenantScopeLayoutRule::internal_phase27()
}

const fn key_rule() -> AdmittedKeyScopeLayoutRule {
    AdmittedKeyScopeLayoutRule::internal_phase27()
}

const fn authenticity_rule() -> AdmittedAuthenticityLayoutRule {
    AdmittedAuthenticityLayoutRule::internal_phase27()
}

const fn custody_rule() -> AdmittedCustodyLayoutRule {
    AdmittedCustodyLayoutRule::internal_phase27()
}

const fn repair_rule() -> AdmittedRepairBlastRadiusLayoutRule {
    AdmittedRepairBlastRadiusLayoutRule::internal_phase27()
}

fn admitted_repair_region() -> StoreRepairPhysicalRegionWitness {
    let authority = current_authority("store.s8.phase27.repair", "repair-region");
    match StoreRepairPhysicalRegionWitness::admit_native(
        &authority,
        StoreRepairPhysicalRegionDeclaration::raw("region-001"),
        StoreKeyVersionPosture::Current,
        StoreCustodyPosture::InternalStoreCustody,
    ) {
        TransitionOutcome::Success(witness) => witness,
        outcome => {
            panic!("repair region witness should admit through production path: {outcome:?}")
        }
    }
}

fn current_authority(identity_key: &str, value: &str) -> StoreCurrentAuthorityWitness {
    require_current_store_authority(boundary_fact(identity_key, value))
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
) -> forge_foundational::ContractValidatedAspectArtifact {
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
        .expect("test physical authority scope should be valid"),
    )
    .expect("test physical boundary witness should admit")
}
