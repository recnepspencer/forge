use forge_relational::facade::identity::{EntityId, PartitionId};
use worth_schema::facade::{
    WorthAspect, WorthDiagnosticsAspect, WorthNamingAspect, WorthTopologyAspect,
    WorthTopologyEntityKind,
};

use super::{
    WorthBoundaryMembershipKind, WorthTopologyDerivedRegion, WorthTopologyEditChangedScope,
    WorthTopologyEditContract, WorthTopologyEditFamily, WorthTopologyEditNamingScope,
};

#[test]
fn create_topology_entity_contract_is_topology_only_and_naming_aware() {
    let contract = WorthTopologyEditContract::create_topology_entity(
        "m3.contract.vertex",
        WorthTopologyEntityKind::Vertex,
    );

    assert_eq!(
        contract.family,
        WorthTopologyEditFamily::CreateTopologyEntity
    );
    assert!(contract
        .touched_aspects()
        .contains(&WorthAspect::Topology(WorthTopologyAspect::Structure)));
    assert!(contract
        .touched_aspects()
        .contains(&WorthAspect::Naming(WorthNamingAspect::PersistentName)));
    assert!(contract
        .touched_aspects()
        .contains(&WorthAspect::Diagnostics(WorthDiagnosticsAspect::Decisions)));
    assert_eq!(
        contract.changed_scopes(),
        &[
            WorthTopologyEditChangedScope::Entity,
            WorthTopologyEditChangedScope::Naming,
        ]
    );
    assert_eq!(
        contract.naming_scopes(),
        &[WorthTopologyEditNamingScope::EditedEntityNames]
    );
    assert_eq!(
        contract.derived_regions(),
        &[
            WorthTopologyDerivedRegion::EditLocalNeighborhoodRegion,
            WorthTopologyDerivedRegion::NamingContinuityRegion,
        ]
    );
}

#[test]
fn boundary_membership_contract_exposes_boundary_scope_and_regions() {
    let contract = WorthTopologyEditContract::attach_boundary_membership(
        "m3.boundary.loop",
        WorthBoundaryMembershipKind::LoopOwnsHalfEdge,
        EntityId::new(PartitionId::main(), 1, 1),
        EntityId::new(PartitionId::main(), 2, 1),
    );

    assert_eq!(
        contract.family,
        WorthTopologyEditFamily::AttachBoundaryMembership
    );
    assert!(contract
        .touched_aspects()
        .contains(&WorthAspect::Topology(WorthTopologyAspect::Boundary)));
    assert!(contract
        .changed_scopes()
        .contains(&WorthTopologyEditChangedScope::Loop));
    assert!(contract
        .derived_regions()
        .contains(&WorthTopologyDerivedRegion::LoopRegion));
}
