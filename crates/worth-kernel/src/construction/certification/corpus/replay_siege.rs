use forge_query::facade::ForgeQueryWorkspace;

use super::cases::primitive_construction_corpus;
use super::replay_siege_builder::{build_corpus_rows, normalized_row_digests};
use crate::construction::certification::corpus::replay_siege_report::{
    PrimitiveConstructionCorpusAuthoringOrderRow, PrimitiveConstructionCorpusOutcomeDisposition,
    PrimitiveConstructionCorpusParameterRole, PrimitiveConstructionCorpusReplaySiegeReport,
};
use crate::construction::digest::digest_owned_parts;
use crate::construction::request::PrimitiveConstructionFamily;
use crate::construction::runtime_basis::PrimitiveConstructionRuntimeBasisError;

#[derive(Debug)]
pub enum PrimitiveConstructionCorpusReplaySiegeError {
    RuntimeBasis(PrimitiveConstructionRuntimeBasisError),
    AcceptedArtifactUnavailable {
        family: PrimitiveConstructionFamily,
        parameter_role: PrimitiveConstructionCorpusParameterRole,
        reason: String,
    },
    ReplayParityDrift {
        family: PrimitiveConstructionFamily,
        parameter_role: PrimitiveConstructionCorpusParameterRole,
    },
    BranchLocalParityDrift {
        family: PrimitiveConstructionFamily,
        parameter_role: PrimitiveConstructionCorpusParameterRole,
    },
}

impl std::fmt::Display for PrimitiveConstructionCorpusReplaySiegeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RuntimeBasis(error) => write!(f, "{error}"),
            Self::AcceptedArtifactUnavailable {
                family,
                parameter_role,
                reason,
            } => write!(
                f,
                "accepted corpus case {}:{} could not materialize its prepared result: {reason}",
                family.as_str(),
                parameter_role.as_str()
            ),
            Self::ReplayParityDrift {
                family,
                parameter_role,
            } => write!(
                f,
                "replay parity drift detected for {}:{}",
                family.as_str(),
                parameter_role.as_str()
            ),
            Self::BranchLocalParityDrift {
                family,
                parameter_role,
            } => write!(
                f,
                "branch-local parity drift detected for {}:{}",
                family.as_str(),
                parameter_role.as_str()
            ),
        }
    }
}

impl std::error::Error for PrimitiveConstructionCorpusReplaySiegeError {}

pub fn prepare_primitive_construction_corpus_replay_siege(
    workspace: &mut ForgeQueryWorkspace,
) -> Result<PrimitiveConstructionCorpusReplaySiegeReport, PrimitiveConstructionCorpusReplaySiegeError>
{
    let scenarios = primitive_construction_corpus();
    let rows = build_corpus_rows(workspace, &scenarios)?;
    let canonical_digest = digest_owned_parts(&normalized_row_digests(&rows));
    let order_lanes = build_authoring_order_rows(workspace, &scenarios, &canonical_digest)?;
    let accepted_count = rows
        .iter()
        .filter(|row| {
            row.outcome_disposition() == PrimitiveConstructionCorpusOutcomeDisposition::Admitted
        })
        .count();
    let rejected_count = rows.len() - accepted_count;
    let rejection_witness_rows =
        super::rejection_witnesses::primitive_construction_rejection_witness_rows();
    Ok(PrimitiveConstructionCorpusReplaySiegeReport::new(
        rows,
        accepted_count,
        rejected_count,
        order_lanes,
        rejection_witness_rows,
    ))
}

fn build_authoring_order_rows(
    workspace: &mut ForgeQueryWorkspace,
    scenarios: &[super::cases::PrimitiveConstructionCorpusScenario],
    canonical_digest: &str,
) -> Result<
    Vec<PrimitiveConstructionCorpusAuthoringOrderRow>,
    PrimitiveConstructionCorpusReplaySiegeError,
> {
    let mut reversed = scenarios.to_vec();
    reversed.reverse();
    let mut rejected_first = scenarios
        .iter()
        .filter(|scenario| {
            matches!(
                scenario.parameter_role,
                PrimitiveConstructionCorpusParameterRole::ThresholdRejected
                    | PrimitiveConstructionCorpusParameterRole::ExplicitRejected
            )
        })
        .cloned()
        .collect::<Vec<_>>();
    rejected_first.extend(
        scenarios
            .iter()
            .filter(|scenario| {
                !matches!(
                    scenario.parameter_role,
                    PrimitiveConstructionCorpusParameterRole::ThresholdRejected
                        | PrimitiveConstructionCorpusParameterRole::ExplicitRejected
                )
            })
            .cloned(),
    );
    let mut role_clustered = scenarios.to_vec();
    role_clustered.sort_by_key(|scenario| {
        (
            scenario.parameter_role.as_str(),
            scenario.family.as_str(),
            scenario.scenario_id,
        )
    });
    let lanes = vec![
        ("canonical", scenarios.to_vec()),
        ("reversed", reversed),
        ("rejected_first", rejected_first),
        ("role_clustered", role_clustered),
    ];

    let mut rows = Vec::new();
    for (lane_name, lane_scenarios) in lanes {
        let lane_rows = build_corpus_rows(workspace, &lane_scenarios)?;
        let normalized_digest = digest_owned_parts(&normalized_row_digests(&lane_rows));
        rows.push(PrimitiveConstructionCorpusAuthoringOrderRow::new(
            lane_name.to_string(),
            super::ordering::lane_digest(&lane_rows),
            normalized_digest.clone(),
            lane_rows.len(),
            normalized_digest == canonical_digest,
        ));
    }
    Ok(rows)
}
