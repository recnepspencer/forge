use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_layout_indexes::declarations::ArtifactFamilyAccessLane;
use forge_store_layout_indexes::strategy_declarations::{
    layout_admission_registry, LayoutAdmissionRequest, LayoutRequestedCapability,
    LayoutStrategyFamily,
};
use forge_store_layout_indexes::{
    IndexMaintenanceMode, LayoutStrategyRegistrySnapshot, PhysicalMutationShape,
};
use forge_store_security::{
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreTenantScope,
};
use forge_store_test_support::SecurityScopeFixtureAuthority;

use super::super::fixture_admission::{admit_family, admit_key_domain, security_scope};

pub(super) fn btree_strategy(
    mode: IndexMaintenanceMode,
    shape: PhysicalMutationShape,
) -> LayoutStrategyRegistrySnapshot {
    let security = page_security();
    let family = admit_family(DurableArtifactFamilyId::PhysicalPage, &security);
    let domain = admit_key_domain(family, &security);
    layout_admission_registry()
        .admit(
            LayoutAdmissionRequest::from_admitted(
                family,
                domain,
                LayoutStrategyFamily::BaselineBTreeRange,
                LayoutRequestedCapability::point_lookup(),
                lane_for(mode),
            )
            .for_mutation_shape(shape)
            .under_maintenance_mode(mode),
        )
        .into_result()
        .expect("ordinary B-tree strategy must admit")
}

pub(super) fn lsm_strategy() -> LayoutStrategyRegistrySnapshot {
    let security = wal_security();
    let family = admit_family(DurableArtifactFamilyId::PublicationWalIntent, &security);
    let domain = admit_key_domain(family, &security);
    layout_admission_registry()
        .admit(LayoutAdmissionRequest::from_admitted(
            family,
            domain,
            LayoutStrategyFamily::BaselineLsmWriteOptimized,
            LayoutRequestedCapability::point_lookup(),
            ArtifactFamilyAccessLane::HotPath,
        ))
        .into_result()
        .expect("ordinary persisted LSM strategy must admit")
}

pub(super) fn page_security() -> forge_store_security::StoreAdmittedSecurityScope {
    security_scope(
        SecurityScopeFixtureAuthority::Current,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    )
}

pub(super) fn wal_security() -> forge_store_security::StoreAdmittedSecurityScope {
    security_scope(
        SecurityScopeFixtureAuthority::Current,
        StoreKeyScope::WalCheckpointEnvelope,
        StoreTenantScope::StoreInternal,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedWalRecord,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    )
}

pub(super) const fn lane_for(mode: IndexMaintenanceMode) -> ArtifactFamilyAccessLane {
    match mode {
        IndexMaintenanceMode::RebuildOnly | IndexMaintenanceMode::MigrationOnly => {
            ArtifactFamilyAccessLane::MaintenancePath
        }
        IndexMaintenanceMode::AdvisoryOnly | IndexMaintenanceMode::VerifierOnly => {
            ArtifactFamilyAccessLane::TerminalPath
        }
        _ => ArtifactFamilyAccessLane::HotPath,
    }
}
