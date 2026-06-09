use crate::construction::tests::support::corpus_cases::PrimitiveConstructionCorpusScenario;
use crate::construction::tests::support::corpus_replay_row::PrimitiveConstructionCorpusParameterRole;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum PrimitiveConstructionCorpusAuthoringOrderLane {
    Canonical,
    Reversed,
    RejectedFirst,
    RoleClustered,
}

impl PrimitiveConstructionCorpusAuthoringOrderLane {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Canonical => "canonical",
            Self::Reversed => "reversed",
            Self::RejectedFirst => "rejected_first",
            Self::RoleClustered => "role_clustered",
        }
    }

    pub(crate) fn all() -> [Self; 4] {
        [
            Self::Canonical,
            Self::Reversed,
            Self::RejectedFirst,
            Self::RoleClustered,
        ]
    }
}

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
