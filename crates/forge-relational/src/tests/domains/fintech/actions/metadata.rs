use crate::facade::{
    BranchId, CommitReference, DerivedIndexBuildOutcome, DerivedIndexBuildRequest,
    DerivedIndexDefinition, DerivedIndexId, DerivedIndexKind, LineageResolutionStatus,
};

use super::super::fixture::{FintechCaseRole, FintechWorld};

pub(crate) fn register_case_book_index(world: &mut FintechWorld) -> DerivedIndexDefinition {
    world.runtime.register_index(DerivedIndexDefinition {
        index_id: DerivedIndexId(0),
        name: "fintech.trade.book".to_string(),
        kind: DerivedIndexKind::EntityPayloadField {
            field: "book".to_string(),
        },
        branch_scoped: true,
    })
}

pub(crate) fn build_branch_scoped_case_index(
    world: &mut FintechWorld,
    index_id: DerivedIndexId,
    branch_id: BranchId,
    source_commit: CommitReference,
) -> DerivedIndexBuildOutcome {
    world.runtime.build_indexes_for_commit(DerivedIndexBuildRequest {
        source_commit_id: source_commit.commit_id,
        branch_id,
        index_ids: vec![index_id],
    })
}

pub(crate) fn promote_case_correspondence(
    world: &mut FintechWorld,
    left: FintechCaseRole,
    right: FintechCaseRole,
    commit: CommitReference,
) -> LineageResolutionStatus {
    let left_lineage = world
        .runtime
        .lineage_for_record(world.workflow_case(left).trade)
        .expect("left case trade should have lineage")
        .lineage_id;
    let right_lineage = world
        .runtime
        .lineage_for_record(world.workflow_case(right).trade)
        .expect("right case trade should have lineage")
        .lineage_id;
    let candidate = world.runtime.record_correspondence_candidate(
        BranchId("main".to_string()),
        vec![left_lineage],
        vec![right_lineage],
        "fintech-case-correspondence",
    );
    world
        .runtime
        .promote_correspondence(candidate.candidate_id, commit)
        .expect("candidate should promote")
        .status
}
