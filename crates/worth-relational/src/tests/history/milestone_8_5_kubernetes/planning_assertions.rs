use super::*;

pub(super) fn planning_for(
    runtime: &RelationalRuntime,
    source_branch: BranchId,
    target_branch: BranchId,
) -> crate::merge::data::MergePlanningArtifactCore {
    runtime
        .merge()
        .inspect_planning_scope(MergePlanningRequest::new(
            target_branch,
            source_branch,
            MergeIntent::ReconcileIntoTarget,
        ))
        .expect("merge planning")
}

fn entity_classification<'a>(
    planning: &'a crate::merge::data::MergePlanningArtifactCore,
    entity: crate::facade::identity::EntityId,
) -> Option<&'a crate::merge::data::MergeConflictClassification> {
    planning
        .conflict_classification
        .classifications
        .iter()
        .find(|classification| {
            classification.record == crate::facade::transactions::RecordRef::Entity(entity)
        })
}

pub(super) fn assert_strategy_conflict(
    planning: &crate::merge::data::MergePlanningArtifactCore,
    entity: crate::facade::identity::EntityId,
    stage: &str,
) {
    let classification = entity_classification(planning, entity).unwrap_or_else(|| {
        panic!("missing entity classification for strategy-conflict stage {stage}")
    });
    assert!(
        classification.class == crate::merge::data::MergeConflictClass::StrategyIntentConflict,
        "expected strategy intent conflict during {stage}, got {:?}",
        classification.class
    );
    assert!(
        classification.strategy_evidence.is_some(),
        "expected strategy evidence for overlap conflict: {classification:?}"
    );
}

pub(super) fn assert_non_strategy_conflict(
    planning: &crate::merge::data::MergePlanningArtifactCore,
    entity: crate::facade::identity::EntityId,
    stage: &str,
) {
    let classification = entity_classification(planning, entity)
        .unwrap_or_else(|| panic!("missing entity classification for benign stage {stage}"));
    assert_ne!(
        classification.class,
        crate::merge::data::MergeConflictClass::StrategyIntentConflict
    );
    assert!(
        classification.class == crate::merge::data::MergeConflictClass::ExactSharedTruth,
        "expected benign exact-shared-truth classification during {stage}, got {:?}",
        classification.class
    );
}

pub(super) fn assert_exact_shared_truth(
    planning: &crate::merge::data::MergePlanningArtifactCore,
    entity: crate::facade::identity::EntityId,
    stage: &str,
) {
    let classification = entity_classification(planning, entity).unwrap_or_else(|| {
        panic!("missing entity classification for exact-shared-truth stage {stage}")
    });
    assert!(
        classification.class == crate::merge::data::MergeConflictClass::ExactSharedTruth,
        "expected exact shared truth during {stage}, got {:?}",
        classification.class
    );
    assert!(
        classification.strategy_evidence.is_some(),
        "expected preserved strategy evidence during exact shared truth stage {stage}: {classification:?}"
    );
}
