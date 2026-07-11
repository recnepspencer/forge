use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_layout_indexes::layout_strategy_admission::{
    phase27_authenticity_rule, phase27_custody_rule, phase27_key_scope_rule,
    phase27_repair_blast_radius_rule, phase27_tenant_scope_rule,
};
use forge_store_security::{
    admitted_tenant_page_export_prepared_scope_for_layout_access_test,
    admitted_tenant_page_security_scope_for_layout_access_test,
    classify_iam_role_as_security_scope_source,
    classify_identity_provider_claim_as_security_scope_source,
    classify_operator_identity_as_security_scope_source, RepairBlastRadiusAuthorityPosture,
    SecurityCustodyLookupAccessShape, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreKeyVersionPosture, StoreSecurityScopeDenialKind, StoreTenantScope,
};

#[test]
fn phase27_reports_carry_grammar_admitted_point_lookup_contract() {
    let admitted = admitted_tenant_page_export_prepared_scope_for_layout_access_test();
    let tenant = admitted.admit_tenant_scope_layout(&phase27_tenant_scope_rule().unwrap());
    let key = admitted.admit_key_scope_layout(&phase27_key_scope_rule().unwrap());
    let authenticity = admitted.admit_authenticity_layout(&phase27_authenticity_rule().unwrap());
    let custody = admitted.admit_custody_layout(&phase27_custody_rule().unwrap());

    assert_eq!(
        tenant.family_id(),
        DurableArtifactFamilyId::SecurityCustodyLookup
    );
    assert_eq!(
        tenant.declared_access_shape(),
        SecurityCustodyLookupAccessShape::PointLookup
    );
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
    assert_eq!(tenant.exact_counters().requests(), 1);
}

#[test]
fn phase27_reports_preserve_denial_and_readiness_on_real_grammar_path() {
    let admitted = admitted_tenant_page_security_scope_for_layout_access_test();
    let tenant = admitted.admit_tenant_scope_layout(&phase27_tenant_scope_rule().unwrap());
    let custody = admitted.admit_custody_layout(&phase27_custody_rule().unwrap());

    assert_eq!(
        tenant
            .deny_authority_source(classify_identity_provider_claim_as_security_scope_source())
            .kind(),
        StoreSecurityScopeDenialKind::JwtSubjectIsNotTenantScope
    );
    assert_eq!(
        custody
            .deny_authority_source(classify_iam_role_as_security_scope_source())
            .kind(),
        StoreSecurityScopeDenialKind::IamRoleIsNotCustodyPosture
    );

    let repair = admitted_repair_region()
        .admit_repair_blast_radius_layout(&phase27_repair_blast_radius_rule().unwrap());
    assert_eq!(
        repair
            .deny_authority_source(classify_operator_identity_as_security_scope_source())
            .kind(),
        StoreSecurityScopeDenialKind::OperatorIdentityIsNotRepairAuthority
    );
    assert_eq!(
        repair.declared_access_shape(),
        SecurityCustodyLookupAccessShape::PointLookup
    );
    assert_eq!(
        repair.authority_posture(),
        RepairBlastRadiusAuthorityPosture::ReadinessOnly
    );
    assert_eq!(
        repair.security_boundary().tenant_scope(),
        StoreTenantScope::RepairBlastRadius
    );
    assert_eq!(
        repair.security_boundary().key_version_posture(),
        StoreKeyVersionPosture::Current
    );
}

fn admitted_repair_region() -> forge_store_security::StoreRepairPhysicalRegionWitness {
    use forge_foundational::{
        aspects, AspectContract, AspectKey, AspectValue, InternedString, ScalarAspectType,
    };
    use forge_proof::TransitionOutcome;
    use forge_store_aspect_native::{
        StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
        StorePhysicalBoundaryWitness,
    };
    use forge_store_authority::{require_current_store_authority, StoreCurrentAuthorityWitness};
    use forge_store_contracts::{
        StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
    };
    use forge_store_security::{StoreCustodyPosture, StoreRepairPhysicalRegionDeclaration};

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

    let authority = current_authority("store.s8.phase27.repair", "repair-region");
    match forge_store_security::StoreRepairPhysicalRegionWitness::admit_native(
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
