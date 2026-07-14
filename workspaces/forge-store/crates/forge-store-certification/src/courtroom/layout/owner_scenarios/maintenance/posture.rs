use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_layout_indexes::declarations::ArtifactFamilyAccessLane;
use forge_store_layout_indexes::strategy_declarations::{
    layout_admission_registry, LayoutAdmissionRequest, LayoutRequestedCapability,
    LayoutStrategyFamily,
};
use forge_store_layout_indexes::{
    access_planning, live_maintenance_posture, IndexMaintenanceMode, LiveMaintenanceRequest,
    ObserveOwnerCase,
};
use forge_store_security::{
    StoreAuthenticityRequirement, StoreCustodyPosture, StoreKeyScope, StoreTenantScope,
};
use forge_store_test_support::{admitted_layout_bootstrap_catalog, SecurityScopeFixtureAuthority};

use super::super::fixture_admission::{admit_family, admit_key_domain, security_scope};
use super::strategy::{lane_for, page_security};
use super::LayoutOwnerObservationLedger;

pub(super) fn execute(ledger: &mut LayoutOwnerObservationLedger) {
    let security = page_security();
    let family = admit_family(DurableArtifactFamilyId::PhysicalPage, &security);
    let domain = admit_key_domain(family, &security);
    let catalog = admitted_layout_bootstrap_catalog();
    let materialization = access_planning()
        .admit_current_catalog_root_materialization(family, &catalog)
        .expect("page family must materialize");

    for mode in [
        IndexMaintenanceMode::SynchronousExact,
        IndexMaintenanceMode::AsynchronousLagged,
        IndexMaintenanceMode::RebuildOnly,
        IndexMaintenanceMode::LazyMaterializedOnDemand,
        IndexMaintenanceMode::AdvisoryOnly,
        IndexMaintenanceMode::VerifierOnly,
        IndexMaintenanceMode::MigrationOnly,
    ] {
        let strategy = layout_admission_registry()
            .admit(
                LayoutAdmissionRequest::from_admitted(
                    family,
                    domain,
                    LayoutStrategyFamily::BaselineBTreeRange,
                    LayoutRequestedCapability::point_lookup(),
                    lane_for(mode),
                )
                .under_maintenance_mode(mode),
            )
            .into_result()
            .expect("maintenance posture strategy must admit");
        let outcome = live_maintenance_posture()
            .classify(LiveMaintenanceRequest::new(&strategy, &materialization));
        ledger.record_live_maintenance_posture(outcome.owner_case_observation());
    }

    let root_security = security_scope(
        SecurityScopeFixtureAuthority::Current,
        StoreKeyScope::StoreManagedRoot,
        StoreTenantScope::StoreInternal,
        StoreAuthenticityRequirement::not_required(),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let root_family = admit_family(
        DurableArtifactFamilyId::PhysicalRootManifest,
        &root_security,
    );
    let root_materialization = access_planning()
        .admit_current_catalog_root_materialization(root_family, &catalog)
        .expect("root family must materialize");
    let strategy = layout_admission_registry()
        .admit(LayoutAdmissionRequest::from_admitted(
            family,
            domain,
            LayoutStrategyFamily::BaselineBTreeRange,
            LayoutRequestedCapability::point_lookup(),
            ArtifactFamilyAccessLane::HotPath,
        ))
        .into_result()
        .expect("ordinary strategy must admit");
    let outcome = live_maintenance_posture().classify(LiveMaintenanceRequest::new(
        &strategy,
        &root_materialization,
    ));
    ledger.record_live_maintenance_posture(outcome.owner_case_observation());
}
