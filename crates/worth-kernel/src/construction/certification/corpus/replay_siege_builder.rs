use forge_query::facade::ForgeQueryWorkspace;

use super::cases::PrimitiveConstructionCorpusScenario;
use super::execution::prepare_corpus_execution_proof_ingredients;
use super::replay_siege::PrimitiveConstructionCorpusReplaySiegeError;
use super::replay_siege_report::{
    PrimitiveConstructionCorpusOutcomeDisposition, PrimitiveConstructionCorpusReplaySiegeRow,
};
use super::row_support::{
    birth_attachment_breadth, certification_breadth, construction_breadth,
    rejection_locality_row_for,
};
use crate::construction::authoring::primitive_construction_authoring;
use crate::construction::outcome::PrimitiveConstructionPreparedOutcome;

pub(super) fn build_corpus_rows(
    workspace: &mut ForgeQueryWorkspace,
    scenarios: &[PrimitiveConstructionCorpusScenario],
) -> Result<
    Vec<PrimitiveConstructionCorpusReplaySiegeRow>,
    PrimitiveConstructionCorpusReplaySiegeError,
> {
    let mut rows = Vec::new();
    for scenario in scenarios {
        let row = build_corpus_row(workspace, scenario)?;
        rows.push(row);
    }
    Ok(rows)
}

fn build_corpus_row(
    workspace: &mut ForgeQueryWorkspace,
    scenario: &PrimitiveConstructionCorpusScenario,
) -> Result<PrimitiveConstructionCorpusReplaySiegeRow, PrimitiveConstructionCorpusReplaySiegeError>
{
    let execution = prepare_corpus_execution_proof_ingredients(
        workspace,
        scenario.intent.clone(),
        || PrimitiveConstructionCorpusReplaySiegeError::ReplayParityDrift {
            family: scenario.family,
            parameter_role: scenario.parameter_role,
        },
        || PrimitiveConstructionCorpusReplaySiegeError::BranchLocalParityDrift {
            family: scenario.family,
            parameter_role: scenario.parameter_role,
        },
    )?;

    match execution.direct_outcome().clone() {
        PrimitiveConstructionPreparedOutcome::Accepted(outcome) => {
            let result = {
                let mut session = primitive_construction_authoring(workspace).map_err(|error| {
                    PrimitiveConstructionCorpusReplaySiegeError::AcceptedArtifactUnavailable {
                        family: scenario.family,
                        parameter_role: scenario.parameter_role,
                        reason: format!("{error:?}"),
                    }
                })?;
                session
                    .author(scenario.intent.clone())
                    .map_err(|error| {
                        PrimitiveConstructionCorpusReplaySiegeError::AcceptedArtifactUnavailable {
                            family: scenario.family,
                            parameter_role: scenario.parameter_role,
                            reason: error.to_string(),
                        }
                    })?
                    .prepare_result()
                    .map_err(|error| {
                        PrimitiveConstructionCorpusReplaySiegeError::AcceptedArtifactUnavailable {
                            family: scenario.family,
                            parameter_role: scenario.parameter_role,
                            reason: error.to_string(),
                        }
                    })?
            };
            Ok(PrimitiveConstructionCorpusReplaySiegeRow::new(
                scenario.scenario_id.to_string(),
                scenario.family,
                scenario.parameter_role,
                PrimitiveConstructionCorpusOutcomeDisposition::Admitted,
                outcome.outcome_digest().to_string(),
                execution.branch_digest().to_string(),
                execution.replay_digest().to_string(),
                Some(outcome.birth_truth_digest().to_string()),
                Some(outcome.realization_strategy()),
                outcome.attempted_realization_strategies().to_vec(),
                Some(outcome.stability_class()),
                Some(result.feature_conditioning_class()),
                Some(result.support_normal_class()),
                Some(result.normalization_disposition()),
                None,
                None,
                None,
                None,
                construction_breadth(execution.request()).map_err(|reason| {
                    PrimitiveConstructionCorpusReplaySiegeError::AcceptedArtifactUnavailable {
                        family: scenario.family,
                        parameter_role: scenario.parameter_role,
                        reason,
                    }
                })?,
                birth_attachment_breadth(&result),
                certification_breadth(&result),
            ))
        }
        PrimitiveConstructionPreparedOutcome::Rejected(outcome) => {
            let rejection_row =
                rejection_locality_row_for(execution.request().clone()).map_err(|reason| {
                    PrimitiveConstructionCorpusReplaySiegeError::AcceptedArtifactUnavailable {
                        family: scenario.family,
                        parameter_role: scenario.parameter_role,
                        reason,
                    }
                })?;
            Ok(PrimitiveConstructionCorpusReplaySiegeRow::new(
                scenario.scenario_id.to_string(),
                scenario.family,
                scenario.parameter_role,
                PrimitiveConstructionCorpusOutcomeDisposition::Rejected,
                outcome.failure_digest().to_string(),
                execution.branch_digest().to_string(),
                execution.replay_digest().to_string(),
                None,
                None,
                outcome.attempted_realization_strategies().to_vec(),
                outcome.stability_class(),
                outcome
                    .conditioning_witness()
                    .map(|witness| witness.feature_conditioning_class()),
                outcome
                    .conditioning_witness()
                    .map(|witness| witness.support_normal_class()),
                outcome
                    .conditioning_witness()
                    .map(|witness| witness.normalization_disposition()),
                outcome.exhaustion_reason(),
                Some(outcome.rejection_class()),
                Some(rejection_row.rejection_locality()),
                Some(rejection_row.blocking_boundary()),
                0,
                0,
                0,
            ))
        }
    }
}
