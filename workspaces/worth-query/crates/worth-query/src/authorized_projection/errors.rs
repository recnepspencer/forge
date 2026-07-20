use super::AuthorizedProjectionCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum AuthorizedProjectionFailureClass {
    MaskedProjectionRequested,
    MaskedPredicateInfluence,
    MaskedOrderingInfluence,
    MaskedGroupingInfluence,
    MaskedDerivedFieldInfluence,
    MaskedAggregationInfluence,
    MaskedCursorInfluence,
    MaskedViewMembershipInfluence,
    NonDisclosingUseWouldBeEmitted,
    UnknownAspectMask,
    ProjectionBudgetExceeded,
    MaskBudgetExceeded,
}

impl AuthorizedProjectionFailureClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MaskedProjectionRequested => "masked_projection_requested",
            Self::MaskedPredicateInfluence => "masked_predicate_influence",
            Self::MaskedOrderingInfluence => "masked_ordering_influence",
            Self::MaskedGroupingInfluence => "masked_grouping_influence",
            Self::MaskedDerivedFieldInfluence => "masked_derived_field_influence",
            Self::MaskedAggregationInfluence => "masked_aggregation_influence",
            Self::MaskedCursorInfluence => "masked_cursor_influence",
            Self::MaskedViewMembershipInfluence => "masked_view_membership_influence",
            Self::NonDisclosingUseWouldBeEmitted => "non_disclosing_use_would_be_emitted",
            Self::UnknownAspectMask => "unknown_aspect_mask",
            Self::ProjectionBudgetExceeded => "projection_budget_exceeded",
            Self::MaskBudgetExceeded => "mask_budget_exceeded",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorizedProjectionError {
    failure_class: AuthorizedProjectionFailureClass,
    message: &'static str,
    counters: AuthorizedProjectionCounters,
}

impl AuthorizedProjectionError {
    pub(crate) fn new(
        failure_class: AuthorizedProjectionFailureClass,
        message: &'static str,
        counters: AuthorizedProjectionCounters,
    ) -> Self {
        Self {
            failure_class,
            message,
            counters,
        }
    }

    pub fn failure_class(&self) -> AuthorizedProjectionFailureClass {
        self.failure_class
    }

    pub fn message(&self) -> &'static str {
        self.message
    }

    pub fn counters(&self) -> &AuthorizedProjectionCounters {
        &self.counters
    }
}
