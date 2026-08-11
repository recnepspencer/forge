#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlanningFailureClass {
    UnsupportedPlanShape,
    IncompletePlanningInputs,
    InternalInvariantBreak,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlanningError {
    MissingBindingResolutionForIdentityBoundQuery,
    UnsupportedBackendParityRequest,
    UnsupportedFallbackShape,
    UnsupportedOrderingFamily,
    UnsupportedCursorShape,
    UnsupportedTraversalBound,
    UnsupportedAggregateFamily,
    UnsupportedCollectionResultFamily,
    BindingResolutionFailed { failure_digest: String },
    PlanningInvariantViolation { message: &'static str },
}

impl PlanningError {
    pub fn failure_class(&self) -> PlanningFailureClass {
        match self {
            Self::MissingBindingResolutionForIdentityBoundQuery => {
                PlanningFailureClass::IncompletePlanningInputs
            }
            Self::UnsupportedBackendParityRequest
            | Self::UnsupportedFallbackShape
            | Self::UnsupportedOrderingFamily
            | Self::UnsupportedCursorShape
            | Self::UnsupportedTraversalBound
            | Self::UnsupportedAggregateFamily
            | Self::UnsupportedCollectionResultFamily => PlanningFailureClass::UnsupportedPlanShape,
            Self::BindingResolutionFailed { .. } => PlanningFailureClass::IncompletePlanningInputs,
            Self::PlanningInvariantViolation { .. } => PlanningFailureClass::InternalInvariantBreak,
        }
    }
}
