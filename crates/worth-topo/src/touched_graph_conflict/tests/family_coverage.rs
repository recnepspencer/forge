use forge_relational::facade::identity::{EntityId, PartitionId};
use schema::facade::platform::authority::replay_undo_semantic_graph::{
    admit_replay_scope_identity, ReplayScopeIdentityInput, ReplayUndoSemanticGraphEquivalenceBasis,
    ReplayUndoSemanticGraphLocalityScope, ReplayUndoSemanticGraphTouchedSubject,
};
use schema::facade::platform::authority::replay_undo_semantic_graph_internal::admit_topology_derived_invalidation_prior_proof_identity;
use schema::facade::platform::authority::touched_graph_conflict::{
    admit_conflict_overlap_identity, admit_conflict_participant_identity, admit_conflict_routing_contract,
    ConflictAspectClass, ConflictOverlapIdentityInput, ConflictParticipantIdentityInput,
    ConflictPriorProofInput, ConflictRoutingPosture,
};
use schema::facade::platform::authority::WorthTopologyTouchedAspect;

use crate::derived_topology::invalidation_plan::selection::selection_test_fixtures::loop_cycles_touched_closure;

#[test]
fn topology_aspect_family_is_selected_through_real_contract_route() {
    let locality = loop_cycles_touched_closure("phase-3-aspect-family-coverage");
    let contract = admit_conflict_routing_contract(
        admit_conflict_overlap_identity(ConflictOverlapIdentityInput::aspect(
            ConflictAspectClass::WorthTopologyTouched(WorthTopologyTouchedAspect::GeometryBinding),
            locality
                .conflict_locality_identity()
                .expect("topology locality admits"),
            vec![admit_conflict_participant_identity(ConflictParticipantIdentityInput::entity(
                EntityId::new(PartitionId::main(), 91, 1),
            ))
            .expect("entity participant admits")],
        ))
        .expect("aspect overlap admits"),
        ConflictPriorProofInput::none(),
        ConflictRoutingPosture::RequiresFamilySelection,
    );

    let matches = locality.matching_aspect_or_locality_conflict_family_identities_for_contract(&contract);

    assert_eq!(
        matches,
        vec![crate::touched_graph_conflict::TopologyConflictFamilyIdentity::AspectSelection]
    );
    assert!(
        !matches.contains(&crate::touched_graph_conflict::TopologyConflictFamilyIdentity::ValidatorSelection)
    );
}

#[test]
fn topology_replay_family_is_selected_through_real_contract_route() {
    let locality = loop_cycles_touched_closure("phase-3-replay-family-coverage");
    let replay_scope = admit_replay_scope_identity(ReplayScopeIdentityInput::new(
        ReplayUndoSemanticGraphEquivalenceBasis::new(
            ReplayUndoSemanticGraphLocalityScope::TopologyTouchedClosure,
            vec![ReplayUndoSemanticGraphTouchedSubject::TopologyAspect {
                aspect: WorthTopologyTouchedAspect::GeometryBinding,
            }],
            admit_topology_derived_invalidation_prior_proof_identity("phase-3-replay-family-proof"),
            None,
        ),
    ));
    let contract = admit_conflict_routing_contract(
        admit_conflict_overlap_identity(ConflictOverlapIdentityInput::replay_undo(
            locality
                .conflict_locality_identity()
                .expect("topology locality admits"),
            vec![replay_scope.clone().into()],
        ))
        .expect("replay overlap admits"),
        ConflictPriorProofInput::from_identities(vec![replay_scope.into()]),
        ConflictRoutingPosture::RequiresFamilySelection,
    );

    let matches = locality.matching_conflict_family_identities_for_contract(&contract);

    assert_eq!(
        matches,
        vec![crate::touched_graph_conflict::TopologyConflictFamilyIdentity::ReplayBoundarySelection]
    );
    assert!(
        !matches.contains(&crate::touched_graph_conflict::TopologyConflictFamilyIdentity::AspectSelection)
    );
}
