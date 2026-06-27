use schema::facade::platform::authority::replay_undo_semantic_graph::{
    admit_replay_scope_identity, admit_undo_scope_identity, ReplayScopeIdentityInput,
    ReplayUndoSemanticGraphEquivalenceBasis, ReplayUndoSemanticGraphLocalityScope,
    ReplayUndoSemanticGraphTouchedSubject, UndoScopeIdentityInput,
};
use schema::facade::platform::authority::replay_undo_semantic_graph_internal::{
    admit_replay_undo_stage_index_identity, admit_spatial_evidence_lookup_prior_proof_identity,
    admit_topology_derived_invalidation_prior_proof_identity,
};
use schema::facade::platform::authority::{WorthTopologyTouchedAspect, WorthTopologyTouchedScope};

#[test]
fn replay_identity_is_stable_under_benign_subject_ordering_noise() {
    let canonical = admit_replay_scope_identity(ReplayScopeIdentityInput::new(
        ReplayUndoSemanticGraphEquivalenceBasis::new(
            ReplayUndoSemanticGraphLocalityScope::TopologyTouchedClosure,
            vec![
                ReplayUndoSemanticGraphTouchedSubject::TopologyEntity {
                    entity_identity: "1:2:3".to_string(),
                },
                ReplayUndoSemanticGraphTouchedSubject::TopologyAspect {
                    aspect: WorthTopologyTouchedAspect::TopologyBoundary,
                },
                ReplayUndoSemanticGraphTouchedSubject::TopologyScope {
                    scope: WorthTopologyTouchedScope::Loop,
                },
            ],
            admit_topology_derived_invalidation_prior_proof_identity("topo-receipt"),
            None,
        ),
    ));
    let reordered = admit_replay_scope_identity(ReplayScopeIdentityInput::new(
        ReplayUndoSemanticGraphEquivalenceBasis::new(
            ReplayUndoSemanticGraphLocalityScope::TopologyTouchedClosure,
            vec![
                ReplayUndoSemanticGraphTouchedSubject::TopologyScope {
                    scope: WorthTopologyTouchedScope::Loop,
                },
                ReplayUndoSemanticGraphTouchedSubject::TopologyAspect {
                    aspect: WorthTopologyTouchedAspect::TopologyBoundary,
                },
                ReplayUndoSemanticGraphTouchedSubject::TopologyEntity {
                    entity_identity: "1:2:3".to_string(),
                },
            ],
            admit_topology_derived_invalidation_prior_proof_identity("topo-receipt"),
            None,
        ),
    ));

    assert_eq!(canonical.digest(), reordered.digest());
}

#[test]
fn replay_identity_drifts_when_prior_proof_class_changes() {
    let topology = admit_replay_scope_identity(ReplayScopeIdentityInput::new(
        ReplayUndoSemanticGraphEquivalenceBasis::new(
            ReplayUndoSemanticGraphLocalityScope::TopologyTouchedClosure,
            vec![ReplayUndoSemanticGraphTouchedSubject::TopologyAspect {
                aspect: WorthTopologyTouchedAspect::TopologyStructure,
            }],
            admit_topology_derived_invalidation_prior_proof_identity("shared-digest"),
            None,
        ),
    ));
    let spatial = admit_replay_scope_identity(ReplayScopeIdentityInput::new(
        ReplayUndoSemanticGraphEquivalenceBasis::new(
            ReplayUndoSemanticGraphLocalityScope::TopologyTouchedClosure,
            vec![ReplayUndoSemanticGraphTouchedSubject::TopologyAspect {
                aspect: WorthTopologyTouchedAspect::TopologyStructure,
            }],
            admit_spatial_evidence_lookup_prior_proof_identity("shared-digest"),
            None,
        ),
    ));

    assert_ne!(topology.digest(), spatial.digest());
}

#[test]
fn undo_identity_drifts_when_stage_index_identity_changes() {
    let first = admit_undo_scope_identity(UndoScopeIdentityInput::new(
        ReplayUndoSemanticGraphEquivalenceBasis::new(
            ReplayUndoSemanticGraphLocalityScope::SpatialTouchAuthority,
            vec![
                ReplayUndoSemanticGraphTouchedSubject::SpatialAuthorityStage {
                    evidence_stage: "event-ledger".to_string(),
                    evidence_identity: "row-a".to_string(),
                },
            ],
            admit_spatial_evidence_lookup_prior_proof_identity("lookup"),
            Some(admit_replay_undo_stage_index_identity("stage-index-a")),
        ),
    ));
    let second = admit_undo_scope_identity(UndoScopeIdentityInput::new(
        ReplayUndoSemanticGraphEquivalenceBasis::new(
            ReplayUndoSemanticGraphLocalityScope::SpatialTouchAuthority,
            vec![
                ReplayUndoSemanticGraphTouchedSubject::SpatialAuthorityStage {
                    evidence_stage: "event-ledger".to_string(),
                    evidence_identity: "row-a".to_string(),
                },
            ],
            admit_spatial_evidence_lookup_prior_proof_identity("lookup"),
            Some(admit_replay_undo_stage_index_identity("stage-index-b")),
        ),
    ));

    assert_ne!(first.digest(), second.digest());
}
