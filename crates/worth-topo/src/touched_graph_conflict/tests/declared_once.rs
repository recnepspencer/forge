use schema::facade::platform::authority::replay_undo_semantic_graph::{
    admit_replay_scope_identity, ReplayScopeIdentityInput, ReplayUndoSemanticGraphEquivalenceBasis,
    ReplayUndoSemanticGraphLocalityScope, ReplayUndoSemanticGraphTouchedSubject,
};
use schema::facade::platform::authority::replay_undo_semantic_graph_internal::admit_topology_derived_invalidation_prior_proof_identity;
use schema::facade::platform::authority::touched_graph_conflict::{
    admit_conflict_overlap_identity, admit_conflict_routing_contract, ConflictAspectClass,
    ConflictOverlapIdentityInput, ConflictParticipantIdentityInput, ConflictPriorProofInput,
    ConflictRoutingPosture,
};
use schema::facade::platform::authority::WorthTopologyTouchedAspect;

use crate::derived_topology::invalidation_plan::selection::selection_test_fixtures::loop_cycles_touched_closure;
use crate::validator_invariant_catalog::current_worth_topology_legality_catalog_closeout;
use forge_relational::facade::identity::{EntityId, PartitionId};

#[test]
fn one_validator_declaration_serves_multiple_consumers_without_local_wiring() {
    let locality = loop_cycles_touched_closure("phase-3-validator-declared-once");
    let closeout = current_worth_topology_legality_catalog_closeout()
        .expect("legality catalog closeout builds");
    let validator_identity = closeout.catalog().records()[0].identity();
    let invariant_identity = closeout.catalog().records()[1].identity();

    let validator_matches = validator_identity
        .matching_conflict_family_identities(&locality)
        .expect("validator identity matches family");
    let invariant_matches = invariant_identity
        .matching_conflict_family_identities(&locality)
        .expect("invariant identity matches family");

    assert_eq!(
        validator_matches,
        vec![crate::touched_graph_conflict::TopologyConflictFamilyIdentity::ValidatorSelection]
    );
    assert_eq!(validator_matches, invariant_matches);

    let replay_scope = admit_replay_scope_identity(ReplayScopeIdentityInput::new(
        ReplayUndoSemanticGraphEquivalenceBasis::new(
            ReplayUndoSemanticGraphLocalityScope::TopologyTouchedClosure,
            vec![ReplayUndoSemanticGraphTouchedSubject::TopologyAspect {
                aspect: WorthTopologyTouchedAspect::GeometryBinding,
            }],
            admit_topology_derived_invalidation_prior_proof_identity(
                "phase-3-validator-declared-once-replay",
            ),
            None,
        ),
    ));
    let replay_contract = admit_conflict_routing_contract(
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

    assert!(validator_identity
        .matching_conflict_family_identities_for_contract(&locality, &replay_contract)
        .is_empty());
    assert!(invariant_identity
        .matching_conflict_family_identities_for_contract(&locality, &replay_contract)
        .is_empty());
}

#[test]
fn aspect_and_validator_consumers_do_not_collapse_into_one_route() {
    let locality = loop_cycles_touched_closure("phase-3-aspect-validator-contrast");
    let closeout = current_worth_topology_legality_catalog_closeout()
        .expect("legality catalog closeout builds");
    let validator_identity = closeout.catalog().records()[0].identity();
    let aspect_contract = admit_conflict_routing_contract(
        admit_conflict_overlap_identity(ConflictOverlapIdentityInput::aspect(
            ConflictAspectClass::WorthTopologyTouched(WorthTopologyTouchedAspect::GeometryBinding),
            locality
                .conflict_locality_identity()
                .expect("topology locality admits"),
            vec![
                schema::facade::platform::authority::touched_graph_conflict::admit_conflict_participant_identity(
                    ConflictParticipantIdentityInput::entity(EntityId::new(PartitionId::main(), 81, 1)),
                )
                .expect("entity participant admits"),
            ],
        ))
        .expect("aspect overlap admits"),
        ConflictPriorProofInput::none(),
        ConflictRoutingPosture::RequiresFamilySelection,
    );

    assert_eq!(
        locality.matching_aspect_or_locality_conflict_family_identities_for_contract(
            &aspect_contract,
        ),
        vec![crate::touched_graph_conflict::TopologyConflictFamilyIdentity::AspectSelection]
    );
    assert!(validator_identity
        .matching_conflict_family_identities_for_contract(&locality, &aspect_contract)
        .is_empty());
}
