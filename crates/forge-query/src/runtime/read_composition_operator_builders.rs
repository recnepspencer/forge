use crate::authoring::{
    AspectFieldSelector, CollectionQueryBuilder, DetailQueryBuilder, EqualityPredicate,
    IntegerComparisonPredicate, OrderingSelector, PresencePredicate, SetMembershipPredicate,
    StringContainsPredicate,
};

pub struct DetailReadOperatorQueryBuilder {
    query: DetailQueryBuilder,
}

impl DetailReadOperatorQueryBuilder {
    pub fn project(mut self, entry: AspectFieldSelector) -> Self {
        self.query = self.query.project(entry);
        self
    }

    pub fn where_equal(mut self, predicate: EqualityPredicate) -> Self {
        self.query = self.query.where_equal(predicate);
        self
    }

    pub fn where_greater_than(mut self, predicate: IntegerComparisonPredicate) -> Self {
        self.query = self.query.where_greater_than(predicate);
        self
    }

    pub fn where_less_than(mut self, predicate: IntegerComparisonPredicate) -> Self {
        self.query = self.query.where_less_than(predicate);
        self
    }

    pub fn where_contains(mut self, predicate: StringContainsPredicate) -> Self {
        self.query = self.query.where_contains(predicate);
        self
    }

    pub fn where_in(mut self, predicate: SetMembershipPredicate) -> Self {
        self.query = self.query.where_in(predicate);
        self
    }

    pub fn where_present(mut self, predicate: PresencePredicate) -> Self {
        self.query = self.query.where_present(predicate);
        self
    }

    pub fn order_by(mut self, entry: OrderingSelector) -> Self {
        self.query = self.query.order_by(entry);
        self
    }

    pub(in crate::runtime) fn new(query: DetailQueryBuilder) -> Self {
        Self { query }
    }

    pub(in crate::runtime) fn finish(self) -> DetailQueryBuilder {
        self.query
    }
}

pub struct CollectionReadOperatorQueryBuilder {
    query: CollectionQueryBuilder,
}

impl CollectionReadOperatorQueryBuilder {
    pub fn project(mut self, entry: AspectFieldSelector) -> Self {
        self.query = self.query.project(entry);
        self
    }

    pub fn where_equal(mut self, predicate: EqualityPredicate) -> Self {
        self.query = self.query.where_equal(predicate);
        self
    }

    pub fn where_greater_than(mut self, predicate: IntegerComparisonPredicate) -> Self {
        self.query = self.query.where_greater_than(predicate);
        self
    }

    pub fn where_less_than(mut self, predicate: IntegerComparisonPredicate) -> Self {
        self.query = self.query.where_less_than(predicate);
        self
    }

    pub fn where_contains(mut self, predicate: StringContainsPredicate) -> Self {
        self.query = self.query.where_contains(predicate);
        self
    }

    pub fn where_in(mut self, predicate: SetMembershipPredicate) -> Self {
        self.query = self.query.where_in(predicate);
        self
    }

    pub fn where_present(mut self, predicate: PresencePredicate) -> Self {
        self.query = self.query.where_present(predicate);
        self
    }

    pub fn order_by(mut self, entry: OrderingSelector) -> Self {
        self.query = self.query.order_by(entry);
        self
    }

    pub(in crate::runtime) fn new(query: CollectionQueryBuilder) -> Self {
        Self { query }
    }

    pub(in crate::runtime) fn finish(self) -> CollectionQueryBuilder {
        self.query
    }
}
