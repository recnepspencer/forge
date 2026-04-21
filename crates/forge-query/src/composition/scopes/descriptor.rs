use crate::authoring::{
    AspectFieldSelector, OrderingSelector, PredicateSelector, TraversalSelector,
};

use super::evidence::BasisScopeEvidence;
use crate::composition::ScopeFamily;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QueryScopeDescriptor {
    Predicate(PredicateScopeDescriptor),
    Ordering(OrderingScopeDescriptor),
    Projection(ProjectionScopeDescriptor),
    TraversalBound(TraversalBoundScopeDescriptor),
    BasisAware(BasisAwareScopeDescriptor),
    #[cfg(test)]
    Unsupported(UnsupportedScopeDescriptor),
}

impl QueryScopeDescriptor {
    pub fn predicate(
        label: impl Into<String>,
        predicates: impl IntoIterator<Item = PredicateSelector>,
    ) -> Self {
        Self::Predicate(PredicateScopeDescriptor::new(label, predicates))
    }

    pub fn ordering(
        label: impl Into<String>,
        ordering: impl IntoIterator<Item = OrderingSelector>,
    ) -> Self {
        Self::Ordering(OrderingScopeDescriptor::new(label, ordering))
    }

    pub fn projection(
        label: impl Into<String>,
        projection: impl IntoIterator<Item = AspectFieldSelector>,
    ) -> Self {
        Self::Projection(ProjectionScopeDescriptor::new(label, projection))
    }

    pub fn traversal_bound(
        label: impl Into<String>,
        max_depth: u8,
        traversal: impl IntoIterator<Item = TraversalSelector>,
    ) -> Self {
        Self::TraversalBound(TraversalBoundScopeDescriptor::new(
            label, max_depth, traversal,
        ))
    }

    pub fn basis_aware(label: impl Into<String>, evidence: BasisScopeEvidence) -> Self {
        Self::BasisAware(BasisAwareScopeDescriptor::new(label, evidence))
    }

    #[cfg(test)]
    pub fn unsupported_for_test(label: impl Into<String>) -> Self {
        Self::Unsupported(UnsupportedScopeDescriptor::new(label))
    }

    pub fn family(&self) -> ScopeFamily {
        match self {
            Self::Predicate(_) => ScopeFamily::PredicateScope,
            Self::Ordering(_) => ScopeFamily::OrderingScope,
            Self::Projection(_) => ScopeFamily::ProjectionScope,
            Self::TraversalBound(_) => ScopeFamily::TraversalBoundScope,
            Self::BasisAware(_) => ScopeFamily::BasisAwareScope,
            #[cfg(test)]
            Self::Unsupported(_) => ScopeFamily::UnsupportedScope,
        }
    }

    pub fn label(&self) -> &str {
        match self {
            Self::Predicate(descriptor) => descriptor.label(),
            Self::Ordering(descriptor) => descriptor.label(),
            Self::Projection(descriptor) => descriptor.label(),
            Self::TraversalBound(descriptor) => descriptor.label(),
            Self::BasisAware(descriptor) => descriptor.label(),
            #[cfg(test)]
            Self::Unsupported(descriptor) => descriptor.label(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PredicateScopeDescriptor {
    label: String,
    predicates: Vec<PredicateSelector>,
}

impl PredicateScopeDescriptor {
    fn new(
        label: impl Into<String>,
        predicates: impl IntoIterator<Item = PredicateSelector>,
    ) -> Self {
        Self {
            label: label.into(),
            predicates: predicates.into_iter().collect(),
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn predicates(&self) -> &[PredicateSelector] {
        &self.predicates
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrderingScopeDescriptor {
    label: String,
    ordering: Vec<OrderingSelector>,
}

impl OrderingScopeDescriptor {
    fn new(label: impl Into<String>, ordering: impl IntoIterator<Item = OrderingSelector>) -> Self {
        Self {
            label: label.into(),
            ordering: ordering.into_iter().collect(),
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn ordering(&self) -> &[OrderingSelector] {
        &self.ordering
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionScopeDescriptor {
    label: String,
    projection: Vec<AspectFieldSelector>,
}

impl ProjectionScopeDescriptor {
    fn new(
        label: impl Into<String>,
        projection: impl IntoIterator<Item = AspectFieldSelector>,
    ) -> Self {
        Self {
            label: label.into(),
            projection: projection.into_iter().collect(),
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn projection(&self) -> &[AspectFieldSelector] {
        &self.projection
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TraversalBoundScopeDescriptor {
    label: String,
    max_depth: u8,
    traversal: Vec<TraversalSelector>,
}

impl TraversalBoundScopeDescriptor {
    fn new(
        label: impl Into<String>,
        max_depth: u8,
        traversal: impl IntoIterator<Item = TraversalSelector>,
    ) -> Self {
        Self {
            label: label.into(),
            max_depth,
            traversal: traversal.into_iter().collect(),
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn max_depth(&self) -> u8 {
        self.max_depth
    }

    pub fn traversal(&self) -> &[TraversalSelector] {
        &self.traversal
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisAwareScopeDescriptor {
    label: String,
    evidence: BasisScopeEvidence,
}

impl BasisAwareScopeDescriptor {
    fn new(label: impl Into<String>, evidence: BasisScopeEvidence) -> Self {
        Self {
            label: label.into(),
            evidence,
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn evidence(&self) -> &BasisScopeEvidence {
        &self.evidence
    }
}

#[cfg(test)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnsupportedScopeDescriptor {
    label: String,
}

#[cfg(test)]
impl UnsupportedScopeDescriptor {
    fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }
}
