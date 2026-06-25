use super::query_projection::WorthGraphReadRequirementDerivationCapabilityGap;
use super::query_requirement_evidence::WorthGraphReadQueryRequirementSetEvidence;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthGraphReadRequirementDerivationOutcome {
    QueryDerived(WorthGraphReadQueryRequirementSetEvidence),
    QueryCapabilityGap(WorthGraphReadRequirementDerivationCapabilityGap),
}

impl WorthGraphReadRequirementDerivationOutcome {
    pub const fn claims_query_requirement_rows_derived(&self) -> bool {
        matches!(self, Self::QueryDerived(_))
    }

    pub const fn claims_graph_read_execution(&self) -> bool {
        false
    }

    pub const fn claims_access_plan_consumption(&self) -> bool {
        false
    }

    pub fn query_requirement_set_evidence(
        &self,
    ) -> Option<&WorthGraphReadQueryRequirementSetEvidence> {
        match self {
            Self::QueryDerived(evidence) => Some(evidence),
            Self::QueryCapabilityGap(_) => None,
        }
    }

    pub fn capability_gap(&self) -> Option<&WorthGraphReadRequirementDerivationCapabilityGap> {
        match self {
            Self::QueryDerived(_) => None,
            Self::QueryCapabilityGap(gap) => Some(gap),
        }
    }
}
