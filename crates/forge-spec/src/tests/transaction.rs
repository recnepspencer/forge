use crate::facade::*;

#[test]
fn draft_commit_creates_immutable_state() {
    let state = SpecState::empty();
    let mut draft = state.into_draft();
    let model = draft.create_node(SpecNodeKind::Model, None, "model").unwrap();
    let feature = draft.create_node(SpecNodeKind::Feature, None, "feature").unwrap();
    draft
        .add_relation(RelationKind::ModelOwnsFeature, model, feature, 0, "ownership")
        .unwrap();
    let committed = draft.commit().unwrap();

    assert_eq!(committed.epoch(), 1);
    assert!(committed.graph().contains_node(model));
    assert!(committed.graph().contains_node(feature));
    assert_eq!(committed.graph().iter_relations().count(), 1);
}

#[test]
fn rollback_restores_prior_snapshot() {
    let state = SpecState::empty();
    let mut draft = state.clone().into_draft();
    let _ = draft.create_node(SpecNodeKind::Feature, None, "temp").unwrap();
    let rolled_back = draft.rollback().unwrap();
    assert_eq!(rolled_back, state);
}

#[test]
fn stable_node_ids_survive_snapshot_commit() {
    let state = SpecState::empty();
    let mut draft = state.into_draft();
    let model = draft.create_node(SpecNodeKind::Model, None, "model").unwrap();
    let committed = draft.commit().unwrap();
    let node = committed.graph().node(model).unwrap();
    assert_eq!(node.id, model);
}

#[test]
fn naming_lineage_and_replay_records_commit() {
    let state = SpecState::empty();
    let mut draft = state.into_draft();
    let feature = draft.create_node(SpecNodeKind::Feature, None, "feature").unwrap();
    let body = draft.create_node(SpecNodeKind::Body, None, "body").unwrap();
    draft
        .create_naming_anchor(body, SpecNodeKind::Body, "primary-body", 0, Some(feature), 1)
        .unwrap();
    draft
        .record_lineage(LineageRecord {
            node: body,
            producing_feature: Some(feature),
            creation_operation: 1,
            parent_nodes: Vec::new(),
            ancestry_hash: 99,
            derivation_role: Some("root".to_string()),
        })
        .unwrap();
    draft
        .record_replay(SpecReplayRecord {
            operation_id: 1,
            operation_name: "make_body".to_string(),
            schema_version: 1,
            parameters: Vec::new(),
            pre_hash: 0,
            post_hash: 1,
            touched_nodes: vec![body],
            touched_relations: Vec::new(),
            mutation_trace: vec!["body created".to_string()],
            projection_refresh_trace: Vec::new(),
            decision_summary: None,
        })
        .unwrap();
    let committed = draft.commit().unwrap();
    assert_eq!(committed.naming_anchors().len(), 1);
    assert_eq!(committed.lineage_records().len(), 1);
    assert_eq!(committed.replay_records().len(), 1);
}
