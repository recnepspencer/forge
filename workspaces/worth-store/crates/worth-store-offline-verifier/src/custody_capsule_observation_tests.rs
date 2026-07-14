use worth_foundational::{aspects, AspectValue, InternedString, ScalarAspectType};
use worth_proof::TransitionOutcome;
use worth_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StorePhysicalBoundaryWitness,
};
use worth_store_authority::{require_current_store_authority, StoreCurrentAuthorityWitness};
use worth_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};
use worth_store_security::{
    store_backup_restore_boundary_fact, store_custody_domain_boundary_fact,
    store_deployment_boundary_fact, store_instance_boundary_fact,
    store_key_scope_generation_boundary_fact, store_offline_transfer_boundary_fact,
    store_tenant_scope_authority_boundary_fact, StoreAuthenticityRequirement,
    StoreAuthenticityRequirementClass, StoreBackupRestoreAfterKeyRotationBoundaryEvidence,
    StoreBackupRestoreAfterKeyRotationBoundaryFact, StoreBackupRestoreBoundaryFactInput,
    StoreCustodyDomainBoundaryEvidence, StoreCustodyDomainBoundaryFact,
    StoreCustodyDomainBoundaryFactInput, StoreCustodyPosture, StoreDeploymentBoundaryFact,
    StoreDifferentDeploymentBoundaryEvidence, StoreDifferentDeploymentBoundaryFact,
    StoreDifferentStoreInstanceBoundaryEvidence, StoreDifferentStoreInstanceBoundaryFact,
    StoreKeyScope, StoreKeyScopeGenerationBoundaryEvidence, StoreKeyScopeGenerationBoundaryFact,
    StoreKeyScopeGenerationBoundaryFactInput, StoreKeyVersionPosture,
    StoreOfflineExportImportBoundaryEvidence, StoreOfflineExportImportBoundaryFact,
    StoreOfflineTransferBoundaryFact, StoreRawSecurityScopeDeclaration,
    StoreSecurityScopeAdmissionExpectation, StoreStoreInstanceBoundaryFact, StoreTenantScope,
    StoreTenantScopeAuthorityBoundaryEvidence, StoreTenantScopeAuthorityBoundaryFact,
    StoreTenantScopeAuthorityBoundaryFactInput, StoreTrustBoundaryCrossing,
    StoreTrustBoundaryReadmissionTrigger,
};

use crate::custody_capsule_observation::OfflineCustodyCapsuleObservationDenial;
use crate::OfflineCustodyCapsuleObservation;

#[test]
fn offline_custody_observation_accepts_only_deserialized_raw_capsule_input() {
    let authority = current_authority();
    let physical_witness = authority.physical_witness();
    let raw = StoreRawSecurityScopeDeclaration::deserialized_unadmitted(
        physical_witness,
        StoreKeyScope::BackupExportEnvelope,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::ImportReadmissionBoundary,
        Some(StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedBackupCapsule,
        )),
        Some(StoreCustodyPosture::ImportedUnreadmitted),
    );
    let observed = OfflineCustodyCapsuleObservation::from_deserialized_capsule(
        raw,
        trigger(
            StoreTrustBoundaryCrossing::OfflineExportImport,
            raw,
            &authority,
        ),
    )
    .expect("raw deserialized capsule should observe");

    assert_eq!(observed.raw_declaration(), raw);
    assert_eq!(
        observed.readmission_trigger().crossing(),
        worth_store_security::StoreTrustBoundaryCrossing::OfflineExportImport
    );

    let native = StoreRawSecurityScopeDeclaration::native(
        physical_witness,
        StoreKeyScope::BackupExportEnvelope,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::ImportReadmissionBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedBackupCapsule,
        ),
        StoreCustodyPosture::Readmitted,
    );
    assert_eq!(
        OfflineCustodyCapsuleObservation::from_deserialized_capsule(
            native,
            trigger(
                StoreTrustBoundaryCrossing::DifferentDeployment,
                raw,
                &authority,
            ),
        ),
        Err(OfflineCustodyCapsuleObservationDenial::NotDeserializedRawInput)
    );
}

fn trigger(
    crossing: StoreTrustBoundaryCrossing,
    raw: StoreRawSecurityScopeDeclaration,
    authority: &StoreCurrentAuthorityWitness,
) -> StoreTrustBoundaryReadmissionTrigger {
    match crossing {
        StoreTrustBoundaryCrossing::DifferentDeployment => {
            let (exported_fact, current_fact) = deployment_facts();
            StoreTrustBoundaryReadmissionTrigger::different_deployment(
                StoreDifferentDeploymentBoundaryFact::from_readmission_candidate(
                    StoreDifferentDeploymentBoundaryEvidence::from_category_facts(
                        exported_fact,
                        current_fact,
                    )
                    .expect("deployment boundary category evidence should build"),
                    raw,
                    authority,
                    import_expectation(),
                )
                .expect("different deployment boundary fact should build"),
            )
        }
        StoreTrustBoundaryCrossing::DifferentStoreInstance => {
            let (exported_fact, current_fact) = store_instance_facts();
            StoreTrustBoundaryReadmissionTrigger::different_store_instance(
                StoreDifferentStoreInstanceBoundaryFact::from_readmission_candidate(
                    StoreDifferentStoreInstanceBoundaryEvidence::from_category_facts(
                        exported_fact,
                        current_fact,
                    )
                    .expect("Store instance boundary category evidence should build"),
                    raw,
                    authority,
                    import_expectation(),
                )
                .expect("different Store instance boundary fact should build"),
            )
        }
        StoreTrustBoundaryCrossing::KeyScopeGenerationChanged => {
            let (exported_fact, current_fact) = key_generation_facts();
            StoreTrustBoundaryReadmissionTrigger::key_scope_generation_changed(
                StoreKeyScopeGenerationBoundaryFact::from_readmission_candidate(
                    StoreKeyScopeGenerationBoundaryEvidence::from_category_facts(
                        exported_fact,
                        current_fact,
                    )
                    .expect("key-scope generation category evidence should build"),
                    raw,
                    authority,
                    import_expectation(),
                )
                .expect("key-scope generation boundary fact should build"),
            )
        }
        StoreTrustBoundaryCrossing::TenantScopeAuthorityChanged => {
            let (exported_fact, current_fact) = tenant_authority_facts();
            StoreTrustBoundaryReadmissionTrigger::tenant_scope_authority_changed(
                StoreTenantScopeAuthorityBoundaryFact::from_readmission_candidate(
                    StoreTenantScopeAuthorityBoundaryEvidence::from_category_facts(
                        exported_fact,
                        current_fact,
                    )
                    .expect("tenant-scope authority category evidence should build"),
                    raw,
                    authority,
                    import_expectation(),
                )
                .expect("tenant-scope authority boundary fact should build"),
            )
        }
        StoreTrustBoundaryCrossing::CustodyDomainChanged => {
            let (exported_fact, current_fact) = custody_domain_facts();
            StoreTrustBoundaryReadmissionTrigger::custody_domain_changed(
                StoreCustodyDomainBoundaryFact::from_readmission_candidate(
                    StoreCustodyDomainBoundaryEvidence::from_category_facts(
                        exported_fact,
                        current_fact,
                    )
                    .expect("custody-domain category evidence should build"),
                    raw,
                    authority,
                    import_expectation(),
                )
                .expect("custody-domain boundary fact should build"),
            )
        }
        StoreTrustBoundaryCrossing::OfflineExportImport => {
            let (exported_fact, current_fact) = offline_transfer_facts();
            StoreTrustBoundaryReadmissionTrigger::offline_export_import(
                StoreOfflineExportImportBoundaryFact::from_readmission_candidate(
                    StoreOfflineExportImportBoundaryEvidence::from_category_facts(
                        exported_fact,
                        current_fact,
                    )
                    .expect("offline export/import category evidence should build"),
                    raw,
                    authority,
                    import_expectation(),
                )
                .expect("offline export/import boundary fact should build"),
            )
        }
        StoreTrustBoundaryCrossing::BackupRestoreAfterKeyRotation => {
            let (exported_fact, current_fact) = backup_restore_facts();
            StoreTrustBoundaryReadmissionTrigger::backup_restore_after_key_rotation(
                StoreBackupRestoreAfterKeyRotationBoundaryFact::from_readmission_candidate(
                    StoreBackupRestoreAfterKeyRotationBoundaryEvidence::from_category_facts(
                        exported_fact,
                        current_fact,
                    )
                    .expect("backup restore category evidence should build"),
                    raw,
                    authority,
                    import_expectation(),
                )
                .expect("backup restore boundary fact should build"),
            )
        }
    }
}

fn deployment_facts() -> (StoreDeploymentBoundaryFact, StoreDeploymentBoundaryFact) {
    (
        store_deployment_boundary_fact(boundary_fact(
            "store.trust_boundary.deployment",
            "exported",
        ))
        .expect("exported deployment fact should admit"),
        store_deployment_boundary_fact(boundary_fact("store.trust_boundary.deployment", "current"))
            .expect("current deployment fact should admit"),
    )
}

fn store_instance_facts() -> (
    StoreStoreInstanceBoundaryFact,
    StoreStoreInstanceBoundaryFact,
) {
    (
        store_instance_boundary_fact(boundary_fact(
            "store.trust_boundary.store_instance",
            "exported",
        ))
        .expect("exported Store instance fact should admit"),
        store_instance_boundary_fact(boundary_fact(
            "store.trust_boundary.store_instance",
            "current",
        ))
        .expect("current Store instance fact should admit"),
    )
}

fn key_generation_facts() -> (
    StoreKeyScopeGenerationBoundaryFactInput,
    StoreKeyScopeGenerationBoundaryFactInput,
) {
    (
        store_key_scope_generation_boundary_fact(boundary_fact(
            "store.trust_boundary.key_scope_generation",
            "exported",
        ))
        .expect("exported key generation fact should admit"),
        store_key_scope_generation_boundary_fact(boundary_fact(
            "store.trust_boundary.key_scope_generation",
            "current",
        ))
        .expect("current key generation fact should admit"),
    )
}

fn tenant_authority_facts() -> (
    StoreTenantScopeAuthorityBoundaryFactInput,
    StoreTenantScopeAuthorityBoundaryFactInput,
) {
    (
        store_tenant_scope_authority_boundary_fact(boundary_fact(
            "store.trust_boundary.tenant_scope_authority",
            "exported",
        ))
        .expect("exported tenant authority fact should admit"),
        store_tenant_scope_authority_boundary_fact(boundary_fact(
            "store.trust_boundary.tenant_scope_authority",
            "current",
        ))
        .expect("current tenant authority fact should admit"),
    )
}

fn custody_domain_facts() -> (
    StoreCustodyDomainBoundaryFactInput,
    StoreCustodyDomainBoundaryFactInput,
) {
    (
        store_custody_domain_boundary_fact(boundary_fact(
            "store.trust_boundary.custody_domain",
            "exported",
        ))
        .expect("exported custody domain fact should admit"),
        store_custody_domain_boundary_fact(boundary_fact(
            "store.trust_boundary.custody_domain",
            "current",
        ))
        .expect("current custody domain fact should admit"),
    )
}

fn offline_transfer_facts() -> (
    StoreOfflineTransferBoundaryFact,
    StoreOfflineTransferBoundaryFact,
) {
    (
        store_offline_transfer_boundary_fact(boundary_fact(
            "store.trust_boundary.offline_transfer",
            "exported",
        ))
        .expect("exported offline transfer fact should admit"),
        store_offline_transfer_boundary_fact(boundary_fact(
            "store.trust_boundary.offline_transfer",
            "current",
        ))
        .expect("current offline transfer fact should admit"),
    )
}

fn backup_restore_facts() -> (
    StoreBackupRestoreBoundaryFactInput,
    StoreBackupRestoreBoundaryFactInput,
) {
    (
        store_backup_restore_boundary_fact(boundary_fact(
            "store.trust_boundary.backup_restore",
            "exported",
        ))
        .expect("exported backup restore fact should admit"),
        store_backup_restore_boundary_fact(boundary_fact(
            "store.trust_boundary.backup_restore",
            "current",
        ))
        .expect("current backup restore fact should admit"),
    )
}

fn import_expectation() -> StoreSecurityScopeAdmissionExpectation {
    StoreSecurityScopeAdmissionExpectation::new(
        StoreKeyScope::BackupExportEnvelope,
        StoreTenantScope::ImportReadmissionBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedBackupCapsule,
        ),
        StoreCustodyPosture::Readmitted,
    )
}

fn current_authority() -> StoreCurrentAuthorityWitness {
    require_current_store_authority(boundary_fact("offline.capsule", "current"))
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
