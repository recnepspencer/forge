use forge_relational::facade::identity::{EntityId, PartitionId};
use schema::facade::platform::authority::replay_undo_semantic_graph::{
    admit_replay_scope_identity, ReplayScopeIdentityInput, ReplayUndoSemanticGraphEquivalenceBasis,
    ReplayUndoSemanticGraphLocalityScope, ReplayUndoSemanticGraphTouchedSubject,
    ReplayUndoTransactionScopeClaim, ReplayUndoTransactionScopeKind,
};
use schema::facade::platform::authority::replay_undo_semantic_graph_internal::admit_topology_derived_invalidation_prior_proof_identity;
use schema::facade::platform::authority::touched_graph_conflict::{
    admit_conflict_overlap_identity, admit_conflict_participant_identity, ConflictAspectClass,
    ConflictOverlapCategory, ConflictOverlapIdentityInput, ConflictParticipantIdentityInput,
    ConflictTransactionProofInput,
};
use schema::facade::platform::authority::WorthTopologyTouchedAspect;

use super::selection_test_fixtures::loop_cycles_touched_closure;
use crate::validator_invariant_catalog::current_worth_topology_legality_catalog_closeout;

#[test]
fn topology_validator_identity_lowers_into_shared_validator_overlap() {
    let closeout = current_worth_topology_legality_catalog_closeout()
        .expect("legality catalog closeout should build");
    let identity = closeout
        .catalog()
        .records()
        .first()
        .expect("catalog exposes at least one legality family")
        .identity();
    let locality = loop_cycles_touched_closure("validator-overlap")
        .conflict_locality_identity()
        .expect("topology locality admits");
    let participant = identity
        .conflict_participant_identity()
        .expect("validator participant admits");

    let overlap = admit_conflict_overlap_identity(ConflictOverlapIdentityInput::validator(
        locality,
        vec![participant],
    ))
    .expect("validator overlap admits");

    assert_eq!(overlap.category(), ConflictOverlapCategory::Validator);
}

#[test]
fn topology_locality_distinguishes_aspect_overlap() {
    let locality = loop_cycles_touched_closure("aspect-overlap")
        .conflict_locality_identity()
        .expect("topology locality admits");
    let participant = admit_conflict_participant_identity(
        ConflictParticipantIdentityInput::entity(EntityId::new(PartitionId::main(), 41, 1)),
    )
    .expect("entity participant admits");

    let locality_overlap =
        admit_conflict_overlap_identity(ConflictOverlapIdentityInput::locality(locality.clone()))
            .expect("locality overlap admits");
    let aspect_overlap = admit_conflict_overlap_identity(ConflictOverlapIdentityInput::aspect(
        ConflictAspectClass::WorthTopologyTouched(WorthTopologyTouchedAspect::GeometryBinding),
        locality,
        vec![participant],
    ))
    .expect("aspect overlap admits");

    assert_eq!(aspect_overlap.category(), ConflictOverlapCategory::Aspect);
    assert_ne!(
        locality_overlap.overlap_identity_digest(),
        aspect_overlap.overlap_identity_digest()
    );
}

#[test]
fn topology_aspect_and_entity_overlap_remain_distinct() {
    let locality = loop_cycles_touched_closure("entity-aspect-overlap")
        .conflict_locality_identity()
        .expect("topology locality admits");
    let participant = admit_conflict_participant_identity(
        ConflictParticipantIdentityInput::entity(EntityId::new(PartitionId::main(), 49, 1)),
    )
    .expect("entity participant admits");

    let entity_overlap =
        admit_conflict_overlap_identity(ConflictOverlapIdentityInput::entity(vec![
            participant.clone()
        ]))
        .expect("entity overlap admits");
    let aspect_overlap = admit_conflict_overlap_identity(ConflictOverlapIdentityInput::aspect(
        ConflictAspectClass::WorthTopologyTouched(WorthTopologyTouchedAspect::GeometryBinding),
        locality,
        vec![participant],
    ))
    .expect("aspect overlap admits");

    assert_ne!(
        entity_overlap.overlap_identity_digest(),
        aspect_overlap.overlap_identity_digest()
    );
}

#[test]
fn topology_aspect_overlap_identity_is_stable_under_rerun() {
    let locality = loop_cycles_touched_closure("aspect-stability")
        .conflict_locality_identity()
        .expect("topology locality admits");
    let first_participant = admit_conflict_participant_identity(
        ConflictParticipantIdentityInput::entity(EntityId::new(PartitionId::main(), 51, 1)),
    )
    .expect("first participant admits");
    let second_participant = admit_conflict_participant_identity(
        ConflictParticipantIdentityInput::entity(EntityId::new(PartitionId::main(), 52, 1)),
    )
    .expect("second participant admits");

    let first = admit_conflict_overlap_identity(ConflictOverlapIdentityInput::aspect(
        ConflictAspectClass::WorthTopologyTouched(WorthTopologyTouchedAspect::GeometryBinding),
        locality.clone(),
        vec![first_participant.clone(), second_participant.clone()],
    ))
    .expect("first aspect overlap admits");
    let second = admit_conflict_overlap_identity(ConflictOverlapIdentityInput::aspect(
        ConflictAspectClass::WorthTopologyTouched(WorthTopologyTouchedAspect::GeometryBinding),
        locality,
        vec![second_participant, first_participant],
    ))
    .expect("second aspect overlap admits");

    assert_eq!(
        first.overlap_identity_digest(),
        second.overlap_identity_digest()
    );
}

#[test]
fn topology_locality_distinguishes_replay_and_transaction_overlap() {
    let locality = loop_cycles_touched_closure("replay-transaction-overlap")
        .conflict_locality_identity()
        .expect("topology locality admits");
    let replay_scope = admit_replay_scope_identity(ReplayScopeIdentityInput::new(
        ReplayUndoSemanticGraphEquivalenceBasis::new(
            ReplayUndoSemanticGraphLocalityScope::TopologyTouchedClosure,
            vec![ReplayUndoSemanticGraphTouchedSubject::TopologyAspect {
                aspect: WorthTopologyTouchedAspect::GeometryBinding,
            }],
            admit_topology_derived_invalidation_prior_proof_identity("phase-2-replay-proof"),
            None,
        ),
    ));

    let replay_overlap = admit_conflict_overlap_identity(
        ConflictOverlapIdentityInput::replay_undo(locality.clone(), vec![replay_scope.into()]),
    )
    .expect("replay overlap admits");
    let transaction_overlap =
        admit_conflict_overlap_identity(ConflictOverlapIdentityInput::transaction(
            locality,
            ConflictTransactionProofInput::new(ReplayUndoTransactionScopeClaim::new(
                ReplayUndoTransactionScopeKind::Replay,
                "phase-2-transaction-scope",
            )),
        ))
        .expect("transaction overlap admits");

    assert_eq!(
        replay_overlap.category(),
        ConflictOverlapCategory::ReplayUndo
    );
    assert_eq!(
        transaction_overlap.category(),
        ConflictOverlapCategory::Transaction
    );
    assert_ne!(
        replay_overlap.overlap_identity_digest(),
        transaction_overlap.overlap_identity_digest()
    );
}

#[test]
fn topology_shared_overlap_categories_remain_distinct_outside_evidence_lane() {
    let closeout = current_worth_topology_legality_catalog_closeout()
        .expect("legality catalog closeout should build");
    let validator = closeout
        .catalog()
        .records()
        .first()
        .expect("catalog exposes at least one legality family")
        .identity()
        .conflict_participant_identity()
        .expect("validator participant admits");
    let locality = loop_cycles_touched_closure("category-lattice")
        .conflict_locality_identity()
        .expect("topology locality admits");
    let replay_scope = admit_replay_scope_identity(ReplayScopeIdentityInput::new(
        ReplayUndoSemanticGraphEquivalenceBasis::new(
            ReplayUndoSemanticGraphLocalityScope::TopologyTouchedClosure,
            vec![ReplayUndoSemanticGraphTouchedSubject::TopologyAspect {
                aspect: WorthTopologyTouchedAspect::GeometryBinding,
            }],
            admit_topology_derived_invalidation_prior_proof_identity("phase-2-lattice-proof"),
            None,
        ),
    ));

    let overlaps = [
        admit_conflict_overlap_identity(ConflictOverlapIdentityInput::entity(vec![
            admit_conflict_participant_identity(ConflictParticipantIdentityInput::entity(
                EntityId::new(PartitionId::main(), 61, 1),
            ))
            .expect("entity participant admits"),
        ]))
        .expect("entity overlap"),
        admit_conflict_overlap_identity(ConflictOverlapIdentityInput::relation(vec![
            admit_conflict_participant_identity(ConflictParticipantIdentityInput::relation(
                forge_relational::facade::identity::RelationId::new(PartitionId::main(), 61, 1),
            ))
            .expect("relation participant admits"),
        ]))
        .expect("relation overlap"),
        admit_conflict_overlap_identity(ConflictOverlapIdentityInput::aspect(
            ConflictAspectClass::WorthTopologyTouched(WorthTopologyTouchedAspect::GeometryBinding),
            locality.clone(),
            vec![
                admit_conflict_participant_identity(ConflictParticipantIdentityInput::entity(
                    EntityId::new(PartitionId::main(), 62, 1),
                ))
                .expect("aspect participant admits"),
            ],
        ))
        .expect("aspect overlap"),
        admit_conflict_overlap_identity(ConflictOverlapIdentityInput::locality(locality.clone()))
            .expect("locality overlap"),
        admit_conflict_overlap_identity(ConflictOverlapIdentityInput::validator(
            locality.clone(),
            vec![validator],
        ))
        .expect("validator overlap"),
        admit_conflict_overlap_identity(ConflictOverlapIdentityInput::replay_undo(
            locality.clone(),
            vec![replay_scope.into()],
        ))
        .expect("replay overlap"),
        admit_conflict_overlap_identity(ConflictOverlapIdentityInput::transaction(
            locality,
            ConflictTransactionProofInput::new(ReplayUndoTransactionScopeClaim::new(
                ReplayUndoTransactionScopeKind::Replay,
                "phase-2-lattice-transaction",
            )),
        ))
        .expect("transaction overlap"),
    ];

    let mut digests = overlaps
        .iter()
        .map(|overlap| overlap.overlap_identity_digest().to_string())
        .collect::<Vec<_>>();
    digests.sort();
    digests.dedup();

    assert_eq!(digests.len(), 7);
}

#[test]
fn topology_shared_overlap_rejects_wrong_participant_authority() {
    let locality = loop_cycles_touched_closure("wrong-authority")
        .conflict_locality_identity()
        .expect("topology locality admits");
    let entity = admit_conflict_participant_identity(ConflictParticipantIdentityInput::entity(
        EntityId::new(PartitionId::main(), 71, 1),
    ))
    .expect("entity participant admits");
    let relation = admit_conflict_participant_identity(ConflictParticipantIdentityInput::relation(
        forge_relational::facade::identity::RelationId::new(PartitionId::main(), 71, 1),
    ))
    .expect("relation participant admits");

    assert!(matches!(
        admit_conflict_overlap_identity(ConflictOverlapIdentityInput::entity(vec![relation])),
        Err(schema::facade::platform::authority::touched_graph_conflict::ConflictRoutingVocabularyError::WrongParticipantAuthority { .. })
    ));
    assert!(matches!(
        admit_conflict_overlap_identity(ConflictOverlapIdentityInput::relation(vec![entity.clone()])),
        Err(schema::facade::platform::authority::touched_graph_conflict::ConflictRoutingVocabularyError::WrongParticipantAuthority { .. })
    ));
    assert!(matches!(
        admit_conflict_overlap_identity(ConflictOverlapIdentityInput::validator(
            locality,
            vec![entity]
        )),
        Err(schema::facade::platform::authority::touched_graph_conflict::ConflictRoutingVocabularyError::WrongParticipantAuthority { .. })
    ));
}
