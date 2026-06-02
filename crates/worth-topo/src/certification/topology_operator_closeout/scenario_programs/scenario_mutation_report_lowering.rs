use crate::certification::error::TopologyCertificationError;
use crate::certification::topology_operator_closeout::replay_step_rows::{
    aggregate_fallback_summary_from_step_rows, aggregate_mutation_families_from_step_rows,
    aggregate_naming_mutation_continuity_matrix_from_step_rows,
    aggregate_topology_mutation_digest_from_step_rows,
};
use crate::certification::topology_operator_closeout::report::{
    MilestoneThreeMutationReplayStepRow, MilestoneThreeScenarioMutationSemanticSummary,
    MilestoneThreeScenarioMutationSynopsis,
};
use crate::topology_operators::application::TopologyDeclarationMutationPayload;

pub(in crate::certification::topology_operator_closeout) fn accepted_mutation_synopsis_from_step_rows(
    step_rows: &[MilestoneThreeMutationReplayStepRow],
) -> MilestoneThreeScenarioMutationSynopsis {
    MilestoneThreeScenarioMutationSynopsis {
        mutation_families: aggregate_mutation_families_from_step_rows(step_rows),
        topology_mutation_digest: aggregate_topology_mutation_digest_from_step_rows(step_rows),
    }
}

pub(in crate::certification::topology_operator_closeout) fn hostile_scenario_mutation_synopsis_from_declaration<
    D,
>(
    declaration: &D,
) -> MilestoneThreeScenarioMutationSynopsis
where
    D: TopologyDeclarationMutationPayload,
{
    MilestoneThreeScenarioMutationSynopsis {
        mutation_families: declaration.semantic_families(),
        topology_mutation_digest: declaration.topology_mutation_digest(),
    }
}

pub(in crate::certification::topology_operator_closeout) fn accepted_semantic_summary_from_step_rows(
    step_rows: &[MilestoneThreeMutationReplayStepRow],
    detail_context: &str,
) -> Result<MilestoneThreeScenarioMutationSemanticSummary, TopologyCertificationError> {
    let derived_fallback_policy =
        aggregate_fallback_summary_from_step_rows(step_rows).ok_or_else(|| {
            TopologyCertificationError::Query(format!(
                "{detail_context} should retain fallback evidence"
            ))
        })?;
    let naming_mutation_continuity_matrix =
        aggregate_naming_mutation_continuity_matrix_from_step_rows(step_rows);
    let continuity_outcome_class = naming_mutation_continuity_matrix.outcome_class();
    let continuity_rejection_class = naming_mutation_continuity_matrix.rejection_class();

    Ok(MilestoneThreeScenarioMutationSemanticSummary {
        naming_mutation_continuity_matrix,
        derived_fallback_policy: Some(derived_fallback_policy),
        continuity_outcome_class,
        continuity_rejection_class,
    })
}

pub(in crate::certification::topology_operator_closeout) fn hostile_scenario_semantic_summary_from_rejected_declaration<
    D,
>(
    declaration: &D,
    _rejection_class: Option<crate::topology_operators::TopologyMutationRejectionClass>,
) -> MilestoneThreeScenarioMutationSemanticSummary
where
    D: TopologyDeclarationMutationPayload,
{
    let naming_mutation_continuity_matrix = declaration.naming_continuity_matrix();
    MilestoneThreeScenarioMutationSemanticSummary {
        continuity_outcome_class: naming_mutation_continuity_matrix.outcome_class(),
        continuity_rejection_class: naming_mutation_continuity_matrix.rejection_class(),
        naming_mutation_continuity_matrix,
        derived_fallback_policy: None,
    }
}
