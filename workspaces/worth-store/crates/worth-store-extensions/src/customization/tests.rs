use super::{
    layout_customization_catalog, ExtensionFamilyPosture,
    FutureLayoutCustomizationAdmissionRequest, FutureLayoutCustomizationDeferred,
    FutureLayoutCustomizationDenial, FutureLayoutTarget,
};
use worth_foundational::{aspects, AspectContract, AspectValue, InternedString, ScalarAspectType};
use worth_proof::TransitionOutcome;
use worth_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StorePhysicalBoundaryWitness,
};
use worth_store_authority::{require_current_store_authority, StoreCurrentAuthorityWitness};
use worth_store_contracts::{
    DurableArtifactFamilyId, StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
};
use worth_store_layout_indexes::customization::{
    FutureLayoutCapabilityRequest,
    FutureLayoutCustomizationDenial as StoreLayoutCustomizationDenial,
};
use worth_store_layout_indexes::declarations::layout_declarations;
use worth_store_layout_indexes::{AdmittedPhysicalArtifactFamily, AdmittedPhysicalKeyDomain};
use worth_store_security::{
    admit_store_security_scope, StoreAdmittedSecurityScope, StoreAuthenticityRequirement,
    StoreAuthenticityRequirementClass, StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture,
    StoreSecurityScopeAdmissionExpectation, StoreSecurityScopeAdmissionRequest, StoreTenantScope,
};

#[test]
fn extensions_registered_targets_preserve_store_customization_admission() {
    let (page_lifecycle, page_domain) = admit_strategy_scope(
        DurableArtifactFamilyId::PhysicalPage,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let target = layout_customization_catalog()
        .declare_stable_basis_read(ExtensionFamilyPosture::Registered, page_domain);
    let request = FutureLayoutCustomizationAdmissionRequest::new(target, page_lifecycle);

    match layout_customization_catalog().admit(request) {
        TransitionOutcome::Success(admitted) => {
            assert_eq!(admitted.target(), target);
            assert_eq!(
                admitted.store_admission().request().capability_request(),
                FutureLayoutCapabilityRequest::point_lookup(page_domain)
            );
        }
        other => panic!("registered target should admit store request: {other:?}"),
    }
}

#[test]
fn extensions_rebuild_and_rejected_targets_do_not_mint_store_authority() {
    let (page_lifecycle, page_domain) = admit_strategy_scope(
        DurableArtifactFamilyId::PhysicalPage,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let rebuild = FutureLayoutCustomizationAdmissionRequest::new(
        layout_customization_catalog()
            .declare_aspect_projection(ExtensionFamilyPosture::RebuildRequired, page_domain),
        page_lifecycle,
    );
    let rejected = FutureLayoutCustomizationAdmissionRequest::new(
        layout_customization_catalog()
            .declare_support_trust(ExtensionFamilyPosture::Rejected, page_domain),
        page_lifecycle,
    );

    assert_eq!(
        layout_customization_catalog().admit(rebuild),
        TransitionOutcome::Deferred(FutureLayoutCustomizationDeferred::TargetRebuildRequired {
            target: FutureLayoutTarget::AspectProjection,
        })
    );
    assert_eq!(
        layout_customization_catalog().admit(rejected),
        TransitionOutcome::Denied(FutureLayoutCustomizationDenial::TargetRejected {
            target: FutureLayoutTarget::SupportTrust,
        })
    );
}

#[test]
fn extensions_targets_own_capability_and_workload_semantics() {
    let (page_lifecycle, page_domain) = admit_strategy_scope(
        DurableArtifactFamilyId::PhysicalPage,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let support_trust = FutureLayoutCustomizationAdmissionRequest::new(
        layout_customization_catalog()
            .declare_support_trust(ExtensionFamilyPosture::Registered, page_domain),
        page_lifecycle,
    );
    let aspect_projection = FutureLayoutCustomizationAdmissionRequest::new(
        layout_customization_catalog()
            .declare_aspect_projection(ExtensionFamilyPosture::Registered, page_domain),
        page_lifecycle,
    );

    assert_eq!(
        layout_customization_catalog().admit(support_trust),
        TransitionOutcome::Denied(FutureLayoutCustomizationDenial::StoreDenied(Box::new(
            StoreLayoutCustomizationDenial::NoStrategySupportsRequestedCapability {
                capability: FutureLayoutCapabilityRequest::verifier_declared_scan(page_domain),
                key_domain: page_domain.witness(),
            }
        )))
    );
    assert_eq!(
        layout_customization_catalog().admit(aspect_projection),
        TransitionOutcome::Denied(FutureLayoutCustomizationDenial::StoreDenied(Box::new(
            StoreLayoutCustomizationDenial::RebuildableProjectionNotYetSupported {
                key_domain: page_domain.witness(),
            }
        )))
    );
}

fn admit_strategy_scope(
    family_id: DurableArtifactFamilyId,
    key_scope: StoreKeyScope,
    tenant_scope: StoreTenantScope,
    authenticity: StoreAuthenticityRequirement,
    custody: StoreCustodyPosture,
) -> (AdmittedPhysicalArtifactFamily, AdmittedPhysicalKeyDomain) {
    let security_scope = admitted_scope(key_scope, tenant_scope, authenticity, custody);
    let declaration = layout_declarations().declaration(family_id).unwrap();
    let family = layout_declarations()
        .admit_physical_artifact_family(declaration, security_scope.witnesses())
        .unwrap();
    let key_domain = layout_declarations()
        .admit_physical_key_domain(family, security_scope.witnesses())
        .unwrap();
    (family, key_domain)
}

fn admitted_scope(
    key_scope: StoreKeyScope,
    tenant_scope: StoreTenantScope,
    authenticity_requirement: StoreAuthenticityRequirement,
    custody_posture: StoreCustodyPosture,
) -> StoreAdmittedSecurityScope {
    let current_authority = current_authority("store.new.extension", "test-current");
    let expectation = StoreSecurityScopeAdmissionExpectation::new(
        key_scope,
        tenant_scope,
        authenticity_requirement,
        custody_posture,
    );
    let request = StoreSecurityScopeAdmissionRequest::new(
        &current_authority,
        key_scope,
        StoreKeyVersionPosture::Current,
        tenant_scope,
        authenticity_requirement,
        custody_posture,
        expectation,
    );

    match admit_store_security_scope(request) {
        TransitionOutcome::Success(admitted) => admitted,
        outcome => panic!("extension scope admission should succeed: {outcome:?}"),
    }
}

fn current_authority(identity_key: &str, value: &str) -> StoreCurrentAuthorityWitness {
    require_current_store_authority(boundary_fact(identity_key, value))
}

fn boundary_fact(identity_key: &str, value: &str) -> StoreAspectBoundaryFact {
    let key = aspects().vocabulary().key(identity_key).unwrap();
    let contract = aspects()
        .contract()
        .for_key(key.clone())
        .identified_by(aspects().vocabulary().identity(1))
        .at_revision(aspects().vocabulary().revision(1))
        .scalar(ScalarAspectType::String);
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
        .expect("test physical authority scope should be valid"),
    )
    .expect("test physical boundary witness should admit")
}
