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
#[cfg(test)]
use worth_store_security::{
    store_backup_restore_boundary_fact, store_custody_domain_boundary_fact,
    store_deployment_boundary_fact, store_instance_boundary_fact,
    store_key_scope_generation_boundary_fact, store_offline_transfer_boundary_fact,
    store_tenant_scope_authority_boundary_fact, StoreBackupRestoreAfterKeyRotationBoundaryEvidence,
    StoreBackupRestoreAfterKeyRotationBoundaryFact, StoreCustodyDomainBoundaryEvidence,
    StoreCustodyDomainBoundaryFact, StoreDifferentDeploymentBoundaryEvidence,
    StoreDifferentDeploymentBoundaryFact, StoreDifferentStoreInstanceBoundaryEvidence,
    StoreDifferentStoreInstanceBoundaryFact, StoreKeyScopeGenerationBoundaryEvidence,
    StoreKeyScopeGenerationBoundaryFact, StoreOfflineExportImportBoundaryEvidence,
    StoreOfflineExportImportBoundaryFact, StoreRawSecurityScopeDeclaration,
    StoreSecurityScopeAdmissionExpectation, StoreTenantScopeAuthorityBoundaryEvidence,
    StoreTenantScopeAuthorityBoundaryFact, StoreTrustBoundaryCrossing,
    StoreTrustBoundaryReadmissionTrigger,
};
pub(crate) fn current_authority(label: &str) -> StoreCurrentAuthorityWitness {
    require_current_store_authority(boundary_fact(label, "current"))
}

#[cfg(test)]
pub(crate) fn readmission_trigger(
    crossing: StoreTrustBoundaryCrossing,
    raw: StoreRawSecurityScopeDeclaration,
    authority: &StoreCurrentAuthorityWitness,
    expectation: StoreSecurityScopeAdmissionExpectation,
) -> StoreTrustBoundaryReadmissionTrigger {
    match crossing {
        StoreTrustBoundaryCrossing::DifferentDeployment => {
            StoreTrustBoundaryReadmissionTrigger::different_deployment(
                StoreDifferentDeploymentBoundaryFact::from_readmission_candidate(
                    StoreDifferentDeploymentBoundaryEvidence::from_category_facts(
                        store_deployment_boundary_fact(boundary_fact(
                            "store.trust_boundary.deployment",
                            "exported",
                        ))
                        .expect("exported deployment fact should admit"),
                        store_deployment_boundary_fact(boundary_fact(
                            "store.trust_boundary.deployment",
                            "current",
                        ))
                        .expect("current deployment fact should admit"),
                    )
                    .expect("different deployment category evidence should build"),
                    raw,
                    authority,
                    expectation,
                )
                .expect("different deployment boundary fact should build"),
            )
        }
        StoreTrustBoundaryCrossing::DifferentStoreInstance => {
            StoreTrustBoundaryReadmissionTrigger::different_store_instance(
                StoreDifferentStoreInstanceBoundaryFact::from_readmission_candidate(
                    StoreDifferentStoreInstanceBoundaryEvidence::from_category_facts(
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
                    .expect("different Store instance category evidence should build"),
                    raw,
                    authority,
                    expectation,
                )
                .expect("different Store instance boundary fact should build"),
            )
        }
        StoreTrustBoundaryCrossing::KeyScopeGenerationChanged => {
            StoreTrustBoundaryReadmissionTrigger::key_scope_generation_changed(
                StoreKeyScopeGenerationBoundaryFact::from_readmission_candidate(
                    StoreKeyScopeGenerationBoundaryEvidence::from_category_facts(
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
                    .expect("key-scope generation category evidence should build"),
                    raw,
                    authority,
                    expectation,
                )
                .expect("key-scope generation boundary fact should build"),
            )
        }
        StoreTrustBoundaryCrossing::TenantScopeAuthorityChanged => {
            StoreTrustBoundaryReadmissionTrigger::tenant_scope_authority_changed(
                StoreTenantScopeAuthorityBoundaryFact::from_readmission_candidate(
                    StoreTenantScopeAuthorityBoundaryEvidence::from_category_facts(
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
                    .expect("tenant-scope authority category evidence should build"),
                    raw,
                    authority,
                    expectation,
                )
                .expect("tenant-scope authority boundary fact should build"),
            )
        }
        StoreTrustBoundaryCrossing::CustodyDomainChanged => {
            StoreTrustBoundaryReadmissionTrigger::custody_domain_changed(
                StoreCustodyDomainBoundaryFact::from_readmission_candidate(
                    StoreCustodyDomainBoundaryEvidence::from_category_facts(
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
                    .expect("custody-domain category evidence should build"),
                    raw,
                    authority,
                    expectation,
                )
                .expect("custody-domain boundary fact should build"),
            )
        }
        StoreTrustBoundaryCrossing::OfflineExportImport => {
            StoreTrustBoundaryReadmissionTrigger::offline_export_import(
                StoreOfflineExportImportBoundaryFact::from_readmission_candidate(
                    StoreOfflineExportImportBoundaryEvidence::from_category_facts(
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
                    .expect("offline export/import category evidence should build"),
                    raw,
                    authority,
                    expectation,
                )
                .expect("offline export/import boundary fact should build"),
            )
        }
        StoreTrustBoundaryCrossing::BackupRestoreAfterKeyRotation => {
            StoreTrustBoundaryReadmissionTrigger::backup_restore_after_key_rotation(
                StoreBackupRestoreAfterKeyRotationBoundaryFact::from_readmission_candidate(
                    StoreBackupRestoreAfterKeyRotationBoundaryEvidence::from_category_facts(
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
                    .expect("backup restore category evidence should build"),
                    raw,
                    authority,
                    expectation,
                )
                .expect("backup restore boundary fact should build"),
            )
        }
    }
}

pub(crate) fn boundary_fact(identity_key: &str, value: &str) -> StoreAspectBoundaryFact {
    let key = aspect_key(identity_key);
    let contract = scalar_string_contract(key.clone());
    let value = validated_scalar_value(&contract, value);
    let admitted_state = match aspects().authoritative_state().admit([value]) {
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
