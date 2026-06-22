use forge_query::facade::{ForgeQueryRuntimeError, ForgeQueryWorkspace};

use crate::construction::certification::corpus::lane_execution::branch_local::{
    prepare_branch_local_lane, PrimitiveConstructionCorpusBranchLocalLaneError,
};
use crate::construction::certification::corpus::lane_execution::current_head::prepare_current_head_lane;
use crate::construction::certification::corpus::lane_execution::replay::prepare_replay_lane;
use crate::construction::intent::PrimitiveConstructionIntent;
use crate::construction::tests::support::corpus_cases::{
    primitive_construction_corpus, PrimitiveConstructionCorpusScenario,
};
use crate::construction::tests::support::corpus_replay_row::PrimitiveConstructionCorpusReplaySiegeRow;

#[derive(Debug)]
pub(crate) enum PrimitiveConstructionCorpusReplaySiegeError {
    QueryRuntime(ForgeQueryRuntimeError),
}

impl std::fmt::Display for PrimitiveConstructionCorpusReplaySiegeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::QueryRuntime(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionCorpusReplaySiegeError {}

impl From<ForgeQueryRuntimeError> for PrimitiveConstructionCorpusReplaySiegeError {
    fn from(error: ForgeQueryRuntimeError) -> Self {
        Self::QueryRuntime(error)
    }
}

impl From<PrimitiveConstructionCorpusBranchLocalLaneError>
    for PrimitiveConstructionCorpusReplaySiegeError
{
    fn from(error: PrimitiveConstructionCorpusBranchLocalLaneError) -> Self {
        match error {
            PrimitiveConstructionCorpusBranchLocalLaneError::QueryRuntime(error) => {
                Self::QueryRuntime(error)
            }
        }
    }
}

pub(crate) fn prepare_primitive_construction_corpus_replay_rows(
    workspace: &mut ForgeQueryWorkspace,
) -> Result<
    Vec<PrimitiveConstructionCorpusReplaySiegeRow>,
    PrimitiveConstructionCorpusReplaySiegeError,
> {
    build_corpus_rows(workspace, "canonical", &primitive_construction_corpus())
}

pub(crate) fn build_corpus_rows(
    workspace: &mut ForgeQueryWorkspace,
    lane_label: &str,
    scenarios: &[PrimitiveConstructionCorpusScenario],
) -> Result<
    Vec<PrimitiveConstructionCorpusReplaySiegeRow>,
    PrimitiveConstructionCorpusReplaySiegeError,
> {
    let mut rows = Vec::new();
    for scenario in scenarios {
        let row = build_corpus_row(workspace, lane_label, scenario)?;
        rows.push(row);
    }
    Ok(rows)
}

fn build_corpus_row(
    workspace: &mut ForgeQueryWorkspace,
    lane_label: &str,
    scenario: &PrimitiveConstructionCorpusScenario,
) -> Result<PrimitiveConstructionCorpusReplaySiegeRow, PrimitiveConstructionCorpusReplaySiegeError>
{
    let intent = PrimitiveConstructionIntent::from(scenario.intent.clone().into_request());
    let current_head_lane = prepare_current_head_lane(intent.clone());
    let branch_local_lane =
        prepare_branch_local_lane(workspace, lane_label, &scenario.scenario_id, &intent)?;
    let replay_lane = prepare_replay_lane(&intent);

    Ok(PrimitiveConstructionCorpusReplaySiegeRow::new(
        scenario.scenario_id.to_string(),
        scenario.family,
        scenario.parameter_role,
        current_head_lane,
        branch_local_lane,
        replay_lane,
    ))
}
