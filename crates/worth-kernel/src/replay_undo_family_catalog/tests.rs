use topology::facade::{
    current_topology_replay_family_catalog, TopologyReplayFamilyIdentity,
    TopologyReplayFamilyWorkloadDependencyPosture,
};
use worth_spatial::facade::replay_family_catalog::{
    current_spatial_replay_family_catalog, SpatialReplayFamilyIdentity,
    SpatialReplayFamilyWorkloadDependencyPosture,
};

use super::consumer_binding::{
    replay_public_closeout_consumer_requirement, retained_replay_workload_consumer_requirement,
    transaction_boundary_undo_consumer_requirement,
};
use super::replay_catalog::{
    current_replay_family_catalog, ReplayFamilyDomain, ReplayFamilyIdentity,
    ReplayFamilyWorkloadDependencyPosture,
};
use super::undo_catalog::{current_undo_family_catalog, UndoFamilyDomain, UndoFamilyIdentity};

#[test]
fn one_replay_family_declaration_serves_domain_and_kernel_consumers() {
    let spatial_catalog = current_spatial_replay_family_catalog();
    let spatial_family = spatial_catalog
        .require_family(SpatialReplayFamilyIdentity::BooleanEventLedgerReplay)
        .expect("domain-local family");
    assert_eq!(
        spatial_family.workload_dependency_posture(),
        SpatialReplayFamilyWorkloadDependencyPosture::RequiresLookupConsumedWorkloadAndRetainedReplay
    );

    let replay_catalog = current_replay_family_catalog();
    let retained_requirement = retained_replay_workload_consumer_requirement();
    let closeout_requirement = replay_public_closeout_consumer_requirement();
    let kernel_family = replay_catalog
        .require_family_for_consumer(&retained_requirement)
        .expect("kernel family");
    let closeout_family = replay_catalog
        .require_family_for_consumer(&closeout_requirement)
        .expect("closeout family");
    assert_eq!(kernel_family.domain(), ReplayFamilyDomain::Spatial);
    assert_eq!(
        kernel_family.workload_dependency_posture(),
        ReplayFamilyWorkloadDependencyPosture::RequiresLookupConsumedWorkloadAndRetainedReplay
    );
    assert_eq!(
        closeout_family.identity(),
        ReplayFamilyIdentity::SpatialBooleanEventLedgerReplay
    );
    assert_eq!(replay_catalog.families_requiring_retained_replay().len(), 1);
}

#[test]
fn topology_replay_declaration_stays_declared_once_in_domain_and_kernel_catalogs() {
    let topology_catalog = current_topology_replay_family_catalog();
    let topology_family = topology_catalog
        .require_family(TopologyReplayFamilyIdentity::TraversalViewsReplay)
        .expect("topology family");
    assert_eq!(
        topology_family.workload_dependency_posture(),
        TopologyReplayFamilyWorkloadDependencyPosture::TopologyOnly
    );

    let replay_catalog = current_replay_family_catalog();
    let kernel_family = replay_catalog
        .require_family(ReplayFamilyIdentity::TopologyTraversalViewsReplay)
        .expect("kernel family");
    assert_eq!(kernel_family.domain(), ReplayFamilyDomain::Topology);
    assert_eq!(
        kernel_family.workload_dependency_posture(),
        ReplayFamilyWorkloadDependencyPosture::TopologyOnly
    );
}

#[test]
fn replay_catalog_exposes_topology_and_spatial_domains_without_consumer_edits() {
    let replay_catalog = current_replay_family_catalog();
    assert_eq!(replay_catalog.counters().topology_family_count(), 2);
    assert_eq!(replay_catalog.counters().spatial_family_count(), 2);
    assert_eq!(
        replay_catalog
            .families_for_domain(ReplayFamilyDomain::Topology)
            .len(),
        2
    );
    assert_eq!(
        replay_catalog
            .families_for_domain(ReplayFamilyDomain::Spatial)
            .len(),
        2
    );
}

#[test]
fn undo_catalog_exposes_topology_and_spatial_domains() {
    let undo_catalog = current_undo_family_catalog();
    assert_eq!(undo_catalog.counters().topology_family_count(), 2);
    assert_eq!(undo_catalog.counters().spatial_family_count(), 2);
    assert_eq!(
        undo_catalog
            .families_for_domain(UndoFamilyDomain::Topology)
            .len(),
        2
    );
    assert_eq!(
        undo_catalog
            .families_for_domain(UndoFamilyDomain::Spatial)
            .len(),
        2
    );
    let requirement = transaction_boundary_undo_consumer_requirement();
    assert_eq!(
        undo_catalog
            .require_family_for_consumer(&requirement)
            .expect("undo consumer family")
            .identity(),
        UndoFamilyIdentity::SpatialBooleanEventLedgerRollback
    );
}

#[test]
fn replay_identity_is_typed_after_declaration() {
    let topology_catalog = current_topology_replay_family_catalog();
    let topology_family = topology_catalog
        .require_family(TopologyReplayFamilyIdentity::TraversalViewsReplay)
        .expect("typed topology family");
    let replay_catalog = current_replay_family_catalog();
    let replay_family = replay_catalog
        .require_family(ReplayFamilyIdentity::TopologyTraversalViewsReplay)
        .expect("typed replay family");

    assert_eq!(
        topology_family.identity(),
        TopologyReplayFamilyIdentity::TraversalViewsReplay
    );
    assert_eq!(
        replay_family.identity(),
        ReplayFamilyIdentity::TopologyTraversalViewsReplay
    );
}
