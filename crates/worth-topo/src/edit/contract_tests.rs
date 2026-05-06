use forge_relational::facade::identity::{EntityId, PartitionId};
use worth_schema::facade::{
    WorthAspect, WorthDiagnosticsAspect, WorthNamingAspect, WorthTopologyAspect,
    WorthTopologyEntityKind,
};

use super::{
    WorthBoundaryMembershipKind, WorthTopologyDerivedRegion, WorthTopologyEditChangedScope,
    WorthTopologyEditContract, WorthTopologyEditFamily, WorthTopologyEditNamingOutcome,
    WorthTopologyEditNamingScope,
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

#[test]
fn edit_batch_digest_is_deterministic_for_same_contracts() {
    let contracts = vec![
        WorthTopologyEditContract::create_topology_entity(
            "m3.digest.vertex",
            WorthTopologyEntityKind::Vertex,
        ),
        WorthTopologyEditContract::attach_boundary_membership(
            "m3.digest.loop",
            WorthBoundaryMembershipKind::LoopOwnsHalfEdge,
            EntityId::new(PartitionId::main(), 1, 1),
            EntityId::new(PartitionId::main(), 2, 1),
        ),
    ];

    let left = super::WorthTopologyEditBatch::new(contracts.clone()).expect("non-empty edit batch");
    let right = super::WorthTopologyEditBatch::new(contracts).expect("non-empty edit batch");

    let left_digest = left.topology_edit_digest();
    let right_digest = right.topology_edit_digest();
    assert_eq!(left_digest, right_digest);
    assert_eq!(left_digest.contract_count, 2);
    assert_eq!(left_digest.family_count, 2);
    assert_eq!(left_digest.changed_scope_count, 5);
    assert_eq!(left_digest.naming_scope_count, 2);
    assert_eq!(left_digest.derived_region_count, 5);
}

#[test]
fn edit_batch_continuity_matrix_counts_naming_outcomes() {
    let batch = super::WorthTopologyEditBatch::new(vec![
        WorthTopologyEditContract::create_topology_entity(
            "m3.naming.vertex",
            WorthTopologyEntityKind::Vertex,
        ),
        WorthTopologyEditContract::attach_boundary_membership(
            "m3.naming.loop",
            WorthBoundaryMembershipKind::LoopOwnsHalfEdge,
            EntityId::new(PartitionId::main(), 1, 1),
            EntityId::new(PartitionId::main(), 2, 1),
        ),
    ])
    .expect("non-empty edit batch");

    let matrix = batch.naming_edit_continuity_matrix();

    assert_eq!(matrix.rows.len(), 2);
    assert_eq!(matrix.preserved_count, 1);
    assert_eq!(matrix.ambiguous_count, 1);
    assert_eq!(matrix.rejected_count, 0);
    assert_eq!(
        matrix.rows[0].outcome,
        WorthTopologyEditNamingOutcome::Preserved
    );
    assert_eq!(
        matrix.rows[1].outcome,
        WorthTopologyEditNamingOutcome::Ambiguous
    );
}

#[test]
fn continuity_matrix_exposes_overall_outcome_class() {
    let ambiguous = super::WorthTopologyEditBatch::new(vec![
        WorthTopologyEditContract::attach_boundary_membership(
            "m3.naming.ambiguous.loop",
            WorthBoundaryMembershipKind::LoopOwnsHalfEdge,
            EntityId::new(PartitionId::main(), 1, 1),
            EntityId::new(PartitionId::main(), 2, 1),
        ),
    ])
    .expect("non-empty ambiguous batch")
    .naming_edit_continuity_matrix();
    assert_eq!(
        ambiguous.outcome_class(),
        WorthTopologyEditNamingOutcome::Ambiguous
    );
    assert_eq!(
        ambiguous.rejection_class(),
        Some(super::WorthTopologyEditRejectionClass::NamingContinuityAmbiguous)
    );

    let rejected = super::WorthTopologyEditBatch::new(vec![
        WorthTopologyEditContract::retire_topology_entity(
            EntityId::new(PartitionId::main(), 3, 1),
            WorthTopologyEntityKind::Loop,
        ),
    ])
    .expect("non-empty rejected batch")
    .naming_edit_continuity_matrix();
    assert_eq!(
        rejected.outcome_class(),
        WorthTopologyEditNamingOutcome::Rejected
    );
    assert_eq!(
        rejected.rejection_class(),
        Some(super::WorthTopologyEditRejectionClass::NamingContinuityRejected)
    );
}
