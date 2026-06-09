use super::builder::PrimitiveConstructionCompoundAdversarialSiegeError;
use super::cases::PrimitiveConstructionCompoundScenario;
use super::rows::PrimitiveConstructionCompoundRow;
use crate::construction::tests::support::runtime_truth::prepare_primitive_construction_certification_runtime_truth;

pub(super) fn build_rows_for_lane(
    scenarios: &[PrimitiveConstructionCompoundScenario],
) -> Result<Vec<PrimitiveConstructionCompoundRow>, PrimitiveConstructionCompoundAdversarialSiegeError>
{
    scenarios
        .iter()
        .map(build_row)
        .collect::<Result<Vec<_>, _>>()
}

fn build_row(
    scenario: &PrimitiveConstructionCompoundScenario,
) -> Result<PrimitiveConstructionCompoundRow, PrimitiveConstructionCompoundAdversarialSiegeError> {
    let intent = scenario
        .resolved_intent()
        .map_err(PrimitiveConstructionCompoundAdversarialSiegeError::Motion)?;
    let runtime_truth =
        prepare_primitive_construction_certification_runtime_truth(intent.clone().into_request());
    Ok(PrimitiveConstructionCompoundRow::new(
        scenario.scenario_id.to_string(),
        scenario.workload_family,
        scenario.topology_class,
        scenario.row_class,
        runtime_truth,
    ))
}
