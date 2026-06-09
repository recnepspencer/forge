use crate::construction::tests::support::compound_runtime::{
    PrimitiveConstructionCompoundRowClass, PrimitiveConstructionCompoundScenario,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum PrimitiveConstructionAdversarialAuthoringOrderLane {
    Canonical,
    Reversed,
    RejectedFirst,
    FamilyClustered,
    EscalationClustered,
}

impl PrimitiveConstructionAdversarialAuthoringOrderLane {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Canonical => "canonical",
            Self::Reversed => "reversed",
            Self::RejectedFirst => "rejected_first",
            Self::FamilyClustered => "family_clustered",
            Self::EscalationClustered => "escalation_clustered",
        }
    }

    pub(crate) fn all_compound() -> [Self; 5] {
        [
            Self::Canonical,
            Self::Reversed,
            Self::RejectedFirst,
            Self::FamilyClustered,
            Self::EscalationClustered,
        ]
    }
}

pub(crate) fn required_compound_adversarial_lane_name_set(
) -> std::collections::BTreeSet<&'static str> {
    PrimitiveConstructionAdversarialAuthoringOrderLane::all_compound()
        .into_iter()
        .map(PrimitiveConstructionAdversarialAuthoringOrderLane::as_str)
        .collect()
}

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
                        PrimitiveConstructionCompoundRowClass::StructuredRealizationExhaustion
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
                        PrimitiveConstructionCompoundRowClass::EscalatedStableExactSupport
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
