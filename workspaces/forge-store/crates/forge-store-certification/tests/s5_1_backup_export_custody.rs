use forge_foundational::{
    aspects, AspectContract, AspectKey, AspectValue, InternedString, ScalarAspectType,
};
use forge_proof::TransitionOutcome;
use forge_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StorePhysicalBoundaryWitness,
};
use forge_store_authority::{require_current_store_authority, StoreCurrentAuthorityWitness};
use forge_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};
use forge_store_offline_verifier::OfflineCustodyCapsuleObservation;
use forge_store_operations::{
    BackupExportCapsuleEmission, BackupExportCustodyDeclaration, BackupExportCustodyMode,
    BackupExportCustodyReadiness, BackupImportCustodyReadmission,
};
use forge_store_security::{
    store_backup_restore_boundary_fact, StoreAuthenticityRequirement,
    StoreAuthenticityRequirementClass, StoreBackupRestoreAfterKeyRotationBoundaryEvidence,
    StoreBackupRestoreAfterKeyRotationBoundaryFact, StoreCustodyPosture, StoreKeyScope,
    StoreKeyVersionPosture, StoreRawSecurityScopeDeclaration,
    StoreSecurityScopeAdmissionExpectation, StoreTenantScope, StoreTrustBoundaryReadmissionTrigger,
};

#[test]
fn certification_courtroom_exercises_backup_export_import_custody_readiness() {
    let authority = current_authority("cert.phase7.backup-export");
    let declaration = BackupExportCustodyDeclaration::native(
        &authority,
        BackupExportCustodyMode::Export,
        StoreKeyVersionPosture::Current,
    )
    .expect("current export declaration should build");

    assert_eq!(
        declaration.raw_declaration().custody_posture(),
        Some(StoreCustodyPosture::ExportPrepared)
    );

    let admission = declaration
        .admit_with_current_authority(&authority)
        .expect("export declaration should admit through custody path");
    let custody = BackupExportCustodyReadiness::from_admitted_custody(admission)
        .expect("backup custody readiness should admit");
    let emission = BackupExportCapsuleEmission::prepare(custody);

    assert_eq!(
        emission.security_scope().tenant_scope(),
        StoreTenantScope::BackupRestoreBoundary
    );
    assert_eq!(
        emission.security_scope().custody_posture(),
        StoreCustodyPosture::ExportPrepared
    );

    let raw = StoreRawSecurityScopeDeclaration::deserialized_unadmitted(
        authority.physical_witness(),
        StoreKeyScope::BackupExportEnvelope,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::ImportReadmissionBoundary,
        Some(backup_authenticity()),
        Some(StoreCustodyPosture::ImportedUnreadmitted),
    );
    let observation = OfflineCustodyCapsuleObservation::from_deserialized_capsule(
        raw,
        backup_restore_trigger(raw, &authority),
    )
    .expect("offline verifier should observe raw capsule declaration only");
    let readmitted = BackupImportCustodyReadmission::new(observation)
        .readmit_with_current_authority(&authority)
        .expect("restored bundle should regain current security scope by readmission");
    let custody = BackupExportCustodyReadiness::from_admitted_custody(readmitted)
        .expect("readmitted import custody should satisfy backup/export readiness");

    assert_eq!(
        custody.identity().tenant_scope(),
        StoreTenantScope::ImportReadmissionBoundary
    );
    assert_eq!(custody.custody_posture(), StoreCustodyPosture::Readmitted);
}

fn backup_restore_trigger(
    raw: StoreRawSecurityScopeDeclaration,
    authority: &StoreCurrentAuthorityWitness,
) -> StoreTrustBoundaryReadmissionTrigger {
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
            import_expectation(),
        )
        .expect("backup restore boundary fact should build"),
    )
}

fn import_expectation() -> StoreSecurityScopeAdmissionExpectation {
    StoreSecurityScopeAdmissionExpectation::new(
        StoreKeyScope::BackupExportEnvelope,
        StoreTenantScope::ImportReadmissionBoundary,
        backup_authenticity(),
        StoreCustodyPosture::Readmitted,
    )
}

const fn backup_authenticity() -> StoreAuthenticityRequirement {
    StoreAuthenticityRequirement::required(
        StoreAuthenticityRequirementClass::AuthenticatedBackupCapsule,
    )
}

fn current_authority(label: &str) -> StoreCurrentAuthorityWitness {
    require_current_store_authority(boundary_fact(label, "current"))
}

fn boundary_fact(identity_key: &str, value: &str) -> StoreAspectBoundaryFact {
    let key = aspect_key(identity_key);
    let contract = scalar_string_contract(key.clone());
    let value = validated_scalar_value(&contract, value);
    let state = match aspects().authoritative_state().admit([value]) {
        TransitionOutcome::Success(state) => state,
        outcome => panic!("state admission should succeed: {outcome:?}"),
    };

    StoreAspectBoundaryFact::from_admitted_state(
        StoreAspectIdentity::from_aspect_key(key),
        StoreAspectAuthorityInput::new(state, physical_witness()),
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
        .unwrap(),
    )
    .unwrap()
}
