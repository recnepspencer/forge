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
