use forge_query::facade::{ForgeQueryRuntimeError, ForgeQueryWorkspace};

use crate::construction::tests::support::corpus_cases::{
    primitive_construction_corpus, PrimitiveConstructionCorpusScenario,
};
use crate::construction::tests::support::corpus_replay_row::PrimitiveConstructionCorpusReplaySiegeRow;
use crate::construction::tests::support::runtime_truth::prepare_primitive_construction_certification_runtime_truth;

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

pub(crate) fn prepare_primitive_construction_corpus_replay_rows(
    workspace: &mut ForgeQueryWorkspace,
) -> Result<
    Vec<PrimitiveConstructionCorpusReplaySiegeRow>,
    PrimitiveConstructionCorpusReplaySiegeError,
> {
    build_corpus_rows(workspace, &primitive_construction_corpus())
}

pub(crate) fn build_corpus_rows(
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
    _workspace: &mut ForgeQueryWorkspace,
    scenario: &PrimitiveConstructionCorpusScenario,
) -> Result<PrimitiveConstructionCorpusReplaySiegeRow, PrimitiveConstructionCorpusReplaySiegeError>
{
    let runtime_truth = prepare_primitive_construction_certification_runtime_truth(
        scenario.intent.clone().into_request(),
    );

    Ok(PrimitiveConstructionCorpusReplaySiegeRow::new(
        scenario.scenario_id.to_string(),
        scenario.family,
        scenario.parameter_role,
        runtime_truth,
    ))
}
