use forge_relational::facade::history::BranchId;
use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::platform::authority::{MutationOrigin, RawTopologyIntent, TopologyMutation};

use super::super::super::super::error::TopologyCertificationError;
use super::super::super::edit_sequence_support::{
    aggregate_naming_edit_continuity_matrix_for_contract_sets,
    aggregate_naming_edit_continuity_matrix_for_declarations,
    aggregate_topology_edit_digest_for_contract_sets,
    aggregate_topology_edit_digest_for_declarations,
    branch_local_raw_topology_intent_for_declaration, topology_edit_families_for_declarations,
};
use crate::certification::shared::digest_rows;
use crate::committed_artifact::TopologyCommittedArtifact;
use crate::test_support::topology_commit::commit_topology_intent_on_branch;
use crate::topology_operators::application::TopologyDeclarationContractPayload;
use crate::topology_operators::{
    topology_edit_families_for_contracts, NamingEditContinuityMatrix, TopologyEditContract,
    TopologyEditDigest, TopologyEditFamily,
};

pub(super) struct AcceptedBranchExecution {
    pub(super) branch_label: String,
    pub(super) branch_id: String,
    pub(super) branch_head_diverged_from_main: bool,
    pub(super) branch_truth_digest: crate::certification::DeterministicDigest,
    pub(super) topology_edit_digest: TopologyEditDigest,
    pub(super) naming_edit_continuity_matrix: NamingEditContinuityMatrix,
    pub(super) edit_families: Vec<TopologyEditFamily>,
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

pub(super) fn apply_branch_declaration<D>(
    runtime: &mut RelationalRuntime,
    branch_id: &BranchId,
    declaration: D,
) -> Result<TopologyCommittedArtifact, TopologyCertificationError>
where
    D: TopologyDeclarationContractPayload,
{
    commit_topology_intent_on_branch(
        runtime,
        branch_local_raw_topology_intent_for_declaration(declaration),
        branch_id.clone(),
    )
    .map_err(|error| TopologyCertificationError::Query(error.to_string()))
}

pub(super) fn apply_branch_mutations(
    runtime: &mut RelationalRuntime,
    branch_id: &BranchId,
    mutations: Vec<TopologyMutation>,
) -> Result<TopologyCommittedArtifact, TopologyCertificationError> {
    commit_topology_intent_on_branch(
        runtime,
        RawTopologyIntent::new(mutations, MutationOrigin::BranchLocalApplication),
        branch_id.clone(),
    )
    .map_err(|error| TopologyCertificationError::Query(error.to_string()))
}

pub(super) fn execution_from_verified_contract_sets(
    runtime: &RelationalRuntime,
    branch: BranchSetup,
    contract_sets: Vec<Vec<TopologyEditContract>>,
    verified: Vec<TopologyCommittedArtifact>,
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
    let topology_edit_digest =
        aggregate_topology_edit_digest_for_contract_sets(contract_sets.clone());
    let naming_edit_continuity_matrix =
        aggregate_naming_edit_continuity_matrix_for_contract_sets(contract_sets.clone());
    let edit_families = contract_sets
        .iter()
        .flat_map(|contracts| topology_edit_families_for_contracts(contracts))
        .collect();
    Ok(AcceptedBranchExecution {
        branch_label: branch.branch_label,
        branch_id: branch.branch_id.0,
        branch_head_diverged_from_main: branch_head_after_edit != main_head_after_edit
            && branch.main_head_before_edit == main_head_after_edit,
        branch_truth_digest,
        topology_edit_digest,
        naming_edit_continuity_matrix,
        edit_families,
    })
}

pub(super) fn execution_from_verified_declarations<D>(
    runtime: &RelationalRuntime,
    branch: BranchSetup,
    declarations: Vec<D>,
    verified: Vec<TopologyCommittedArtifact>,
) -> Result<AcceptedBranchExecution, TopologyCertificationError>
where
    D: TopologyDeclarationContractPayload,
{
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
        topology_edit_digest: aggregate_topology_edit_digest_for_declarations(declarations.clone()),
        naming_edit_continuity_matrix: aggregate_naming_edit_continuity_matrix_for_declarations(
            declarations.clone(),
        ),
        edit_families: topology_edit_families_for_declarations(declarations),
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
