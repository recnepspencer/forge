use forge_relational::facade::runtime::RelationalRuntime;

use super::super::super::super::error::TopologyCertificationError;
use super::super::super::mutation_sequence_support::{
    aggregate_naming_mutation_continuity_matrix_for_plans,
    aggregate_topology_mutation_digest_for_plans, closeout_mutation_plan_for_declaration,
    TopologyCloseoutMutationPlan,
};
use crate::test_support::schema_topology_authoring_boundary::{
    commit_topology_intent_on_branch_through_schema_execution,
    open_schema_topology_authoring_branch_execution, SchemaTopologyAuthoringBranchExecutionLedger,
};
use crate::topology_operators::application::TopologyDeclarationMutationPayload;
use crate::topology_operators::{
    NamingMutationContinuityMatrix, TopologyMutationDerivedFallbackPolicy, TopologyMutationDigest,
    TopologyMutationFamily,
};

use super::accepted_branch_schema_authority_projection::AcceptedBranchSchemaAuthorityProjection;

pub(super) struct AcceptedBranchExecution {
    pub(super) branch_label: String,
    pub(super) branch_id: String,
    pub(super) branch_head_diverged_from_main: bool,
    pub(super) branch_truth_digest: crate::certification::DeterministicDigest,
    pub(super) topology_mutation_digest: TopologyMutationDigest,
    pub(super) naming_mutation_continuity_matrix: NamingMutationContinuityMatrix,
    pub(super) mutation_families: Vec<TopologyMutationFamily>,
    pub(super) derived_fallback_policy: TopologyMutationDerivedFallbackPolicy,
}

pub(super) fn create_branch(
    runtime: &mut RelationalRuntime,
    stem: &str,
    label: &str,
) -> Result<SchemaTopologyAuthoringBranchExecutionLedger, TopologyCertificationError> {
    open_schema_topology_authoring_branch_execution(runtime, format!("{stem}.branch_local_{label}"))
        .map_err(TopologyCertificationError::Query)
}

pub(super) fn apply_branch_plan(
    runtime: &mut RelationalRuntime,
    branch_execution: &mut SchemaTopologyAuthoringBranchExecutionLedger,
    plan: &TopologyCloseoutMutationPlan,
) -> Result<AcceptedBranchSchemaAuthorityProjection, TopologyCertificationError> {
    let commit_input = commit_topology_intent_on_branch_through_schema_execution(
        runtime,
        branch_execution,
        plan.raw_intent.clone(),
    )
    .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    Ok(AcceptedBranchSchemaAuthorityProjection::from_plan(
        commit_input,
        plan.clone(),
    ))
}

pub(super) fn apply_branch_declaration<D>(
    runtime: &mut RelationalRuntime,
    branch_execution: &mut SchemaTopologyAuthoringBranchExecutionLedger,
    declaration: D,
) -> Result<AcceptedBranchSchemaAuthorityProjection, TopologyCertificationError>
where
    D: TopologyDeclarationMutationPayload,
{
    let plan = closeout_mutation_plan_for_declaration(declaration);
    apply_branch_plan(runtime, branch_execution, &plan)
}

pub(super) fn execution_from_verified_plans(
    runtime: &RelationalRuntime,
    branch_execution: SchemaTopologyAuthoringBranchExecutionLedger,
    projections: Vec<AcceptedBranchSchemaAuthorityProjection>,
) -> Result<AcceptedBranchExecution, TopologyCertificationError> {
    let derived_fallback_policy = if projections.iter().any(|projection| {
        projection.derived_fallback_policy()
            == TopologyMutationDerivedFallbackPolicy::RejectAnyFallback
    }) {
        TopologyMutationDerivedFallbackPolicy::RejectAnyFallback
    } else {
        TopologyMutationDerivedFallbackPolicy::AllowExplicitFallback
    };
    let plans: Vec<TopologyCloseoutMutationPlan> = projections
        .iter()
        .map(|projection| projection.plan().clone())
        .collect();
    Ok(AcceptedBranchExecution {
        branch_label: branch_execution.branch_label().to_string(),
        branch_id: branch_execution.branch_id().0.clone(),
        branch_head_diverged_from_main: branch_execution
            .branch_head_diverged_from_main(runtime)
            .map_err(TopologyCertificationError::Query)?,
        branch_truth_digest: branch_execution.branch_truth_digest(),
        topology_mutation_digest: aggregate_topology_mutation_digest_for_plans(plans.clone()),
        naming_mutation_continuity_matrix: aggregate_naming_mutation_continuity_matrix_for_plans(
            plans.clone(),
        ),
        mutation_families: plans
            .into_iter()
            .flat_map(|plan| plan.mutation_families)
            .collect(),
        derived_fallback_policy,
    })
}

pub(super) fn execution_from_verified_declarations<D>(
    runtime: &RelationalRuntime,
    branch_execution: SchemaTopologyAuthoringBranchExecutionLedger,
    declarations: Vec<D>,
    projections: Vec<AcceptedBranchSchemaAuthorityProjection>,
) -> Result<AcceptedBranchExecution, TopologyCertificationError>
where
    D: TopologyDeclarationMutationPayload,
{
    let expected_families = declarations
        .into_iter()
        .flat_map(|declaration| declaration.semantic_families())
        .collect::<Vec<_>>();
    let execution = execution_from_verified_plans(runtime, branch_execution, projections)?;
    if execution.mutation_families != expected_families {
        return Err(TopologyCertificationError::Query(
            "accepted branch execution drifted from declaration family sequence".to_string(),
        ));
    }
    Ok(execution)
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
