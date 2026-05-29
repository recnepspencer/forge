use forge_relational::facade::history::BranchId;
use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::topology_authoring::verify_topology_intent_on_branch;
use schema::facade::{MutationOrigin, RawTopologyIntent, TopologyMutation, VerifiedTopologyCommit};

use super::super::super::super::error::TopologyCertificationError;
use crate::certification::shared::digest_rows;
use crate::topology_operators::{
    TopologyEditApplicationMode, TopologyEditBatch, TopologyEditDigest,
};

pub(super) struct AcceptedBranchExecution {
    pub(super) branch_label: String,
    pub(super) branch_id: String,
    pub(super) branch_head_diverged_from_main: bool,
    pub(super) branch_truth_digest: crate::certification::DeterministicDigest,
    pub(super) batches: Vec<TopologyEditBatch>,
}

pub(super) struct BranchSetup {
    pub(super) branch_label: String,
    pub(super) branch_id: BranchId,
    pub(super) main_head_before_edit: forge_relational::facade::history::CommitId,
}

pub(super) fn create_branch(
    runtime: &mut RelationalRuntime,
    stem: &str,
    label: &str,
) -> Result<BranchSetup, TopologyCertificationError> {
    let branch_id = BranchId(format!("{stem}.branch_local_{label}"));
    runtime
        .history_authority()
        .create_branch(branch_id.clone(), &BranchId("main".to_string()))
        .map_err(|error| TopologyCertificationError::Query(format!("{error:?}")))?;
    let main_head_before_edit = runtime
        .history()
        .branch_head(&BranchId("main".to_string()))
        .ok_or_else(|| TopologyCertificationError::Query("main branch head missing".into()))?
        .commit_id;
    Ok(BranchSetup {
        branch_label: branch_id.0.clone(),
        branch_id,
        main_head_before_edit,
    })
}

pub(super) fn apply_branch_batch(
    runtime: &mut RelationalRuntime,
    branch_id: &BranchId,
    batch: TopologyEditBatch,
) -> Result<VerifiedTopologyCommit, TopologyCertificationError> {
    let mode = TopologyEditApplicationMode::BranchLocal(branch_id.clone());
    verify_topology_intent_on_branch(runtime, batch.into_raw_intent(&mode), branch_id.clone())
        .map_err(|failure| TopologyCertificationError::Query(format!("{:?}", failure.into_error())))
}

pub(super) fn apply_branch_mutations(
    runtime: &mut RelationalRuntime,
    branch_id: &BranchId,
    mutations: Vec<TopologyMutation>,
) -> Result<VerifiedTopologyCommit, TopologyCertificationError> {
    verify_topology_intent_on_branch(
        runtime,
        RawTopologyIntent::new(mutations, MutationOrigin::BranchLocalApplication),
        branch_id.clone(),
    )
    .map_err(|failure| TopologyCertificationError::Query(format!("{:?}", failure.into_error())))
}

pub(super) fn execution_from_verified(
    runtime: &RelationalRuntime,
    branch: BranchSetup,
    batches: Vec<TopologyEditBatch>,
    verified: Vec<VerifiedTopologyCommit>,
) -> Result<AcceptedBranchExecution, TopologyCertificationError> {
    let branch_head_after_edit = runtime
        .history()
        .branch_head(&branch.branch_id)
        .ok_or_else(|| TopologyCertificationError::Query("accepted branch head missing".into()))?
        .commit_id;
    let main_head_after_edit = runtime
        .history()
        .branch_head(&BranchId("main".to_string()))
        .ok_or_else(|| TopologyCertificationError::Query("main branch head missing".into()))?
        .commit_id;
    let branch_truth_digest = digest_rows(verified.iter().flat_map(|commit| {
        commit
            .canonical_batch
            .batch
            .mutations
            .iter()
            .map(|mutation| serde_json::to_string(mutation).expect("mutation serializes"))
    }));
    Ok(AcceptedBranchExecution {
        branch_label: branch.branch_label,
        branch_id: branch.branch_id.0,
        branch_head_diverged_from_main: branch_head_after_edit != main_head_after_edit
            && branch.main_head_before_edit == main_head_after_edit,
        branch_truth_digest,
        batches,
    })
}

pub(super) fn edit_digest_shape_matches(
    left: &TopologyEditDigest,
    right: &TopologyEditDigest,
) -> bool {
    left.contract_count == right.contract_count
        && left.family_count == right.family_count
        && left.changed_scope_count == right.changed_scope_count
        && left.naming_scope_count == right.naming_scope_count
        && left.derived_region_count == right.derived_region_count
        && left.fallback_policy_count == right.fallback_policy_count
        && left.fallback_rejection_policy_count == right.fallback_rejection_policy_count
}
