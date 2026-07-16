use worth_foundational::{
    aspects, AspectContract, AspectKey, AspectValue, InternedString, ScalarAspectType,
};
use worth_proof::TransitionOutcome;
use worth_store_aspect_native::StorePhysicalBoundaryWitness;
use worth_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
};
use worth_store_authority::{require_current_store_authority, StoreCurrentAuthorityWitness};
use worth_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};

use crate::{
    admit_readmitted_trust_boundary_security_scope, admit_store_security_scope,
    store_offline_transfer_boundary_fact, StoreAdmittedSecurityScope, StoreAuthenticityRequirement,
    StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture,
    StoreOfflineExportImportBoundaryEvidence, StoreOfflineExportImportBoundaryFact,
    StoreRawSecurityScopeDeclaration, StoreReadmittedSecurityScope,
    StoreSecurityScopeAdmissionExpectation, StoreSecurityScopeAdmissionRequest, StoreTenantScope,
};

pub fn readmitted_wal_security_scope_for_test() -> StoreReadmittedSecurityScope {
    readmitted_wal_security_scope("store.physical.default_instance")
}

pub fn readmitted_foreign_wal_security_scope_for_test() -> StoreReadmittedSecurityScope {
    readmitted_wal_security_scope("store.physical.foreign_instance")
}

fn readmitted_wal_security_scope(identity_key: &str) -> StoreReadmittedSecurityScope {
    let authority = current_authority(identity_key, "test-current");
    let authenticity = StoreAuthenticityRequirement::required(
        crate::StoreAuthenticityRequirementClass::AuthenticatedWalRecord,
    );
    let expectation = StoreSecurityScopeAdmissionExpectation::new(
        StoreKeyScope::WalCheckpointEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        authenticity,
        StoreCustodyPosture::Readmitted,
    );
    let raw = StoreRawSecurityScopeDeclaration::deserialized_unadmitted(
        authority.physical_witness(),
        StoreKeyScope::WalCheckpointEnvelope,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::TenantPhysicalBoundary,
        Some(authenticity),
        Some(StoreCustodyPosture::ImportedUnreadmitted),
    );
    let trigger = crate::StoreTrustBoundaryReadmissionTrigger::offline_export_import(
        StoreOfflineExportImportBoundaryFact::from_readmission_candidate(
            StoreOfflineExportImportBoundaryEvidence::from_category_facts(
                store_offline_transfer_boundary_fact(boundary_fact(
                    "store.trust_boundary.offline_transfer",
                    "exported",
                ))
                .expect("exported transfer fact must admit"),
                store_offline_transfer_boundary_fact(boundary_fact(
                    "store.trust_boundary.offline_transfer",
                    "current",
                ))
                .expect("current transfer fact must admit"),
            )
            .expect("different offline transfer facts must bind"),
            raw,
            &authority,
            expectation,
        )
        .expect("readmission trigger must bind to its candidate"),
    );
    admit_readmitted_trust_boundary_security_scope(
        &authority,
        raw,
        StoreKeyVersionPosture::Current,
        expectation,
        trigger,
    )
    .expect("test WAL scope must readmit through the production trust boundary")
}

pub fn admitted_store_internal_security_scope_for_io_qos_test() -> StoreAdmittedSecurityScope {
    admitted_scope(
        StoreKeyScope::StoreManagedRoot,
        StoreTenantScope::StoreInternal,
        StoreAuthenticityRequirement::not_required(),
        StoreCustodyPosture::InternalStoreCustody,
    )
}

pub fn admitted_security_scope_for_identity_for_test(
    identity: crate::StoreSecurityScopeIdentity,
) -> StoreAdmittedSecurityScope {
    assert_eq!(identity.physical_witness(), physical_witness());
    assert_eq!(
        identity.key_version_posture(),
        StoreKeyVersionPosture::Current
    );
    admitted_scope(
        identity.key_scope(),
        identity.tenant_scope(),
        identity.authenticity_requirement(),
        identity.custody_posture(),
    )
}

pub fn admitted_wrong_io_qos_security_scope_for_test() -> StoreAdmittedSecurityScope {
    admitted_scope(
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            crate::StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    )
}

pub fn admitted_store_managed_root_security_scope_for_layout_partition_test(
) -> StoreAdmittedSecurityScope {
    admitted_store_internal_security_scope_for_io_qos_test()
}

pub fn admitted_tenant_page_security_scope_for_layout_partition_test() -> StoreAdmittedSecurityScope
{
    admitted_wrong_io_qos_security_scope_for_test()
}

pub fn admitted_tenant_artifact_security_scope_for_layout_partition_test(
) -> StoreAdmittedSecurityScope {
    admitted_scope(
        StoreKeyScope::ArtifactEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            crate::StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    )
}

pub fn admitted_tenant_wal_checkpoint_security_scope_for_layout_partition_test(
) -> StoreAdmittedSecurityScope {
    admitted_scope(
        StoreKeyScope::WalCheckpointEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            crate::StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    )
}

pub fn admitted_store_wal_checkpoint_security_scope_for_layout_partition_test(
) -> StoreAdmittedSecurityScope {
    admitted_scope(
        StoreKeyScope::WalCheckpointEnvelope,
        StoreTenantScope::StoreInternal,
        StoreAuthenticityRequirement::required(
            crate::StoreAuthenticityRequirementClass::AuthenticatedWalRecord,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    )
}

pub fn admitted_tenant_page_without_authenticity_for_layout_partition_test(
) -> StoreAdmittedSecurityScope {
    admitted_scope(
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::not_required(),
        StoreCustodyPosture::InternalStoreCustody,
    )
}

pub fn admitted_tenant_page_export_prepared_scope_for_layout_partition_test(
) -> StoreAdmittedSecurityScope {
    admitted_scope(
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            crate::StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::ExportPrepared,
    )
}

fn admitted_scope(
    key_scope: StoreKeyScope,
    tenant_scope: StoreTenantScope,
    authenticity_requirement: StoreAuthenticityRequirement,
    custody_posture: StoreCustodyPosture,
) -> StoreAdmittedSecurityScope {
    let current_authority = current_authority("store.physical.default_instance", "test-current");
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
        outcome => panic!("test security scope should admit through production path: {outcome:?}"),
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
