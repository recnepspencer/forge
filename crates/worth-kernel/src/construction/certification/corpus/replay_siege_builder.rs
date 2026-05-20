use forge_query::facade::ForgeQueryWorkspace;

use super::cases::PrimitiveConstructionCorpusScenario;
use super::replay_siege::PrimitiveConstructionCorpusReplaySiegeError;
use super::replay_siege_report::{
    PrimitiveConstructionCorpusOutcomeDisposition, PrimitiveConstructionCorpusReplaySiegeRow,
};
use super::row_support::{
    birth_attachment_breadth, certification_breadth, construction_breadth,
    rejection_locality_row_for,
};
use crate::construction::outcome::PrimitiveConstructionPreparedOutcome;
use crate::construction::parity::{
    prepare_primitive_construction_branch_local_parity_report,
    prepare_primitive_construction_replay_parity_report,
};
use crate::construction::result::{
    prepare_primitive_construction_result, PrimitiveConstructionResultError,
};

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

pub(super) fn normalized_row_digests(
    rows: &[PrimitiveConstructionCorpusReplaySiegeRow],
) -> Vec<String> {
    let mut digests = rows
        .iter()
        .map(|row| format!("{}:{}", row.scenario_id(), row.row_digest()))
        .collect::<Vec<_>>();
    digests.sort();
    digests
}

fn build_corpus_row(
    workspace: &mut ForgeQueryWorkspace,
    scenario: &PrimitiveConstructionCorpusScenario,
) -> Result<PrimitiveConstructionCorpusReplaySiegeRow, PrimitiveConstructionCorpusReplaySiegeError>
{
    let request = scenario.intent.request().clone();
    let replay = prepare_primitive_construction_replay_parity_report(scenario.intent.clone());
    if !replay.parity_verified() {
        return Err(
            PrimitiveConstructionCorpusReplaySiegeError::ReplayParityDrift {
                family: scenario.family,
                parameter_role: scenario.parameter_role,
            },
        );
    }
    let branch = prepare_primitive_construction_branch_local_parity_report(
        workspace,
        scenario.intent.clone(),
    )
    .map_err(PrimitiveConstructionCorpusReplaySiegeError::RuntimeBasis)?;
    if !branch.parity_verified() {
        return Err(
            PrimitiveConstructionCorpusReplaySiegeError::BranchLocalParityDrift {
                family: scenario.family,
                parameter_role: scenario.parameter_role,
            },
        );
    }

    let direct_outcome = replay.direct_outcome().clone();
    let replay_digest = replay.replay_outcome().outcome_digest().to_string();
    let branch_digest = branch
        .branch_preview_runtime_report()
        .outcome()
        .outcome_digest()
        .to_string();

    match direct_outcome {
        PrimitiveConstructionPreparedOutcome::Accepted(outcome) => {
            let result = prepare_primitive_construction_result(scenario.intent.clone()).map_err(
                |error| PrimitiveConstructionCorpusReplaySiegeError::AcceptedArtifactUnavailable {
                    family: scenario.family,
                    parameter_role: scenario.parameter_role,
                    reason: result_error_reason(&error),
                },
            )?;
            Ok(PrimitiveConstructionCorpusReplaySiegeRow::new(
                scenario.scenario_id.to_string(),
                scenario.family,
                scenario.parameter_role,
                PrimitiveConstructionCorpusOutcomeDisposition::Admitted,
                outcome.outcome_digest().to_string(),
                branch_digest,
                replay_digest,
                Some(outcome.birth_truth_digest().to_string()),
                Some(outcome.realization_strategy()),
                outcome.attempted_realization_strategies().to_vec(),
                Some(outcome.stability_class()),
                Some(result.canonical_artifact().feature_conditioning_class()),
                Some(result.canonical_artifact().support_normal_class()),
                Some(result.canonical_artifact().normalization_disposition()),
                None,
                None,
                None,
                None,
                construction_breadth(&request).map_err(|reason| {
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
            let rejection_row = rejection_locality_row_for(request).map_err(|reason| {
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
                branch_digest,
                replay_digest,
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

fn result_error_reason(error: &PrimitiveConstructionResultError) -> String {
    error.to_string()
}
