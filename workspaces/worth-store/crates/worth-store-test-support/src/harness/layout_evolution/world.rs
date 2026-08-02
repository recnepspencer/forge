use worth_proof::TransitionOutcome;
use worth_store_authority::StoreCurrentAuthorityWitness;
use worth_store_compatibility::{
    compatibility_admission, ArtifactCompatibilityWindow, ArtifactFormatVersion,
    ArtifactSemanticVersion, RollingUpgradeAdmissionPlan, RollingUpgradePolicy,
};
use worth_store_contracts::DurableArtifactFamilyId;
use worth_store_layout_indexes::evolution::migration::{
    layout_evolution_binding, LayoutBindingRequest, LayoutBindingWitness,
    LayoutCompatibilityWindow, LayoutEvolutionDeclaration, LayoutInterruptionPolicy,
    LayoutReadCompatibilityPosture, LayoutVersion, LayoutWriteCompatibilityPosture,
};
use worth_store_layout_indexes::{
    declarations::{layout_declarations, PhysicalArtifactFamilyDeclaration},
    AdmittedPhysicalArtifactFamily,
};
use worth_store_physical_isolation::PublicationRootCandidate;
use worth_store_security::{
    admit_store_security_scope, StoreAuthenticityRequirement, StoreCustodyPosture, StoreKeyScope,
    StoreKeyVersionPosture, StoreSecurityScopeAdmissionExpectation,
    StoreSecurityScopeAdmissionRequest, StoreTenantScope,
};

use super::authority::current_authority;
use crate::harness::{layout::admitted_layout_bootstrap_catalog, physical_isolation::publication};

pub(super) fn authority(label: &str) -> StoreCurrentAuthorityWitness {
    current_authority(label, "current")
}

pub(super) fn version(format: u32, major: u16, minor: u16) -> LayoutVersion {
    LayoutVersion::new(
        ArtifactFormatVersion(format),
        ArtifactSemanticVersion::new(major, minor),
    )
}

pub(super) fn declaration(policy: LayoutInterruptionPolicy) -> LayoutEvolutionDeclaration {
    let authority = authority("store.layout_evolution.declaration");
    LayoutEvolutionDeclaration::from_admitted_family(
        admitted_family(declared_family(), &authority),
        version(7, 2, 1),
        LayoutCompatibilityWindow::new(
            ArtifactFormatVersion(5),
            ArtifactFormatVersion(7),
            ArtifactFormatVersion(7),
            LayoutReadCompatibilityPosture::ReadOldWriteNew,
            LayoutWriteCompatibilityPosture::WriteNewDuringRollingUpgrade,
        )
        .unwrap(),
        version(5, 1, 0),
        version(7, 2, 1),
        version(7, 2, 1),
        version(5, 1, 0),
        policy,
    )
}

pub(super) fn declaration_for_family(
    family: &'static PhysicalArtifactFamilyDeclaration,
    policy: LayoutInterruptionPolicy,
) -> LayoutEvolutionDeclaration {
    let baseline = declaration(policy);
    let authority = authority("store.layout_evolution.foreign_declaration");
    LayoutEvolutionDeclaration::from_admitted_family(
        admitted_family(family, &authority),
        baseline.layout_version(),
        baseline.compatibility_window(),
        baseline.migration_source(),
        baseline.migration_target(),
        baseline.rollback_source(),
        baseline.rollback_target(),
        baseline.interruption_policy(),
    )
}

pub(super) fn declared_family() -> &'static PhysicalArtifactFamilyDeclaration {
    layout_declarations()
        .declaration(DurableArtifactFamilyId::PhysicalRootManifest)
        .unwrap()
}

pub(super) fn other_declared_family() -> &'static PhysicalArtifactFamilyDeclaration {
    layout_declarations()
        .declaration(DurableArtifactFamilyId::PublicationSnapshotImage)
        .unwrap()
}

pub(super) fn admitted_family(
    declaration: &'static PhysicalArtifactFamilyDeclaration,
    authority: &StoreCurrentAuthorityWitness,
) -> AdmittedPhysicalArtifactFamily {
    let expectation = StoreSecurityScopeAdmissionExpectation::new(
        StoreKeyScope::StoreManagedRoot,
        StoreTenantScope::StoreInternal,
        StoreAuthenticityRequirement::not_required(),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let security = match admit_store_security_scope(StoreSecurityScopeAdmissionRequest::new(
        authority,
        StoreKeyScope::StoreManagedRoot,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::StoreInternal,
        StoreAuthenticityRequirement::not_required(),
        StoreCustodyPosture::InternalStoreCustody,
        expectation,
    )) {
        TransitionOutcome::Success(value) => value,
        outcome => panic!("evolution security scope must admit: {outcome:?}"),
    };
    layout_declarations()
        .admit_physical_artifact_family(declaration, security.witnesses())
        .unwrap()
}

pub(super) fn compatibility(
    declaration: LayoutEvolutionDeclaration,
) -> worth_store_compatibility::RollingWindowCompatibilityReceipt {
    compatibility_admission()
        .admit_rolling(RollingUpgradeAdmissionPlan::new(
            declaration.compatibility_window().artifact_window(),
            RollingUpgradePolicy::ReadOldWriteNew,
        ))
        .into_admitted()
        .unwrap()
}

pub(super) fn foreign_compatibility() -> worth_store_compatibility::RollingWindowCompatibilityReceipt
{
    compatibility_admission()
        .admit_rolling(RollingUpgradeAdmissionPlan::new(
            ArtifactCompatibilityWindow::new(
                ArtifactFormatVersion(4),
                ArtifactFormatVersion(6),
                ArtifactFormatVersion(7),
            )
            .unwrap(),
            RollingUpgradePolicy::ReadOldWriteNew,
        ))
        .into_admitted()
        .unwrap()
}

pub(super) fn physical_inputs(
    authority: &StoreCurrentAuthorityWitness,
    generation: u64,
) -> publication::PublicationInputs {
    let store = worth_store_physical_format::PhysicalStoreIdentity::from_aspect_identity(
        authority.identity().clone(),
    );
    publication::publication_inputs_for_store(&store, generation)
}

pub(super) fn binding_outcome(
    declaration: LayoutEvolutionDeclaration,
    family: AdmittedPhysicalArtifactFamily,
    authority: StoreCurrentAuthorityWitness,
    compatibility: worth_store_compatibility::RollingWindowCompatibilityReceipt,
    source: PublicationRootCandidate,
) -> worth_store_layout_indexes::evolution::migration::LayoutBindingAdmissionOutcome {
    let catalog = admitted_layout_bootstrap_catalog();
    layout_evolution_binding().admit(LayoutBindingRequest::from_bootstrap_catalog(
        declaration,
        family,
        authority,
        compatibility,
        source,
        &catalog,
    ))
}

pub(super) fn source_binding(
    declaration: LayoutEvolutionDeclaration,
    authority: &StoreCurrentAuthorityWitness,
) -> LayoutBindingWitness {
    let family = admitted_family(declaration.family_declaration(), authority);
    let source = physical_inputs(authority, 10_000).old_candidate;
    binding_outcome(
        declaration,
        family,
        authority.clone(),
        compatibility(declaration),
        source,
    )
    .into_admitted()
    .unwrap()
}

pub(super) fn source_binding_at_version(
    declaration: LayoutEvolutionDeclaration,
    authority: &StoreCurrentAuthorityWitness,
    version: LayoutVersion,
) -> LayoutBindingWitness {
    let observation_declaration = LayoutEvolutionDeclaration::from_admitted_family(
        admitted_family(declaration.family_declaration(), authority),
        declaration.layout_version(),
        declaration.compatibility_window(),
        version,
        version,
        declaration.rollback_source(),
        declaration.rollback_target(),
        declaration.interruption_policy(),
    );
    source_binding(observation_declaration, authority)
}
