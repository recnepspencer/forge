use forge_query::facade::ForgeQueryWorkspace;

use super::cases::primitive_construction_corpus;
use super::ordering::{
    apply_corpus_authoring_order_lane, lane_digest, normalized_matrix_digest,
    PrimitiveConstructionCorpusAuthoringOrderLane,
};
use super::replay_siege_builder::build_corpus_rows;
use crate::construction::certification::corpus::replay_siege_report::{
    PrimitiveConstructionCorpusAuthoringOrderRow, PrimitiveConstructionCorpusOutcomeDisposition,
    PrimitiveConstructionCorpusParameterRole, PrimitiveConstructionCorpusReplaySiegeReport,
};
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

impl From<PrimitiveConstructionRuntimeBasisError> for PrimitiveConstructionCorpusReplaySiegeError {
    fn from(error: PrimitiveConstructionRuntimeBasisError) -> Self {
        Self::RuntimeBasis(error)
    }
}

pub fn prepare_primitive_construction_corpus_replay_siege(
    workspace: &mut ForgeQueryWorkspace,
) -> Result<PrimitiveConstructionCorpusReplaySiegeReport, PrimitiveConstructionCorpusReplaySiegeError>
{
    let scenarios = primitive_construction_corpus();
    let rows = build_corpus_rows(workspace, &scenarios)?;
    let canonical_digest = normalized_matrix_digest(
        rows.iter()
            .map(|row| (row.scenario_id().to_string(), row.row_digest().to_string())),
    );
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
    let mut rows = Vec::new();
    for lane in PrimitiveConstructionCorpusAuthoringOrderLane::all() {
        let lane_scenarios = apply_corpus_authoring_order_lane(lane, scenarios);
        let lane_rows = build_corpus_rows(workspace, &lane_scenarios)?;
        let normalized_digest = normalized_matrix_digest(
            lane_rows
                .iter()
                .map(|row| (row.scenario_id().to_string(), row.row_digest().to_string())),
        );
        rows.push(PrimitiveConstructionCorpusAuthoringOrderRow::new(
            lane.as_str().to_string(),
            lane_digest(lane_rows.iter().map(|row| row.row_digest().to_string())),
            normalized_digest.clone(),
            lane_rows.len(),
            normalized_digest == canonical_digest,
        ));
    }
    Ok(rows)
}
