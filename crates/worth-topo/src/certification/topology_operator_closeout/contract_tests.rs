use forge_relational::facade::identity::{EntityId, PartitionId};
use schema::facade::{Aspect, DiagnosticsAspect, NamingAspect, TopologyAspect, TopologyEntityKind};

use crate::topology_operators::{
    BoundaryMembershipKind, TopologyDerivedRegion, TopologyEditBatch, TopologyEditChangedScope,
    TopologyEditContract, TopologyEditDerivedFallbackPolicy, TopologyEditFamily,
    TopologyEditNamingOutcome, TopologyEditNamingScope, TopologyEditRejectionClass,
    TopologyOperatorExecutionError,
};

#[test]
fn create_topology_entity_contract_is_topology_only_and_naming_aware() {
    let contract = TopologyEditContract::create_topology_entity(
        "m3.contract.vertex",
        TopologyEntityKind::Vertex,
    );

    assert_eq!(contract.family, TopologyEditFamily::CreateTopologyEntity);
    assert!(contract
        .touched_aspects()
        .contains(&Aspect::Topology(TopologyAspect::Structure)));
    assert!(contract
        .touched_aspects()
        .contains(&Aspect::Naming(NamingAspect::PersistentName)));
    assert!(contract
        .touched_aspects()
        .contains(&Aspect::Diagnostics(DiagnosticsAspect::Decisions)));
    assert_eq!(
        contract.changed_scopes(),
        &[
            TopologyEditChangedScope::Entity,
            TopologyEditChangedScope::Naming,
        ]
    );
    assert_eq!(
        contract.naming_scopes(),
        &[TopologyEditNamingScope::EditedEntityNames]
    );
    assert_eq!(
        contract.derived_regions(),
        &[
            TopologyDerivedRegion::EditLocalNeighborhoodRegion,
            TopologyDerivedRegion::NamingContinuityRegion,
        ]
    );
    assert_eq!(
        contract.derived_fallback_policy(),
        TopologyEditDerivedFallbackPolicy::AllowExplicitFallback
    );
}

#[test]
fn boundary_membership_contract_exposes_boundary_scope_and_regions() {
    let contract = TopologyEditContract::attach_boundary_membership(
        "m3.boundary.loop",
        BoundaryMembershipKind::LoopOwnsHalfEdge,
        EntityId::new(PartitionId::main(), 1, 1),
        EntityId::new(PartitionId::main(), 2, 1),
    );

    assert_eq!(
        contract.family,
        TopologyEditFamily::AttachBoundaryMembership
    );
    assert!(contract
        .touched_aspects()
        .contains(&Aspect::Topology(TopologyAspect::Boundary)));
    assert!(contract
        .changed_scopes()
        .contains(&TopologyEditChangedScope::Loop));
    assert!(contract
        .derived_regions()
        .contains(&TopologyDerivedRegion::LoopRegion));
}

#[test]
fn edit_batch_digest_is_deterministic_for_same_contracts() {
    let contracts = vec![
        TopologyEditContract::create_topology_entity(
            "m3.digest.vertex",
            TopologyEntityKind::Vertex,
        ),
        TopologyEditContract::attach_boundary_membership(
            "m3.digest.loop",
            BoundaryMembershipKind::LoopOwnsHalfEdge,
            EntityId::new(PartitionId::main(), 1, 1),
            EntityId::new(PartitionId::main(), 2, 1),
        ),
    ];

    let left = TopologyEditBatch::new(contracts.clone()).expect("non-empty edit batch");
    let right = TopologyEditBatch::new(contracts).expect("non-empty edit batch");

    let left_digest = left.topology_edit_digest();
    let right_digest = right.topology_edit_digest();
    assert_eq!(left_digest, right_digest);
    assert_eq!(left_digest.contract_count, 2);
    assert_eq!(left_digest.family_count, 2);
    assert_eq!(left_digest.changed_scope_count, 5);
    assert_eq!(left_digest.naming_scope_count, 2);
    assert_eq!(left_digest.derived_region_count, 5);
    assert_eq!(left_digest.fallback_policy_count, 2);
    assert_eq!(left_digest.fallback_rejection_policy_count, 0);
}

#[test]
fn edit_batch_digest_tracks_locality_only_fallback_policy() {
    let contract = TopologyEditContract::attach_boundary_membership(
        "m3.digest.local_only.loop",
        BoundaryMembershipKind::LoopOwnsHalfEdge,
        EntityId::new(PartitionId::main(), 1, 1),
        EntityId::new(PartitionId::main(), 2, 1),
    )
    .with_derived_fallback_policy(TopologyEditDerivedFallbackPolicy::RejectAnyFallback);
    let batch = TopologyEditBatch::new(vec![contract]).expect("non-empty edit batch");
    let digest = batch.topology_edit_digest();

    assert_eq!(digest.fallback_policy_count, 1);
    assert_eq!(digest.fallback_rejection_policy_count, 1);
}

#[test]
fn edit_batch_continuity_matrix_counts_naming_outcomes() {
    let batch = TopologyEditBatch::new(vec![
        TopologyEditContract::create_topology_entity(
            "m3.naming.vertex",
            TopologyEntityKind::Vertex,
        ),
        TopologyEditContract::attach_boundary_membership(
            "m3.naming.loop",
            BoundaryMembershipKind::LoopOwnsHalfEdge,
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
    assert_eq!(matrix.rows[0].outcome, TopologyEditNamingOutcome::Preserved);
    assert_eq!(matrix.rows[1].outcome, TopologyEditNamingOutcome::Ambiguous);
}

#[test]
fn continuity_matrix_exposes_overall_outcome_class() {
    let ambiguous = TopologyEditBatch::new(vec![TopologyEditContract::attach_boundary_membership(
        "m3.naming.ambiguous.loop",
        BoundaryMembershipKind::LoopOwnsHalfEdge,
        EntityId::new(PartitionId::main(), 1, 1),
        EntityId::new(PartitionId::main(), 2, 1),
    )])
    .expect("non-empty ambiguous batch")
    .naming_edit_continuity_matrix();
    assert_eq!(
        ambiguous.outcome_class(),
        TopologyEditNamingOutcome::Ambiguous
    );
    assert_eq!(
        ambiguous.rejection_class(),
        Some(TopologyEditRejectionClass::NamingContinuityAmbiguous)
    );

    let rejected = TopologyEditBatch::new(vec![TopologyEditContract::retire_topology_entity(
        EntityId::new(PartitionId::main(), 3, 1),
        TopologyEntityKind::Loop,
    )])
    .expect("non-empty rejected batch")
    .naming_edit_continuity_matrix();
    assert_eq!(
        rejected.outcome_class(),
        TopologyEditNamingOutcome::Rejected
    );
    assert_eq!(
        rejected.rejection_class(),
        Some(TopologyEditRejectionClass::NamingContinuityRejected)
    );
}

#[test]
fn topology_edit_rejection_taxonomy_matches_milestone_three_spec() {
    assert_eq!(
        TopologyEditRejectionClass::ALL,
        [
            TopologyEditRejectionClass::OutOfClassEdit,
            TopologyEditRejectionClass::InvariantBlocked,
            TopologyEditRejectionClass::NamingContinuityAmbiguous,
            TopologyEditRejectionClass::NamingContinuityRejected,
            TopologyEditRejectionClass::ScopeLocalizationUnavailable,
            TopologyEditRejectionClass::DerivedFallbackExceeded,
        ]
    );
    assert_eq!(
        TopologyEditRejectionClass::ScopeLocalizationUnavailable.as_str(),
        "ScopeLocalizationUnavailable"
    );
    assert_eq!(
        TopologyEditRejectionClass::DerivedFallbackExceeded.as_str(),
        "DerivedFallbackExceeded"
    );
}

#[test]
fn missing_authoritative_scope_reports_scope_localization_unavailable() {
    let missing_entity = EntityId::new(PartitionId::main(), 99, 1);
    let error = TopologyOperatorExecutionError::MissingExistingEntityBinding(missing_entity);

    assert_eq!(
        error.rejection_class(),
        Some(TopologyEditRejectionClass::ScopeLocalizationUnavailable),
        "a missing live authority binding means the edit runner cannot localize the requested scope, not that a specific invariant was proven false"
    );
}
