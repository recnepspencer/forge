use crate::construction::certification::corpus::compound::{
    PrimitiveConstructionCompoundRowClass, PrimitiveConstructionCompoundScenario,
};

use super::compound_lanes::PrimitiveConstructionAdversarialAuthoringOrderLane;

pub(crate) fn apply_compound_authoring_order_lane(
    lane: PrimitiveConstructionAdversarialAuthoringOrderLane,
    scenarios: &[PrimitiveConstructionCompoundScenario],
) -> Vec<PrimitiveConstructionCompoundScenario> {
    let mut rows = scenarios.to_vec();
    match lane {
        PrimitiveConstructionAdversarialAuthoringOrderLane::Canonical => rows,
        PrimitiveConstructionAdversarialAuthoringOrderLane::Reversed => {
            rows.reverse();
            rows
        }
        PrimitiveConstructionAdversarialAuthoringOrderLane::RejectedFirst => {
            rows.sort_by_key(|scenario: &PrimitiveConstructionCompoundScenario| {
                (
                    !matches!(
                        scenario.row_class(),
                        PrimitiveConstructionCompoundRowClass::StructuredAdmissionRejection
                            | PrimitiveConstructionCompoundRowClass::StructuredRealizationExhaustion
                            | PrimitiveConstructionCompoundRowClass::BoundaryDriftGuardCase
                    ),
                    scenario.scenario_id(),
                )
            });
            rows
        }
        PrimitiveConstructionAdversarialAuthoringOrderLane::FamilyClustered => {
            rows.sort_by_key(|scenario: &PrimitiveConstructionCompoundScenario| {
                (scenario.workload_family() as u8, scenario.scenario_id())
            });
            rows
        }
        PrimitiveConstructionAdversarialAuthoringOrderLane::EscalationClustered => {
            rows.sort_by_key(|scenario: &PrimitiveConstructionCompoundScenario| {
                (
                    !matches!(
                        scenario.row_class(),
                        PrimitiveConstructionCompoundRowClass::EscalatedStableLocalNormalized
                            | PrimitiveConstructionCompoundRowClass::EscalatedStableExactSupport
                            | PrimitiveConstructionCompoundRowClass::StructuredRealizationExhaustion
                    ),
                    scenario.row_class() as u8,
                    scenario.scenario_id(),
                )
            });
            rows
        }
    }
}
