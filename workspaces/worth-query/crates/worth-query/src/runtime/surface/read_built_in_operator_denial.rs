use super::WorthQueryReadBuiltInOperator;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryReadBuiltInOperatorDenialReason {
    EmptyFrontier,
    ZeroDepth,
    DegenerateSuccessorWalkShape,
    DegenerateBoundedWalkShape,
    DuplicateFrontierRelation,
    DegenerateFrontierShape,
    TooFewSharedRelations,
    DuplicateSharedRelation,
    MissingBroadSearchPredicate,
}

impl WorthQueryReadBuiltInOperatorDenialReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyFrontier => "empty_frontier",
            Self::ZeroDepth => "zero_depth",
            Self::DegenerateSuccessorWalkShape => "degenerate_successor_walk_shape",
            Self::DegenerateBoundedWalkShape => "degenerate_bounded_walk_shape",
            Self::DuplicateFrontierRelation => "duplicate_frontier_relation",
            Self::DegenerateFrontierShape => "degenerate_frontier_shape",
            Self::TooFewSharedRelations => "too_few_shared_relations",
            Self::DuplicateSharedRelation => "duplicate_shared_relation",
            Self::MissingBroadSearchPredicate => "missing_broad_search_predicate",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryReadBuiltInOperatorDenial {
    operator: WorthQueryReadBuiltInOperator,
    reason: WorthQueryReadBuiltInOperatorDenialReason,
}

impl WorthQueryReadBuiltInOperatorDenial {
    pub fn operator(&self) -> &WorthQueryReadBuiltInOperator {
        &self.operator
    }

    pub fn reason(&self) -> &WorthQueryReadBuiltInOperatorDenialReason {
        &self.reason
    }

    pub(in crate::runtime) fn new(
        operator: WorthQueryReadBuiltInOperator,
        reason: WorthQueryReadBuiltInOperatorDenialReason,
    ) -> Self {
        Self { operator, reason }
    }
}
