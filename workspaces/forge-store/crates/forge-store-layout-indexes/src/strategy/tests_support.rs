use crate::strategy::{S8AdmittedLayoutStrategy, S8LayoutStrategyFamily};
use crate::strategy_registry::{
    layout_admission_registry, S8LayoutAdmissionRequest, S8LayoutRequestedCapability,
};
use crate::{
    layout_declarations, ArtifactFamilyAccessLane, ArtifactFamilyLifecycleAdmission,
    CanonicalKeyBytes, PhysicalKeyDomainWitness,
};
use forge_foundational::{aspects, AspectContract, AspectValue, InternedString, ScalarAspectType};
use forge_proof::TransitionOutcome;
use forge_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StorePhysicalBoundaryWitness,
};
use forge_store_authority::{require_current_store_authority, StoreCurrentAuthorityWitness};
use forge_store_contracts::{
    DurableArtifactFamilyId, StorePhysicalAuthorityWitness, WalRecordFamily,
    ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
};
use forge_store_physical_format::{PhysicalPageId, PhysicalSegmentId};
use forge_store_security::{
    admit_store_security_scope, StoreAdmittedSecurityScope, StoreAuthenticityRequirement,
    StoreAuthenticityRequirementClass, StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture,
    StoreSecurityScopeAdmissionExpectation, StoreSecurityScopeAdmissionRequest, StoreTenantScope,
};
use forge_store_wal::StoreWalRecordIdentity;

pub(crate) fn admit_phase_five_scope(
    family_id: DurableArtifactFamilyId,
    key_scope: StoreKeyScope,
    tenant_scope: StoreTenantScope,
    authenticity: StoreAuthenticityRequirement,
    custody: StoreCustodyPosture,
) -> (ArtifactFamilyLifecycleAdmission, PhysicalKeyDomainWitness) {
    let security_scope = admitted_scope(key_scope, tenant_scope, authenticity, custody);
    let declaration = layout_declarations().declaration(family_id).unwrap();
    let classification = layout_declarations().classify_family(declaration);
    let authority = layout_declarations()
        .require_production_authority(classification)
        .unwrap();
    let lifecycle = layout_declarations()
        .require_strategy_lifecycle(authority)
        .unwrap();
    let scope = layout_declarations()
        .require_scope_partition(
            layout_declarations().declare_derived_accuracy_class(
                layout_declarations().declare_authority_role(classification),
            ),
            security_scope.witnesses(),
        )
        .unwrap();
    let key_domain = layout_declarations()
        .declare_physical_key_domain(scope)
        .unwrap();
    (lifecycle, key_domain)
}

pub(crate) fn admit_btree_page_strategy() -> S8AdmittedLayoutStrategy {
    let (lifecycle, key_domain) = admit_phase_five_scope(
        DurableArtifactFamilyId::PhysicalPage,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    );
    match layout_admission_registry().admit(S8LayoutAdmissionRequest::new(
        lifecycle,
        key_domain,
        S8LayoutStrategyFamily::BaselineBTreeRange,
        S8LayoutRequestedCapability::point_lookup(),
        ArtifactFamilyAccessLane::HotPath,
    )) {
        TransitionOutcome::Success(admitted) => admitted,
        outcome => panic!("btree strategy admission should succeed: {outcome:?}"),
    }
}

pub(crate) fn admit_lsm_wal_strategy() -> S8AdmittedLayoutStrategy {
    let (lifecycle, key_domain) = admit_phase_five_scope(
        DurableArtifactFamilyId::PublicationWalIntent,
        StoreKeyScope::WalCheckpointEnvelope,
        StoreTenantScope::StoreInternal,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedWalRecord,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    );
    match layout_admission_registry().admit(S8LayoutAdmissionRequest::new(
        lifecycle,
        key_domain,
        S8LayoutStrategyFamily::BaselineLsmWriteOptimized,
        S8LayoutRequestedCapability::point_lookup(),
        ArtifactFamilyAccessLane::HotPath,
    )) {
        TransitionOutcome::Success(admitted) => admitted,
        outcome => panic!("lsm strategy admission should succeed: {outcome:?}"),
    }
}

pub(crate) fn admitted_page_key_bytes(segment: u64, page: u64) -> CanonicalKeyBytes {
    let strategy = admit_btree_page_strategy();
    let domain = strategy.key_domain();
    let encoding = layout_declarations().require_canonical_key_encoding(domain);
    let comparator = layout_declarations().declare_comparator_law(encoding);
    let key = layout_declarations()
        .admit_page_address_key(domain, segment_id(segment), page_id(page))
        .unwrap();
    layout_declarations()
        .canonical_key_bytes(comparator, key)
        .unwrap()
}

pub(crate) fn admitted_wal_key_bytes(sequence: u64) -> CanonicalKeyBytes {
    let strategy = admit_lsm_wal_strategy();
    let domain = strategy.key_domain();
    let encoding = layout_declarations().require_canonical_key_encoding(domain);
    let comparator = layout_declarations().declare_comparator_law(encoding);
    let key = layout_declarations()
        .admit_wal_record_key(
            domain,
            WalRecordFamily::DurableMutationIntent,
            StoreWalRecordIdentity::new(sequence),
        )
        .unwrap();
    layout_declarations()
        .canonical_key_bytes(comparator, key)
        .unwrap()
}

pub(crate) fn root_manifest_scope() -> (ArtifactFamilyLifecycleAdmission, PhysicalKeyDomainWitness)
{
    admit_phase_five_scope(
        DurableArtifactFamilyId::PhysicalRootManifest,
        StoreKeyScope::StoreManagedRoot,
        StoreTenantScope::StoreInternal,
        StoreAuthenticityRequirement::not_required(),
        StoreCustodyPosture::InternalStoreCustody,
    )
}

fn admitted_scope(
    key_scope: StoreKeyScope,
    tenant_scope: StoreTenantScope,
    authenticity_requirement: StoreAuthenticityRequirement,
    custody_posture: StoreCustodyPosture,
) -> StoreAdmittedSecurityScope {
    let current_authority = current_authority("store.s8.strategy", "test-current");
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
        outcome => panic!("strategy scope admission should succeed: {outcome:?}"),
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

fn segment_id(value: u64) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(value).expect("test segment id is non-zero")
}

fn page_id(value: u64) -> PhysicalPageId {
    PhysicalPageId::from_raw(value).expect("test page id is non-zero")
}
