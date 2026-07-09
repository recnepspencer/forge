use super::selection_basis::S8DeterministicSelectionRule;
use crate::access_shape::S8AccessShape;
use crate::strategy::S8LayoutStrategyFamily;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8SelectionPolicy;

impl S8SelectionPolicy {
    pub(crate) const fn rule(&self, shape: S8AccessShape) -> S8DeterministicSelectionRule {
        match shape {
            S8AccessShape::PointLookup
            | S8AccessShape::BatchPointLookup
            | S8AccessShape::SortedBatchLookup
            | S8AccessShape::RangeLookup
            | S8AccessShape::MultiRangeLookup
            | S8AccessShape::PrefixLookup
            | S8AccessShape::GroupedPrefixLookup
            | S8AccessShape::CoalescedPageRead => {
                S8DeterministicSelectionRule::OrderedIndexReadsPreferBTree
            }
            _ => S8DeterministicSelectionRule::BufferedOrTraversalReadsPreferLsm,
        }
    }

    pub(crate) const fn rank(&self, family: S8LayoutStrategyFamily, shape: S8AccessShape) -> u8 {
        match self.rule(shape) {
            S8DeterministicSelectionRule::OrderedIndexReadsPreferBTree => match family {
                S8LayoutStrategyFamily::BaselineBTreeRange => 0,
                S8LayoutStrategyFamily::BaselineLsmWriteOptimized => 1,
                _ => 2,
            },
            S8DeterministicSelectionRule::BufferedOrTraversalReadsPreferLsm => match family {
                S8LayoutStrategyFamily::BaselineLsmWriteOptimized => 0,
                S8LayoutStrategyFamily::BaselineBTreeRange => 1,
                _ => 2,
            },
            _ => 0,
        }
    }
}
