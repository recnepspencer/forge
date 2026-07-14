use worth_proof::TransitionOutcome;
use worth_store_authority::StoreCurrentAuthorityWitness;
use worth_store_contracts::DurableArtifactFamilyId;

mod authority;

pub(crate) use authority::current_authority;

use super::{
    layout_evolution_binding, layout_migration_execution, layout_migration_operation,
    LayoutBindingRequest, LayoutMigrationExecutionRequest, LayoutMigrationReceipt,
    LayoutMigrationRequest, LayoutRollbackRequest,
};
use crate::{
    layout_declarations, ArtifactFamilyAuthorityWitness, LayoutBindingWitness,
    LayoutCompatibilityWindow, LayoutEvolutionDeclaration, LayoutInterruptionPolicy,
    LayoutReadCompatibilityPosture, LayoutVersion, LayoutWriteCompatibilityPosture,
};

use worth_store_compatibility::{
    compatibility_admission, ArtifactFormatVersion, ArtifactSemanticVersion,
    RollingUpgradeAdmissionPlan, RollingUpgradePolicy,
};
use worth_store_security::{
    admit_store_security_scope, StoreAuthenticityRequirement, StoreCustodyPosture, StoreKeyScope,
    StoreKeyVersionPosture, StoreSecurityScopeAdmissionExpectation,
    StoreSecurityScopeAdmissionRequest, StoreTenantScope,
};

pub(crate) fn declared_family() -> ArtifactFamilyAuthorityWitness {
    let declaration = layout_declarations().seed_family();
    let classification = layout_declarations().classify_family(declaration);
    layout_declarations()
        .require_production_authority(classification)
        .expect("seed family should stay authoritative")
}

pub(crate) fn other_declared_family() -> ArtifactFamilyAuthorityWitness {
    let declaration = layout_declarations()
        .declaration(DurableArtifactFamilyId::PublicationSnapshotImage)
        .expect("publication snapshot image family should stay declared");
    let classification = layout_declarations().classify_family(declaration);
    layout_declarations()
        .require_production_authority(classification)
        .expect("publication snapshot image family should stay authoritative")
}

pub(crate) fn version(format: u32, major: u16, minor: u16) -> LayoutVersion {
    LayoutVersion::new(
        ArtifactFormatVersion(format),
        ArtifactSemanticVersion::new(major, minor),
    )
}

pub(crate) fn declaration() -> LayoutEvolutionDeclaration {
    LayoutEvolutionDeclaration::new(
        declared_family(),
        version(7, 2, 1),
        LayoutCompatibilityWindow::new(
            ArtifactFormatVersion(5),
            ArtifactFormatVersion(7),
            ArtifactFormatVersion(7),
            LayoutReadCompatibilityPosture::ReadOldWriteNew,
            LayoutWriteCompatibilityPosture::WriteNewDuringRollingUpgrade,
        )
        .expect("test compatibility window should admit"),
        version(5, 1, 0),
        version(7, 2, 1),
        version(7, 2, 1),
        version(5, 1, 0),
        LayoutInterruptionPolicy::ResumeDeclaredMigration,
    )
}

pub(crate) fn binding(
    bound_version: LayoutVersion,
    observed_version: LayoutVersion,
    bound_authority: StoreCurrentAuthorityWitness,
) -> LayoutBindingWitness {
    admitted_binding(
        declaration(),
        layout_declarations().seed_family(),
        bound_version,
        observed_version,
        bound_authority,
    )
}

pub(crate) fn source_binding_for_declaration(
    declaration: LayoutEvolutionDeclaration,
    bound_authority: StoreCurrentAuthorityWitness,
) -> LayoutBindingWitness {
    let family = admitted_family_for_scope(
        declaration.family().declaration(),
        &bound_authority,
        StoreKeyScope::StoreManagedRoot,
        StoreTenantScope::StoreInternal,
    );
    let compatibility = compatibility_admission()
        .admit_rolling(RollingUpgradeAdmissionPlan::new(
            declaration.compatibility_window().artifact_window(),
            RollingUpgradePolicy::ReadOldWriteNew,
        ))
        .into_admitted()
        .expect("declared rolling window should admit");
    let catalog = crate::bootstrap::test_support::bootstrap_catalog_read_admission();
    let physical_source = current_publication_source(&bound_authority);

    layout_evolution_binding()
        .admit(LayoutBindingRequest::from_bootstrap_catalog(
            declaration,
            family,
            bound_authority,
            compatibility,
            physical_source,
            &catalog,
        ))
        .into_admitted()
        .expect("declared source binding should admit through the production owner")
}

pub(crate) fn other_family_binding(
    bound_version: LayoutVersion,
    observed_version: LayoutVersion,
    bound_authority: StoreCurrentAuthorityWitness,
) -> LayoutBindingWitness {
    let declaration = LayoutEvolutionDeclaration::new(
        other_declared_family(),
        declaration().layout_version(),
        declaration().compatibility_window(),
        declaration().migration_source(),
        declaration().migration_target(),
        declaration().rollback_source(),
        declaration().rollback_target(),
        declaration().interruption_policy(),
    );
    admitted_binding(
        declaration,
        other_declared_family().declaration(),
        bound_version,
        observed_version,
        bound_authority,
    )
}

pub(crate) fn migrated_binding(
    declaration: LayoutEvolutionDeclaration,
    authority: &StoreCurrentAuthorityWitness,
) -> LayoutMigrationReceipt {
    let request = migration_execution_request(declaration, authority);
    let mut publication =
        worth_store_physical_isolation::PhysicalRootPublicationRuntime::from_current_root(
            request.publication_source_root(),
        );
    layout_migration_execution(&mut publication)
        .execute(request)
        .into_published()
        .expect("migration fixture must publish through physical copy-on-write")
}

pub(crate) fn migration_execution_request(
    declaration: LayoutEvolutionDeclaration,
    authority: &StoreCurrentAuthorityWitness,
) -> LayoutMigrationExecutionRequest {
    migration_execution_request_for_publication(
        declaration,
        authority,
        "layout-migration-publication",
        1_901,
    )
}

pub(crate) fn migration_execution_request_for_publication(
    declaration: LayoutEvolutionDeclaration,
    authority: &StoreCurrentAuthorityWitness,
    operation_digest: &str,
    generation: u64,
) -> LayoutMigrationExecutionRequest {
    let source = binding(
        declaration.migration_source(),
        declaration.migration_source(),
        authority.clone(),
    );
    let plan = layout_migration_operation()
        .plan(migration_request(declaration, source), authority)
        .into_ready()
        .expect("migration fixture must plan through the ordinary owner");
    LayoutMigrationExecutionRequest::new(
        plan,
        publication_plan(authority, operation_digest, generation),
    )
}

pub(crate) fn migration_request(
    declaration: LayoutEvolutionDeclaration,
    binding: LayoutBindingWitness,
) -> LayoutMigrationRequest {
    let current_family = binding.admitted_family();
    LayoutMigrationRequest::new(declaration, binding, current_family)
}

pub(crate) fn rollback_request(
    declaration: LayoutEvolutionDeclaration,
    binding: LayoutBindingWitness,
) -> LayoutRollbackRequest {
    let current_family = binding.admitted_family();
    LayoutRollbackRequest::new(declaration, binding, current_family)
}

pub(crate) fn rollback_execution_request_for_publication(
    declaration: LayoutEvolutionDeclaration,
    authority: &StoreCurrentAuthorityWitness,
    operation_digest: &str,
    generation: u64,
) -> super::LayoutRollbackExecutionRequest {
    let migrated = migrated_binding(declaration, authority);
    let plan = super::layout_rollback_operation()
        .plan(
            rollback_request(declaration, migrated.target_binding().clone()),
            authority,
        )
        .into_ready()
        .expect("rollback fixture must plan through the ordinary owner");
    super::LayoutRollbackExecutionRequest::new(
        plan,
        rollback_publication_plan(authority, operation_digest, generation),
    )
}

pub(crate) fn admitted_family_for_scope(
    family_declaration: &'static crate::PhysicalArtifactFamilyDeclaration,
    authority: &StoreCurrentAuthorityWitness,
    key_scope: StoreKeyScope,
    tenant_scope: StoreTenantScope,
) -> crate::AdmittedPhysicalArtifactFamily {
    let expectation = StoreSecurityScopeAdmissionExpectation::new(
        key_scope,
        tenant_scope,
        StoreAuthenticityRequirement::not_required(),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let security = match admit_store_security_scope(StoreSecurityScopeAdmissionRequest::new(
        authority,
        key_scope,
        StoreKeyVersionPosture::Current,
        tenant_scope,
        StoreAuthenticityRequirement::not_required(),
        StoreCustodyPosture::InternalStoreCustody,
        expectation,
    )) {
        TransitionOutcome::Success(admitted) => admitted,
        outcome => panic!("migration security scope should admit: {outcome:?}"),
    };
    layout_declarations()
        .admit_physical_artifact_family(family_declaration, security.witnesses())
        .unwrap()
}

pub(crate) fn publication_plan(
    authority: &StoreCurrentAuthorityWitness,
    operation_digest: &str,
    generation: u64,
) -> worth_store_physical_isolation::CopyOnWritePublicationPlan {
    let inputs = publication_inputs(authority, operation_digest, generation);
    worth_store_test_support::harness::physical_isolation::publication::admitted_copy_on_write_plan(
        &inputs,
    )
}

pub(crate) fn rollback_publication_plan(
    authority: &StoreCurrentAuthorityWitness,
    operation_digest: &str,
    generation: u64,
) -> worth_store_physical_isolation::CopyOnWritePublicationPlan {
    let store = worth_store_physical_format::PhysicalStoreIdentity::from_aspect_identity(
        authority.identity().clone(),
    );
    let migration = publication_inputs(authority, "layout-migration-publication", 1_901);
    let rollback = worth_store_test_support::harness::physical_isolation::publication::successor_publication_inputs_for_store(
        &migration,
        &store,
        operation_digest,
        generation,
    );
    worth_store_test_support::harness::physical_isolation::publication::admitted_copy_on_write_plan(
        &rollback,
    )
}

fn current_publication_source(
    authority: &StoreCurrentAuthorityWitness,
) -> worth_store_physical_isolation::PublicationRootCandidate {
    publication_inputs(authority, "layout-binding-physical-source", 1_900).old_candidate
}

fn publication_inputs(
    authority: &StoreCurrentAuthorityWitness,
    operation_digest: &str,
    generation: u64,
) -> worth_store_test_support::harness::physical_isolation::publication::PublicationInputs {
    let store = worth_store_physical_format::PhysicalStoreIdentity::from_aspect_identity(
        authority.identity().clone(),
    );
    worth_store_test_support::harness::physical_isolation::publication::publication_inputs_for_store(
        &store,
        operation_digest,
        generation,
    )
}

pub(crate) fn other_family_migrated_binding(
    authority: &StoreCurrentAuthorityWitness,
) -> LayoutMigrationReceipt {
    let declared = LayoutEvolutionDeclaration::new(
        other_declared_family(),
        declaration().layout_version(),
        declaration().compatibility_window(),
        declaration().migration_source(),
        declaration().migration_target(),
        declaration().rollback_source(),
        declaration().rollback_target(),
        declaration().interruption_policy(),
    );
    let source = other_family_binding(
        declared.migration_source(),
        declared.migration_source(),
        authority.clone(),
    );
    let plan = layout_migration_operation()
        .plan(migration_request(declared, source), authority)
        .into_ready()
        .expect("migration fixture must plan through the ordinary owner");
    let publication = publication_plan(authority, "layout-migration-publication", 1_901);
    let request = LayoutMigrationExecutionRequest::new(plan, publication);
    let mut runtime =
        worth_store_physical_isolation::PhysicalRootPublicationRuntime::from_current_root(
            request.publication_source_root(),
        );
    layout_migration_execution(&mut runtime)
        .execute(request)
        .into_published()
        .expect("migration fixture must publish through physical copy-on-write")
}

fn admitted_binding(
    declaration: LayoutEvolutionDeclaration,
    family_declaration: &'static crate::PhysicalArtifactFamilyDeclaration,
    bound_version: LayoutVersion,
    observed_version: LayoutVersion,
    bound_authority: StoreCurrentAuthorityWitness,
) -> LayoutBindingWitness {
    try_admitted_binding(
        declaration,
        family_declaration,
        bound_version,
        observed_version,
        bound_authority,
    )
    .unwrap()
}

fn try_admitted_binding(
    declaration: LayoutEvolutionDeclaration,
    family_declaration: &'static crate::PhysicalArtifactFamilyDeclaration,
    bound_version: LayoutVersion,
    observed_version: LayoutVersion,
    bound_authority: StoreCurrentAuthorityWitness,
) -> Result<LayoutBindingWitness, super::LayoutEvolutionDenial> {
    assert_eq!(bound_version, declaration.migration_source());
    assert_eq!(observed_version, bound_version);
    let family = admitted_family_for_scope(
        family_declaration,
        &bound_authority,
        StoreKeyScope::StoreManagedRoot,
        StoreTenantScope::StoreInternal,
    );
    let compatibility = compatibility_admission()
        .admit_rolling(RollingUpgradeAdmissionPlan::new(
            declaration.compatibility_window().artifact_window(),
            RollingUpgradePolicy::ReadOldWriteNew,
        ))
        .into_admitted()
        .unwrap();
    let catalog = crate::bootstrap::test_support::bootstrap_catalog_read_admission();
    let physical_source = current_publication_source(&bound_authority);
    layout_evolution_binding()
        .admit(LayoutBindingRequest::from_bootstrap_catalog(
            declaration,
            family,
            bound_authority,
            compatibility,
            physical_source,
            &catalog,
        ))
        .into_admitted()
}
