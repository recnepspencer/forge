use super::*;
use crate::tests::milestone_6_certification::suite_helpers::*;

pub(super) fn admitted_truth_parity() -> Vec<LaneResult<String>> {
    [StoreLane::InMemory, StoreLane::LocalFile, StoreLane::Sqlite]
        .into_iter()
        .map(|lane| {
            let bundle = entity_set_bundle_for_lane(lane);
            LaneResult::new(
                lane.label(),
                serde_json::to_string(&canonical_truth(&bundle)).unwrap(),
            )
        })
        .collect::<Vec<_>>()
}

pub(super) fn admitted_counter_parity() -> Vec<LaneResult<String>> {
    [StoreLane::InMemory, StoreLane::LocalFile, StoreLane::Sqlite]
        .into_iter()
        .map(|lane| {
            let bundle = entity_set_bundle_for_lane(lane);
            LaneResult::new(
                lane.label(),
                serde_json::to_string(&bundle.counter_contract).unwrap(),
            )
        })
        .collect::<Vec<_>>()
}

pub(super) fn admitted_artifact_parity() -> Vec<LaneResult<String>> {
    [StoreLane::InMemory, StoreLane::LocalFile, StoreLane::Sqlite]
        .into_iter()
        .map(|lane| {
            let bundle = entity_set_bundle_for_lane(lane);
            LaneResult::new(lane.label(), bundle.artifact_digest.clone())
        })
        .collect::<Vec<_>>()
}

pub(super) fn scope_shape_divergence() -> Vec<LaneResult<String>> {
    vec![
        LaneResult::new(
            "single_entity",
            serde_json::to_string(&canonical_truth(&single_entity_bundle_for_lane(
                StoreLane::InMemory,
            )))
            .unwrap(),
        ),
        LaneResult::new(
            "entity_set_uniform",
            serde_json::to_string(&canonical_truth(&entity_set_bundle_for_lane(
                StoreLane::InMemory,
            )))
            .unwrap(),
        ),
    ]
}

pub(super) fn generalized_scope_rejection() -> Vec<LaneResult<String>> {
    let (store, root) = store_for_lane_with_root(StoreLane::InMemory, "fallback");
    let error = store
        .milestone_6_certification_bundle(request_for_scope(
            &root,
            AspectScopeClass::Generalized {
                descriptor: "wildcard-join".to_string(),
            },
            &["profile"],
        ))
        .unwrap_err();
    vec![LaneResult::new(
        "generalized_scope",
        serde_json::to_string(&fallback_surface(&error, store.counters())).unwrap(),
    )]
}
