use super::ForgeQueryReadBuiltInOperator;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryReadBuiltInOperatorDenialReason {
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

impl ForgeQueryReadBuiltInOperatorDenialReason {
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
pub struct ForgeQueryReadBuiltInOperatorDenial {
    operator: ForgeQueryReadBuiltInOperator,
    reason: ForgeQueryReadBuiltInOperatorDenialReason,
}

impl ForgeQueryReadBuiltInOperatorDenial {
    pub fn operator(&self) -> &ForgeQueryReadBuiltInOperator {
        &self.operator
    }

    pub fn reason(&self) -> &ForgeQueryReadBuiltInOperatorDenialReason {
        &self.reason
    }

    pub(in crate::runtime) fn new(
        operator: ForgeQueryReadBuiltInOperator,
        reason: ForgeQueryReadBuiltInOperatorDenialReason,
    ) -> Self {
        Self { operator, reason }
    }
}
