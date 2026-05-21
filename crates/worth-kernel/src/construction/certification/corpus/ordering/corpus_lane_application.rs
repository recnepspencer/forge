use crate::construction::certification::corpus::cases::PrimitiveConstructionCorpusScenario;
use crate::construction::certification::corpus::replay_siege_report::PrimitiveConstructionCorpusParameterRole;

use super::corpus_lanes::PrimitiveConstructionCorpusAuthoringOrderLane;

pub(crate) fn apply_corpus_authoring_order_lane(
    lane: PrimitiveConstructionCorpusAuthoringOrderLane,
    scenarios: &[PrimitiveConstructionCorpusScenario],
) -> Vec<PrimitiveConstructionCorpusScenario> {
    let mut rows = scenarios.to_vec();
    match lane {
        PrimitiveConstructionCorpusAuthoringOrderLane::Canonical => rows,
        PrimitiveConstructionCorpusAuthoringOrderLane::Reversed => {
            rows.reverse();
            rows
        }
        PrimitiveConstructionCorpusAuthoringOrderLane::RejectedFirst => {
            rows.sort_by_key(|scenario| {
                (
                    !matches!(
                        scenario.parameter_role,
                        PrimitiveConstructionCorpusParameterRole::ThresholdRejected
                            | PrimitiveConstructionCorpusParameterRole::ExplicitRejected
                    ),
                    scenario.family.as_str(),
                    scenario.scenario_id,
                )
            });
            rows
        }
        PrimitiveConstructionCorpusAuthoringOrderLane::RoleClustered => {
            rows.sort_by_key(|scenario| {
                (
                    scenario.parameter_role.as_str(),
                    scenario.family.as_str(),
                    scenario.scenario_id,
                )
            });
            rows
        }
    }
}
