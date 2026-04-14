use std::marker::PhantomData;

use crate::authoring::{
    AuthoringError, EqualityPredicate, IntegerComparisonPredicate, OrderingSelector,
    PresencePredicate, QueryAuthoringFamily, QueryBuilder, RootEntityKey, SetMembershipPredicate,
    StringContainsPredicate, TraversalSelector,
};

use super::traits::{
    TypedEqualityField, TypedIntegerComparableField, TypedMembershipField, TypedOrderableField,
    TypedPresenceField, TypedProjectableField, TypedSchemaRoot, TypedStringContainsField,
    TypedTraversalRelation,
};
use super::utils::selector_for;

pub trait TypedQuerySurfaceFamily: Clone + std::fmt::Debug + Eq + PartialEq {
    type AuthoringFamily: QueryAuthoringFamily;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypedAuthoredQuery<S: TypedSchemaRoot, F: TypedQuerySurfaceFamily> {
    pub(super) inner: crate::authoring::AuthoredQuery<F::AuthoringFamily>,
    pub(super) _schema: PhantomData<S>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypedQueryBuilder<S: TypedSchemaRoot, F: TypedQuerySurfaceFamily> {
    inner: QueryBuilder<F::AuthoringFamily>,
    _schema: PhantomData<S>,
}

impl<S: TypedSchemaRoot, F: TypedQuerySurfaceFamily> TypedQueryBuilder<S, F> {
    pub fn new() -> Self {
        Self {
            inner: QueryBuilder::new(
                RootEntityKey::new(S::ROOT_ENTITY)
                    .expect("typed schema root entity must be a valid non-empty constant"),
            ),
            _schema: PhantomData,
        }
    }

    pub fn project<P: TypedProjectableField<Schema = S>>(mut self) -> Self {
        self.inner = self.inner.project(selector_for::<P>());
        self
    }

    pub fn traverse<R: TypedTraversalRelation<Schema = S>>(
        mut self,
        depth: u8,
    ) -> Result<Self, AuthoringError> {
        self.inner = self
            .inner
            .traverse(TraversalSelector::bounded(R::RELATION, depth)?);
        Ok(self)
    }

    pub fn where_equal<P: TypedEqualityField<Schema = S>>(mut self, value: P::Value) -> Self {
        self.inner = self.inner.where_equal(
            EqualityPredicate::new(P::ASPECT, P::FIELD, P::into_scalar(value))
                .expect("typed equality predicate constants must be valid"),
        );
        self
    }

    pub fn where_greater_than<P: TypedIntegerComparableField<Schema = S>>(
        mut self,
        value: i64,
    ) -> Self {
        self.inner = self.inner.where_greater_than(
            IntegerComparisonPredicate::greater_than(P::ASPECT, P::FIELD, value)
                .expect("typed integer predicate constants must be valid"),
        );
        self
    }

    pub fn where_less_than<P: TypedIntegerComparableField<Schema = S>>(
        mut self,
        value: i64,
    ) -> Self {
        self.inner = self.inner.where_less_than(
            IntegerComparisonPredicate::less_than(P::ASPECT, P::FIELD, value)
                .expect("typed integer predicate constants must be valid"),
        );
        self
    }

    pub fn where_contains<P: TypedStringContainsField<Schema = S>>(
        mut self,
        value: impl Into<String>,
    ) -> Self {
        self.inner = self.inner.where_contains(
            StringContainsPredicate::new(P::ASPECT, P::FIELD, value)
                .expect("typed contains predicate constants must be valid"),
        );
        self
    }

    pub fn where_in<P, I>(mut self, values: I) -> Self
    where
        P: TypedMembershipField<Schema = S>,
        I: IntoIterator<Item = P::Value>,
    {
        self.inner = self.inner.where_in(
            SetMembershipPredicate::new(
                P::ASPECT,
                P::FIELD,
                values.into_iter().map(P::into_scalar),
            )
            .expect("typed membership predicate constants must be valid"),
        );
        self
    }

    pub fn where_present<P: TypedPresenceField<Schema = S>>(mut self) -> Self {
        self.inner = self.inner.where_present(
            PresencePredicate::is_present(P::ASPECT, P::FIELD)
                .expect("typed presence predicate constants must be valid"),
        );
        self
    }

    pub fn order_by_ascending<P: TypedOrderableField<Schema = S>>(mut self) -> Self {
        self.inner = self.inner.order_by(
            OrderingSelector::ascending(P::ASPECT, P::FIELD)
                .expect("typed ordering constants must be valid"),
        );
        self
    }

    pub fn order_by_descending<P: TypedOrderableField<Schema = S>>(mut self) -> Self {
        self.inner = self.inner.order_by(
            OrderingSelector::descending(P::ASPECT, P::FIELD)
                .expect("typed ordering constants must be valid"),
        );
        self
    }

    pub fn build(self) -> Result<TypedAuthoredQuery<S, F>, AuthoringError> {
        Ok(TypedAuthoredQuery {
            inner: self.inner.build()?,
            _schema: PhantomData,
        })
    }
}

impl<S: TypedSchemaRoot, F: TypedQuerySurfaceFamily> Default for TypedQueryBuilder<S, F> {
    fn default() -> Self {
        Self::new()
    }
}
