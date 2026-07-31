use worth_foundational::facade::AspectValue;
use worth_query_declaration::facade::application_query::{
    ApplicationQueryResultFieldRef, ApplicationQueryResultRelationCardinality,
    ApplicationQueryResultRelationRef, ApplicationQueryResultTraversal, ExactlyOneResult,
    ManyResults, OptionalOneResult,
};
use worth_query_installation::facade::{
    ApplicationFieldCurrency, TypedApplicationReadableValue, WritePosture,
};

use super::{
    projection_denial, relation_cardinality_denial, WorthQueryApplicationProjectedRelation,
    WorthQueryApplicationProjectionDenial, WorthQueryApplicationProjectionDenialKind,
    WorthQueryApplicationProjectionRow, WorthQueryApplicationProjectionRows,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryApplicationOmission {
    classification: String,
    required_disclosure: AspectValue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryApplicationDisclosed<Value> {
    Disclosed(Value),
    Omitted(WorthQueryApplicationOmission),
}

impl WorthQueryApplicationOmission {
    pub fn classification(&self) -> &str {
        &self.classification
    }

    pub const fn required_disclosure(&self) -> &AspectValue {
        &self.required_disclosure
    }
}

impl<Value> WorthQueryApplicationDisclosed<Value> {
    pub(super) fn into_required(
        self,
        kind: WorthQueryApplicationProjectionDenialKind,
    ) -> Result<Value, WorthQueryApplicationProjectionDenial> {
        match self {
            Self::Disclosed(value) => Ok(value),
            Self::Omitted(omission) => {
                Err(projection_denial(kind, omission.classification))
            }
        }
    }
}

impl<'row, Schema, Query> WorthQueryApplicationProjectionRow<'row, Schema, Query> {
    pub fn disclosed_field<Slot, Entity, Aspect, Field, Value, Write, Equality, Currency>(
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
    ) -> Result<
        WorthQueryApplicationDisclosed<Value>,
        WorthQueryApplicationProjectionDenial,
    >
    where
        Value: TypedApplicationReadableValue,
        Write: WritePosture,
        Currency: ApplicationFieldCurrency,
        Query: 'static,
        Slot: 'static,
    {
        let slot = selector.slot_key();
        if let Some(omission) = self.omission(&slot) {
            return Ok(WorthQueryApplicationDisclosed::Omitted(omission));
        }
        if !self.governance.is_disclosed(&slot) {
            return Err(projection_denial(
                WorthQueryApplicationProjectionDenialKind::FieldContractMismatch,
                selector.slot_type(),
            ));
        }
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
        let value = Value::from_foundational_value(projected.value()).ok_or_else(|| {
            projection_denial(
                WorthQueryApplicationProjectionDenialKind::FieldTypeMismatch,
                projected.result_path(),
            )
        })?;
        Ok(WorthQueryApplicationDisclosed::Disclosed(value))
    }

    pub fn disclosed_optional<Slot, Relation, From, To, Direction>(
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
        WorthQueryApplicationDisclosed<Option<WorthQueryApplicationProjectionRow<'_, Schema, Query>>>,
        WorthQueryApplicationProjectionDenial,
    >
    where
        Direction: ApplicationQueryResultTraversal,
        Query: 'static,
        Slot: 'static,
    {
        self.disclosed_relation(&selector).and_then(|disclosure| {
            disclosure.map(|relation| match relation.rows() {
                [] => Ok(None),
                [row] => Ok(Some(WorthQueryApplicationProjectionRow::new(
                    row,
                    self.governance,
                ))),
                _ => Err(relation_cardinality_denial(relation)),
            })
        })
    }

    pub fn disclosed_one<Slot, Relation, From, To, Direction>(
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
        WorthQueryApplicationDisclosed<WorthQueryApplicationProjectionRow<'_, Schema, Query>>,
        WorthQueryApplicationProjectionDenial,
    >
    where
        Direction: ApplicationQueryResultTraversal,
        Query: 'static,
        Slot: 'static,
    {
        self.disclosed_relation(&selector).and_then(|disclosure| {
            disclosure.map(|relation| match relation.rows() {
                [row] => Ok(WorthQueryApplicationProjectionRow::new(
                    row,
                    self.governance,
                )),
                _ => Err(relation_cardinality_denial(relation)),
            })
        })
    }

    pub fn disclosed_many<Slot, Relation, From, To, Direction>(
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
        WorthQueryApplicationDisclosed<WorthQueryApplicationProjectionRows<'_, Schema, Query>>,
        WorthQueryApplicationProjectionDenial,
    >
    where
        Direction: ApplicationQueryResultTraversal,
        Query: 'static,
        Slot: 'static,
    {
        self.disclosed_relation(&selector).and_then(|disclosure| {
            disclosure.map(|relation| {
                Ok(WorthQueryApplicationProjectionRows {
                    rows: relation.rows(),
                    governance: self.governance,
                    _marker: std::marker::PhantomData,
                })
            })
        })
    }

    pub(super) fn disclosed_relation<Slot, Relation, From, To, Direction, Cardinality>(
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
    ) -> Result<
        WorthQueryApplicationDisclosed<&WorthQueryApplicationProjectedRelation>,
        WorthQueryApplicationProjectionDenial,
    >
    where
        Direction: ApplicationQueryResultTraversal,
        Cardinality: ApplicationQueryResultRelationCardinality,
        Query: 'static,
        Slot: 'static,
    {
        let slot = selector.slot_key();
        if let Some(omission) = self.omission(&slot) {
            return Ok(WorthQueryApplicationDisclosed::Omitted(omission));
        }
        if !self.governance.is_disclosed(&slot) {
            return Err(projection_denial(
                WorthQueryApplicationProjectionDenialKind::RelationContractMismatch,
                selector.slot_type(),
            ));
        }
        let projected = self.node.relation(selector.slot_type()).ok_or_else(|| {
            projection_denial(
                WorthQueryApplicationProjectionDenialKind::RelationNotProjected,
                selector.slot_type(),
            )
        })?;
        if projected.matches(selector) {
            Ok(WorthQueryApplicationDisclosed::Disclosed(projected))
        } else {
            Err(projection_denial(
                WorthQueryApplicationProjectionDenialKind::RelationContractMismatch,
                projected.result_path(),
            ))
        }
    }

    fn omission(
        &self,
        slot: &worth_query_declaration::facade::application_query::ApplicationQueryResultSlotKey,
    ) -> Option<WorthQueryApplicationOmission> {
        self.governance
            .omission(slot)
            .map(|(classification, required_disclosure)| WorthQueryApplicationOmission {
                classification: classification.to_string(),
                required_disclosure: required_disclosure.clone(),
            })
    }
}

impl<Value> WorthQueryApplicationDisclosed<Value> {
    fn map<Output>(
        self,
        disclosed: impl FnOnce(Value) -> Result<Output, WorthQueryApplicationProjectionDenial>,
    ) -> Result<WorthQueryApplicationDisclosed<Output>, WorthQueryApplicationProjectionDenial> {
        match self {
            Self::Disclosed(value) => disclosed(value).map(WorthQueryApplicationDisclosed::Disclosed),
            Self::Omitted(omission) => Ok(WorthQueryApplicationDisclosed::Omitted(omission)),
        }
    }
}
