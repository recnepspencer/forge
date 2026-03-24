use crate::facade::history::{BranchId, CommitReference};
use crate::facade::indexes::{
    DerivedIndexBuildOutcome, DerivedIndexBuildRequest, DerivedIndexDefinition, DerivedIndexId,
    DerivedIndexKind,
};
use crate::facade::lineage::LineageResolutionStatus;

use super::super::fixture::{FintechCaseRole, FintechWorld};

pub(crate) fn register_case_book_index(world: &mut FintechWorld) -> DerivedIndexDefinition {
    world
        .runtime
        .index_authority()
        .register(DerivedIndexDefinition {
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
    world
        .runtime
        .index_authority()
        .build_for_commit(DerivedIndexBuildRequest {
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
    let authoritative_commit = world
        .runtime
        .history_access()
        .branch_head(&commit.branch_id)
        .cloned()
        .unwrap_or(commit.clone());
    let left_lineage = world
        .runtime
        .lineage_access()
        .for_record(world.workflow_case(left).trade)
        .expect("left case trade should have lineage")
        .lineage_id;
    let right_lineage = world
        .runtime
        .lineage_access()
        .for_record(world.workflow_case(right).trade)
        .expect("right case trade should have lineage")
        .lineage_id;
    let candidate = world
        .runtime
        .lineage_authority()
        .record_correspondence_candidate(
            authoritative_commit.branch_id.clone(),
            vec![left_lineage],
            vec![right_lineage],
            "fintech-case-correspondence",
        );
    world
        .runtime
        .lineage_authority()
        .promote_correspondence(candidate.candidate_id, authoritative_commit)
        .expect("candidate should promote")
        .status()
}
