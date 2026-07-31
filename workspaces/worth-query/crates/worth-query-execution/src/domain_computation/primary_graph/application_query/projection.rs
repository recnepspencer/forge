use std::marker::PhantomData;

use worth_query_declaration::facade::application_query::{
    ApplicationQueryResultFieldRef, ApplicationQueryResultRelationCardinality,
    ApplicationQueryResultRelationRef, ApplicationQueryResultTraversal, ExactlyOneResult,
    ManyResults, OptionalOneResult,
};
use worth_query_installation::facade::{
    ApplicationFieldCurrency, TypedApplicationReadableValue, WritePosture,
};

mod projected_tree;

pub(super) use projected_tree::{
    WorthQueryApplicationProjectedField, WorthQueryApplicationProjectedRelation,
    WorthQueryApplicationProjectionNode,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryApplicationProjectionDenialKind {
    FieldNotProjected,
    FieldContractMismatch,
    FieldTypeMismatch,
    RelationNotProjected,
    RelationContractMismatch,
    RelationCardinalityMismatch,
    DomainProjectionRejected,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryApplicationProjectionDenial {
    kind: WorthQueryApplicationProjectionDenialKind,
    subject: String,
}

/// Query-owned authoritative row presented to domain projection code.
///
/// Construction is private. Reads require query-bound result selectors whose
/// slot, path meaning, field or relation contract, and cardinality match the
/// installed application query.
pub struct WorthQueryApplicationProjectionRow<'row, Schema, Query> {
    node: &'row WorthQueryApplicationProjectionNode,
    _marker: PhantomData<fn() -> (Schema, Query)>,
}

pub struct WorthQueryApplicationProjectionRows<'row, Schema, Query> {
    rows: &'row [WorthQueryApplicationProjectionNode],
    _marker: PhantomData<fn() -> (Schema, Query)>,
}

impl WorthQueryApplicationProjectionDenial {
    pub fn reject(subject: impl Into<String>) -> Self {
        Self::new(
            WorthQueryApplicationProjectionDenialKind::DomainProjectionRejected,
            subject,
        )
    }

    pub const fn kind(&self) -> WorthQueryApplicationProjectionDenialKind {
        self.kind
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }

    fn new(kind: WorthQueryApplicationProjectionDenialKind, subject: impl Into<String>) -> Self {
        Self {
            kind,
            subject: subject.into(),
        }
    }
}

impl<'row, Schema, Query> WorthQueryApplicationProjectionRow<'row, Schema, Query> {
    pub fn field<Slot, Entity, Aspect, Field, Value, Write, Equality, Currency>(
        &self,
        selector: ApplicationQueryResultFieldRef<
            Query,
            Slot,
            Schema,
            Entity,
            Aspect,
            Field,
            Value,
            Write,
            Equality,
            Currency,
        >,
    ) -> Result<Value, WorthQueryApplicationProjectionDenial>
    where
        Value: TypedApplicationReadableValue,
        Write: WritePosture,
        Currency: ApplicationFieldCurrency,
        Query: 'static,
        Slot: 'static,
    {
        let projected = self.node.field(selector.slot_type()).ok_or_else(|| {
            projection_denial(
                WorthQueryApplicationProjectionDenialKind::FieldNotProjected,
                selector.slot_type(),
            )
        })?;
        if !projected.matches(&selector) {
            return Err(projection_denial(
                WorthQueryApplicationProjectionDenialKind::FieldContractMismatch,
                projected.result_path(),
            ));
        }
        Value::from_foundational_value(projected.value()).ok_or_else(|| {
            projection_denial(
                WorthQueryApplicationProjectionDenialKind::FieldTypeMismatch,
                projected.result_path(),
            )
        })
    }

    pub fn optional<Slot, Relation, From, To, Direction>(
        &self,
        selector: ApplicationQueryResultRelationRef<
            Query,
            Slot,
            Schema,
            Relation,
            From,
            To,
            Direction,
            OptionalOneResult,
        >,
    ) -> Result<
        Option<WorthQueryApplicationProjectionRow<'_, Schema, Query>>,
        WorthQueryApplicationProjectionDenial,
    >
    where
        Direction: ApplicationQueryResultTraversal,
        Query: 'static,
        Slot: 'static,
    {
        let relation = self.relation(&selector)?;
        match relation.rows() {
            [] => Ok(None),
            [row] => Ok(Some(WorthQueryApplicationProjectionRow::new(row))),
            _ => Err(relation_cardinality_denial(relation)),
        }
    }

    pub fn one<Slot, Relation, From, To, Direction>(
        &self,
        selector: ApplicationQueryResultRelationRef<
            Query,
            Slot,
            Schema,
            Relation,
            From,
            To,
            Direction,
            ExactlyOneResult,
        >,
    ) -> Result<
        WorthQueryApplicationProjectionRow<'_, Schema, Query>,
        WorthQueryApplicationProjectionDenial,
    >
    where
        Direction: ApplicationQueryResultTraversal,
        Query: 'static,
        Slot: 'static,
    {
        let relation = self.relation(&selector)?;
        match relation.rows() {
            [row] => Ok(WorthQueryApplicationProjectionRow::new(row)),
            _ => Err(relation_cardinality_denial(relation)),
        }
    }

    pub fn many<Slot, Relation, From, To, Direction>(
        &self,
        selector: ApplicationQueryResultRelationRef<
            Query,
            Slot,
            Schema,
            Relation,
            From,
            To,
            Direction,
            ManyResults,
        >,
    ) -> Result<
        WorthQueryApplicationProjectionRows<'_, Schema, Query>,
        WorthQueryApplicationProjectionDenial,
    >
    where
        Direction: ApplicationQueryResultTraversal,
        Query: 'static,
        Slot: 'static,
    {
        let relation = self.relation(&selector)?;
        Ok(WorthQueryApplicationProjectionRows {
            rows: relation.rows(),
            _marker: PhantomData,
        })
    }

    fn relation<Slot, Relation, From, To, Direction, Cardinality>(
        &self,
        selector: &ApplicationQueryResultRelationRef<
            Query,
            Slot,
            Schema,
            Relation,
            From,
            To,
            Direction,
            Cardinality,
        >,
    ) -> Result<&WorthQueryApplicationProjectedRelation, WorthQueryApplicationProjectionDenial>
    where
        Direction: ApplicationQueryResultTraversal,
        Cardinality: ApplicationQueryResultRelationCardinality,
        Query: 'static,
        Slot: 'static,
    {
        let projected = self.node.relation(selector.slot_type()).ok_or_else(|| {
            projection_denial(
                WorthQueryApplicationProjectionDenialKind::RelationNotProjected,
                selector.slot_type(),
            )
        })?;
        if projected.matches(selector) {
            Ok(projected)
        } else {
            Err(projection_denial(
                WorthQueryApplicationProjectionDenialKind::RelationContractMismatch,
                projected.result_path(),
            ))
        }
    }

    pub(super) const fn new(node: &'row WorthQueryApplicationProjectionNode) -> Self {
        Self {
            node,
            _marker: PhantomData,
        }
    }
}

impl<'row, Schema, Query> WorthQueryApplicationProjectionRows<'row, Schema, Query> {
    pub const fn len(&self) -> usize {
        self.rows.len()
    }

    pub const fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn iter(
        &self,
    ) -> impl ExactSizeIterator<Item = WorthQueryApplicationProjectionRow<'_, Schema, Query>> {
        self.rows
            .iter()
            .map(WorthQueryApplicationProjectionRow::new)
    }
}

pub trait WorthQueryApplicationProjection<Schema, Query>: Sized {
    fn project(
        row: &WorthQueryApplicationProjectionRow<'_, Schema, Query>,
    ) -> Result<Self, WorthQueryApplicationProjectionDenial>;
}

fn projection_denial(
    kind: WorthQueryApplicationProjectionDenialKind,
    subject: impl Into<String>,
) -> WorthQueryApplicationProjectionDenial {
    WorthQueryApplicationProjectionDenial::new(kind, subject)
}

fn relation_cardinality_denial(
    relation: &WorthQueryApplicationProjectedRelation,
) -> WorthQueryApplicationProjectionDenial {
    projection_denial(
        WorthQueryApplicationProjectionDenialKind::RelationCardinalityMismatch,
        relation.result_path(),
    )
}

impl std::fmt::Display for WorthQueryApplicationProjectionDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "application-query projection denied: {:?} ({})",
            self.kind, self.subject
        )
    }
}

impl std::error::Error for WorthQueryApplicationProjectionDenial {}
