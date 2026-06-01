use forge_relational::facade::history::BranchId;
use forge_relational::facade::runtime::RelationalRuntime;

use super::super::super::super::error::TopologyCertificationError;
use super::super::super::mutation_sequence_support::{
    aggregate_naming_mutation_continuity_matrix_for_plans,
    aggregate_topology_mutation_digest_for_plans, closeout_mutation_plan_for_declaration,
    topology_mutation_families_for_declarations, TopologyCloseoutMutationPlan,
};
use crate::certification::shared::digest_rows;
use crate::committed_artifact::TopologyCommittedArtifact;
use crate::test_support::topology_commit::commit_topology_intent_on_branch;
use crate::topology_operators::application::TopologyDeclarationMutationPayload;
use crate::topology_operators::{
    NamingMutationContinuityMatrix, TopologyMutationDigest, TopologyMutationFamily,
};

pub(super) struct AcceptedBranchExecution {
    pub(super) branch_label: String,
    pub(super) branch_id: String,
    pub(super) branch_head_diverged_from_main: bool,
    pub(super) branch_truth_digest: crate::certification::DeterministicDigest,
    pub(super) topology_mutation_digest: TopologyMutationDigest,
    pub(super) naming_mutation_continuity_matrix: NamingMutationContinuityMatrix,
    pub(super) mutation_families: Vec<TopologyMutationFamily>,
}

pub(super) struct BranchSetup {
    pub(super) branch_label: String,
    pub(super) branch_id: BranchId,
    pub(super) main_head_before_mutation: forge_relational::facade::history::CommitId,
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
    let main_head_before_mutation = runtime
        .history()
        .branch_head(&BranchId("main".to_string()))
        .ok_or_else(|| TopologyCertificationError::Query("main branch head missing".into()))?
        .commit_id;
    Ok(BranchSetup {
        branch_label: branch_id.0.clone(),
        branch_id,
        main_head_before_mutation,
    })
}

pub(super) fn apply_branch_plan(
    runtime: &mut RelationalRuntime,
    branch_id: &BranchId,
    plan: &TopologyCloseoutMutationPlan,
) -> Result<TopologyCommittedArtifact, TopologyCertificationError> {
    commit_topology_intent_on_branch(runtime, plan.raw_intent.clone(), branch_id.clone())
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))
}

pub(super) fn apply_branch_declaration<D>(
    runtime: &mut RelationalRuntime,
    branch_id: &BranchId,
    declaration: D,
) -> Result<TopologyCommittedArtifact, TopologyCertificationError>
where
    D: TopologyDeclarationMutationPayload,
{
    let plan = closeout_mutation_plan_for_declaration(declaration);
    apply_branch_plan(runtime, branch_id, &plan)
}

pub(super) fn execution_from_verified_plans(
    runtime: &RelationalRuntime,
    branch: BranchSetup,
    plans: Vec<TopologyCloseoutMutationPlan>,
    verified: Vec<TopologyCommittedArtifact>,
) -> Result<AcceptedBranchExecution, TopologyCertificationError> {
    let branch_head_after_mutation = runtime
        .history()
        .branch_head(&branch.branch_id)
        .ok_or_else(|| TopologyCertificationError::Query("accepted branch head missing".into()))?
        .commit_id;
    let main_head_after_mutation = runtime
        .history()
        .branch_head(&BranchId("main".to_string()))
        .ok_or_else(|| TopologyCertificationError::Query("main branch head missing".into()))?
        .commit_id;
    let branch_truth_digest = digest_rows(verified.iter().flat_map(|commit| {
        commit
            .mutations()
            .iter()
            .map(|mutation| serde_json::to_string(mutation).expect("mutation serializes"))
    }));
    let topology_mutation_digest = aggregate_topology_mutation_digest_for_plans(plans.clone());
    let naming_mutation_continuity_matrix =
        aggregate_naming_mutation_continuity_matrix_for_plans(plans.clone());
    let mutation_families = plans
        .into_iter()
        .flat_map(|plan| plan.mutation_families)
        .collect();
    Ok(AcceptedBranchExecution {
        branch_label: branch.branch_label,
        branch_id: branch.branch_id.0,
        branch_head_diverged_from_main: branch_head_after_mutation != main_head_after_mutation
            && branch.main_head_before_mutation == main_head_after_mutation,
        branch_truth_digest,
        topology_mutation_digest,
        naming_mutation_continuity_matrix,
        mutation_families,
    })
}

pub(super) fn execution_from_verified_declarations<D>(
    runtime: &RelationalRuntime,
    branch: BranchSetup,
    declarations: Vec<D>,
    verified: Vec<TopologyCommittedArtifact>,
) -> Result<AcceptedBranchExecution, TopologyCertificationError>
where
    D: TopologyDeclarationMutationPayload,
{
    let branch_head_after_mutation = runtime
        .history()
        .branch_head(&branch.branch_id)
        .ok_or_else(|| TopologyCertificationError::Query("accepted branch head missing".into()))?
        .commit_id;
    let main_head_after_mutation = runtime
        .history()
        .branch_head(&BranchId("main".to_string()))
        .ok_or_else(|| TopologyCertificationError::Query("main branch head missing".into()))?
        .commit_id;
    let branch_truth_digest = digest_rows(verified.iter().flat_map(|commit| {
        commit
            .mutations()
            .iter()
            .map(|mutation| serde_json::to_string(mutation).expect("mutation serializes"))
    }));
    let plans = declarations
        .clone()
        .into_iter()
        .map(closeout_mutation_plan_for_declaration)
        .collect::<Vec<_>>();
    Ok(AcceptedBranchExecution {
        branch_label: branch.branch_label,
        branch_id: branch.branch_id.0,
        branch_head_diverged_from_main: branch_head_after_mutation != main_head_after_mutation
            && branch.main_head_before_mutation == main_head_after_mutation,
        branch_truth_digest,
        topology_mutation_digest: aggregate_topology_mutation_digest_for_plans(plans.clone()),
        naming_mutation_continuity_matrix: aggregate_naming_mutation_continuity_matrix_for_plans(
            plans,
        ),
        mutation_families: topology_mutation_families_for_declarations(declarations),
    })
}

pub(super) fn mutation_digest_shape_matches(
    left: &TopologyMutationDigest,
    right: &TopologyMutationDigest,
) -> bool {
    left.mutation_record_count == right.mutation_record_count
        && left.family_count == right.family_count
        && left.changed_scope_count == right.changed_scope_count
        && left.naming_scope_count == right.naming_scope_count
        && left.derived_region_count == right.derived_region_count
        && left.fallback_policy_count == right.fallback_policy_count
        && left.fallback_rejection_policy_count == right.fallback_rejection_policy_count
}
