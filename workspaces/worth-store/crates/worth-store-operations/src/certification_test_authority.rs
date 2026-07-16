use worth_proof::TransitionOutcome;
use worth_store_authority::StoreCurrentAuthorityWitness;
use worth_store_contracts::DurableArtifactFamilyId;
use worth_store_layout_indexes::declarations::layout_declarations;
use worth_store_layout_indexes::integrity::{
    layout_corruption, offline_readmission, LayoutReadmissionWitness, OfflineReadmissionView,
};
use worth_store_layout_indexes::materialization::{
    AdmittedLayoutMaterialization, RestoredArtifactMaterializationAdmissionCaseId,
};
use worth_store_layout_indexes::{
    access_planning, AdmittedPhysicalArtifactFamily, ObserveOwnerCase, OwnerCaseObservation,
};
use worth_store_offline_verifier::OfflineCustodyCapsuleObservation;
use worth_store_recovery_physics::ReopenedRecoveryArtifactAdmission;
use worth_store_security::{
    admit_readmitted_trust_boundary_security_scope, admit_store_security_scope,
    StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture, StoreRawSecurityScopeDeclaration,
    StoreSecurityScopeAdmissionExpectation, StoreSecurityScopeAdmissionRequest, StoreTenantScope,
    StoreTrustBoundaryCrossing,
};

use crate::backup::export::{backup_capsule_authenticity, current_authority, readmission_trigger};
use crate::{
    admit_restored_layout_materialization, BackupExportCustodyAdmission,
    BackupExportCustodyDeclaration, BackupExportCustodyMode, BackupImportCustodyReadmission,
    RestoredLayoutMaterializationObservation,
};

pub struct RestoreOwnerScenarioObservations {
    materialization: [OwnerCaseObservation<RestoredArtifactMaterializationAdmissionCaseId>; 4],
    integration: [RestoredLayoutMaterializationObservation; 3],
}

impl RestoreOwnerScenarioObservations {
    pub const fn materialization(
        &self,
    ) -> &[OwnerCaseObservation<RestoredArtifactMaterializationAdmissionCaseId>; 4] {
        &self.materialization
    }

    pub const fn integration(&self) -> &[RestoredLayoutMaterializationObservation; 3] {
        &self.integration
    }
}

pub fn execute_restore_owner_scenarios(
    catalog: &worth_store_layout_indexes::BootstrapCatalogReadAdmission,
    reopened: &ReopenedRecoveryArtifactAdmission,
) -> RestoreOwnerScenarioObservations {
    let authority = current_authority("store.physical.default_instance");
    let foreign = current_authority("store.physical.foreign_instance");
    let family = admitted_page_family(&authority);
    let other_family = admitted_root_family(&authority);
    let readmission = offline_layout_readmission(family, reopened);
    let other_readmission = offline_layout_readmission(other_family, reopened);
    let custody = readmitted_custody(&authority);
    let foreign_custody = readmitted_custody(&foreign);
    let wrong_custody = readmitted_page_custody(&authority);

    let materialization = [
        access_planning()
            .admit_restored_artifact_materialization(family, catalog, readmission, &custody)
            .owner_case_observation(),
        access_planning()
            .admit_restored_artifact_materialization(family, catalog, other_readmission, &custody)
            .owner_case_observation(),
        access_planning()
            .admit_restored_artifact_materialization(family, catalog, readmission, &wrong_custody)
            .owner_case_observation(),
        access_planning()
            .admit_restored_artifact_materialization(family, catalog, readmission, &foreign_custody)
            .owner_case_observation(),
    ];

    let physical_family = family.declaration().family();
    let integration = [
        admit_restored_layout_materialization(
            physical_family,
            family,
            catalog,
            reopened,
            &custody_admission(&authority),
        )
        .owner_case_observation(),
        admit_restored_layout_materialization(
            physical_family,
            family,
            catalog,
            reopened,
            &outbound_custody(&authority),
        )
        .owner_case_observation(),
        admit_restored_layout_materialization(
            physical_family,
            family,
            catalog,
            reopened,
            &custody_admission(&foreign),
        )
        .owner_case_observation(),
    ];

    RestoreOwnerScenarioObservations {
        materialization,
        integration,
    }
}

pub struct ImportPublicationScenarioPreparation {
    authority: StoreCurrentAuthorityWitness,
    materialization: AdmittedLayoutMaterialization,
}

impl ImportPublicationScenarioPreparation {
    pub const fn authority(&self) -> &StoreCurrentAuthorityWitness {
        &self.authority
    }

    pub fn into_materialization(self) -> AdmittedLayoutMaterialization {
        self.materialization
    }
}

pub fn prepare_import_publication_owner_scenario(
    catalog: &worth_store_layout_indexes::BootstrapCatalogReadAdmission,
    reopened: &ReopenedRecoveryArtifactAdmission,
) -> ImportPublicationScenarioPreparation {
    let authority = current_authority("store.physical.default_instance");
    let family = admitted_page_family(&authority);
    let materialization = admit_restored_layout_materialization(
        family.declaration().family(),
        family,
        catalog,
        reopened,
        &custody_admission(&authority),
    )
    .into_materialized()
    .expect("certification import materialization must admit");
    ImportPublicationScenarioPreparation {
        authority,
        materialization,
    }
}

fn admitted_page_family(
    authority: &worth_store_authority::StoreCurrentAuthorityWitness,
) -> AdmittedPhysicalArtifactFamily {
    admitted_family(
        authority,
        DurableArtifactFamilyId::PhysicalPage,
        StoreSecurityScopeAdmissionRequest::platform_page_envelope(
            authority,
            StoreKeyVersionPosture::Current,
            StoreCustodyPosture::InternalStoreCustody,
        ),
    )
}

fn admitted_root_family(
    authority: &worth_store_authority::StoreCurrentAuthorityWitness,
) -> AdmittedPhysicalArtifactFamily {
    admitted_family(
        authority,
        DurableArtifactFamilyId::PhysicalRootManifest,
        StoreSecurityScopeAdmissionRequest::new(
            authority,
            StoreKeyScope::StoreManagedRoot,
            StoreKeyVersionPosture::Current,
            StoreTenantScope::StoreInternal,
            worth_store_security::StoreAuthenticityRequirement::not_required(),
            StoreCustodyPosture::InternalStoreCustody,
            StoreSecurityScopeAdmissionExpectation::new(
                StoreKeyScope::StoreManagedRoot,
                StoreTenantScope::StoreInternal,
                worth_store_security::StoreAuthenticityRequirement::not_required(),
                StoreCustodyPosture::InternalStoreCustody,
            ),
        ),
    )
}

fn admitted_family(
    _authority: &worth_store_authority::StoreCurrentAuthorityWitness,
    family_id: DurableArtifactFamilyId,
    request: StoreSecurityScopeAdmissionRequest<'_>,
) -> AdmittedPhysicalArtifactFamily {
    let scope = match admit_store_security_scope(request) {
        TransitionOutcome::Success(scope) => scope,
        outcome => panic!("restore target security scope must admit: {outcome:?}"),
    };
    let declaration = layout_declarations().declaration(family_id).unwrap();
    layout_declarations()
        .admit_physical_artifact_family(declaration, scope.witnesses())
        .unwrap()
}

fn offline_layout_readmission(
    family: AdmittedPhysicalArtifactFamily,
    reopened: &ReopenedRecoveryArtifactAdmission,
) -> LayoutReadmissionWitness {
    let requirement = layout_corruption()
        .require_offline_readmission(family, reopened)
        .into_offline_readmission_requirement()
        .unwrap();
    let recovery = worth_store_recovery_physics::layout_readmission()
        .admit_offline(family.family_id(), reopened)
        .expect("offline recovery source must readmit");
    match offline_readmission().admit(requirement, recovery).view() {
        OfflineReadmissionView::Readmitted(witness) => *witness,
        OfflineReadmissionView::Denied(denial) => panic!("offline readmission failed: {denial:?}"),
    }
}

fn readmitted_custody(
    authority: &worth_store_authority::StoreCurrentAuthorityWitness,
) -> worth_store_security::StoreReadmittedSecurityScope {
    readmitted_scope(
        authority,
        StoreKeyScope::BackupExportEnvelope,
        StoreTenantScope::ImportReadmissionBoundary,
        backup_capsule_authenticity(),
    )
}

fn readmitted_page_custody(
    authority: &worth_store_authority::StoreCurrentAuthorityWitness,
) -> worth_store_security::StoreReadmittedSecurityScope {
    readmitted_scope(
        authority,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        worth_store_security::StoreAuthenticityRequirement::required(
            worth_store_security::StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
    )
}

fn readmitted_scope(
    authority: &worth_store_authority::StoreCurrentAuthorityWitness,
    key_scope: StoreKeyScope,
    tenant_scope: StoreTenantScope,
    authenticity: worth_store_security::StoreAuthenticityRequirement,
) -> worth_store_security::StoreReadmittedSecurityScope {
    let expectation = StoreSecurityScopeAdmissionExpectation::new(
        key_scope,
        tenant_scope,
        authenticity,
        StoreCustodyPosture::Readmitted,
    );
    let raw = StoreRawSecurityScopeDeclaration::deserialized_unadmitted(
        authority.physical_witness(),
        key_scope,
        StoreKeyVersionPosture::Current,
        tenant_scope,
        Some(authenticity),
        Some(StoreCustodyPosture::ImportedUnreadmitted),
    );
    let trigger = readmission_trigger(
        StoreTrustBoundaryCrossing::OfflineExportImport,
        raw,
        authority,
        expectation,
    );
    admit_readmitted_trust_boundary_security_scope(
        authority,
        raw,
        StoreKeyVersionPosture::Current,
        expectation,
        trigger,
    )
    .unwrap()
}

fn custody_admission(
    authority: &worth_store_authority::StoreCurrentAuthorityWitness,
) -> BackupExportCustodyAdmission {
    let expectation = StoreSecurityScopeAdmissionExpectation::new(
        StoreKeyScope::BackupExportEnvelope,
        StoreTenantScope::ImportReadmissionBoundary,
        backup_capsule_authenticity(),
        StoreCustodyPosture::Readmitted,
    );
    let raw = StoreRawSecurityScopeDeclaration::deserialized_unadmitted(
        authority.physical_witness(),
        StoreKeyScope::BackupExportEnvelope,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::ImportReadmissionBoundary,
        Some(backup_capsule_authenticity()),
        Some(StoreCustodyPosture::ImportedUnreadmitted),
    );
    let trigger = readmission_trigger(
        StoreTrustBoundaryCrossing::OfflineExportImport,
        raw,
        authority,
        expectation,
    );
    let observation = OfflineCustodyCapsuleObservation::from_deserialized_capsule(raw, trigger)
        .expect("offline custody observation must remain non-authoritative");
    BackupImportCustodyReadmission::new(observation)
        .readmit_with_current_authority(authority)
        .unwrap()
}

fn outbound_custody(
    authority: &worth_store_authority::StoreCurrentAuthorityWitness,
) -> BackupExportCustodyAdmission {
    BackupExportCustodyDeclaration::native(
        authority,
        BackupExportCustodyMode::Backup,
        StoreKeyVersionPosture::Current,
    )
    .unwrap()
    .admit_with_current_authority(authority)
    .unwrap()
}
