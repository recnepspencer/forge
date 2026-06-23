use std::marker::PhantomData;

use super::{
    AspectFieldSelector, AuthoringError, EqualityPredicate,
    ForgeQueryGraphReadDomainOperationDeclaration, IntegerComparisonPredicate, OrderingSelector,
    PredicateSelector, PresencePredicate, RawAuthoredQuery, RootEntityKey, SetMembershipPredicate,
    StringContainsPredicate, TraversalSelector,
};

pub trait QueryAuthoringFamily: Clone + std::fmt::Debug + Eq + PartialEq {
    fn initialize(root: RootEntityKey) -> RawAuthoredQuery;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthoredQuery<F: QueryAuthoringFamily> {
    raw: RawAuthoredQuery,
    _family: PhantomData<F>,
}

impl<F: QueryAuthoringFamily> AuthoredQuery<F> {
    pub(crate) fn into_raw(self) -> RawAuthoredQuery {
        self.raw
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryBuilder<F: QueryAuthoringFamily> {
    query: RawAuthoredQuery,
    _family: PhantomData<F>,
}

impl<F: QueryAuthoringFamily> QueryBuilder<F> {
    pub fn new(root: RootEntityKey) -> Self {
        Self {
            query: F::initialize(root),
            _family: PhantomData,
        }
    }

    pub fn project(mut self, entry: AspectFieldSelector) -> Self {
        self.query = self.query.with_projection(entry);
        self
    }

    pub fn traverse(mut self, entry: TraversalSelector) -> Self {
        self.query = self.query.with_traversal(entry);
        self
    }

    pub fn domain_graph_operation(
        mut self,
        operation: ForgeQueryGraphReadDomainOperationDeclaration,
    ) -> Self {
        self.query = self.query.with_domain_graph_operation(operation);
        self
    }

    pub fn where_equal(mut self, predicate: EqualityPredicate) -> Self {
        self.query = self
            .query
            .with_predicate(PredicateSelector::Equality(predicate));
        self
    }

    pub fn where_greater_than(mut self, predicate: IntegerComparisonPredicate) -> Self {
        self.query = self
            .query
            .with_predicate(PredicateSelector::IntegerComparison(predicate));
        self
    }

    pub fn where_less_than(mut self, predicate: IntegerComparisonPredicate) -> Self {
        self.query = self
            .query
            .with_predicate(PredicateSelector::IntegerComparison(predicate));
        self
    }

    pub fn where_contains(mut self, predicate: StringContainsPredicate) -> Self {
        self.query = self
            .query
            .with_predicate(PredicateSelector::StringContains(predicate));
        self
    }

    pub fn where_in(mut self, predicate: SetMembershipPredicate) -> Self {
        self.query = self
            .query
            .with_predicate(PredicateSelector::SetMembership(predicate));
        self
    }

    pub fn where_present(mut self, predicate: PresencePredicate) -> Self {
        self.query = self
            .query
            .with_predicate(PredicateSelector::Presence(predicate));
        self
    }

    pub fn order_by(mut self, entry: OrderingSelector) -> Self {
        self.query = self.query.with_ordering(entry);
        self
    }

    pub fn build(self) -> Result<AuthoredQuery<F>, AuthoringError> {
        if self.query.projection().is_empty() {
            return Err(AuthoringError::EmptyProjectionSet);
        }

        Ok(AuthoredQuery {
            raw: self.query,
            _family: PhantomData,
        })
    }
}
