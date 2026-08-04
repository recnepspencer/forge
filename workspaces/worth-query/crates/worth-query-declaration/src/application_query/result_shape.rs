use std::marker::PhantomData;

use worth_foundational::facade::ScalarAspectType;

use crate::application_schema::{
    ApplicationFieldCurrency, ApplicationFieldPresence, OptionalApplicationFieldValue,
    RequiredApplicationFieldValue, TypedApplicationValue,
};

use super::{
    ApplicationQueryCardinality, ApplicationQueryOptionalResultFieldRef,
    ApplicationQueryResultFieldRef, ApplicationQueryResultRelationCardinality,
    ApplicationQueryResultRelationRef, ApplicationQueryResultSlotKey,
    ApplicationQueryResultTraversalDirection, ApplicationQueryResultTraversalEndpoints,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationQueryResultField {
    slot_key: ApplicationQueryResultSlotKey,
    query_type: &'static str,
    slot_type: &'static str,
    entity: &'static str,
    aspect: &'static str,
    field: &'static str,
    output_name: &'static str,
    scalar_family: ScalarAspectType,
    value_type: &'static str,
    presence: ApplicationFieldPresence,
}

impl ApplicationQueryResultField {
    pub const fn slot_key(&self) -> ApplicationQueryResultSlotKey {
        self.slot_key
    }

    pub const fn query_type(&self) -> &'static str {
        self.query_type
    }

    pub const fn slot_type(&self) -> &'static str {
        self.slot_type
    }

    pub const fn entity(&self) -> &'static str {
        self.entity
    }

    pub const fn aspect(&self) -> &'static str {
        self.aspect
    }

    pub const fn field(&self) -> &'static str {
        self.field
    }

    pub const fn output_name(&self) -> &'static str {
        self.output_name
    }

    pub const fn scalar_family(&self) -> ScalarAspectType {
        self.scalar_family
    }

    pub const fn value_type(&self) -> &'static str {
        self.value_type
    }

    pub const fn presence(&self) -> ApplicationFieldPresence {
        self.presence
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationQueryResultRelation {
    slot_key: ApplicationQueryResultSlotKey,
    query_type: &'static str,
    slot_type: &'static str,
    relation: &'static str,
    from: &'static str,
    to: &'static str,
    direction: ApplicationQueryResultTraversalDirection,
    output_name: &'static str,
    cardinality: ApplicationQueryCardinality,
    nested_shape: Box<ApplicationQueryResultShape>,
}

impl ApplicationQueryResultRelation {
    pub const fn slot_key(&self) -> ApplicationQueryResultSlotKey {
        self.slot_key
    }

    pub const fn query_type(&self) -> &'static str {
        self.query_type
    }

    pub const fn slot_type(&self) -> &'static str {
        self.slot_type
    }

    pub const fn relation(&self) -> &'static str {
        self.relation
    }

    pub const fn from(&self) -> &'static str {
        self.from
    }

    pub const fn to(&self) -> &'static str {
        self.to
    }

    pub const fn direction(&self) -> ApplicationQueryResultTraversalDirection {
        self.direction
    }

    pub const fn output_name(&self) -> &'static str {
        self.output_name
    }

    pub const fn cardinality(&self) -> ApplicationQueryCardinality {
        self.cardinality
    }
    pub fn nested_shape(&self) -> &ApplicationQueryResultShape {
        &self.nested_shape
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationQueryResultShape {
    query_type: &'static str,
    root_entity: &'static str,
    result_type: &'static str,
    fields: Vec<ApplicationQueryResultField>,
    relations: Vec<ApplicationQueryResultRelation>,
}

impl ApplicationQueryResultShape {
    pub const fn query_type(&self) -> &'static str {
        self.query_type
    }

    pub const fn root_entity(&self) -> &'static str {
        self.root_entity
    }

    pub const fn result_type(&self) -> &'static str {
        self.result_type
    }

    pub fn fields(&self) -> &[ApplicationQueryResultField] {
        &self.fields
    }

    pub fn relations(&self) -> &[ApplicationQueryResultRelation] {
        &self.relations
    }
}

pub struct TypedApplicationQueryResultShape<Schema, Query, Entity, Result> {
    shape: ApplicationQueryResultShape,
    _marker: PhantomData<fn() -> (Schema, Query, Entity, Result)>,
}

impl<Schema, Query, Entity, Result>
    TypedApplicationQueryResultShape<Schema, Query, Entity, Result>
{
    pub(crate) fn into_erased(self) -> ApplicationQueryResultShape {
        self.shape
    }
}

pub struct ApplicationQueryResultShapeBuilder<Schema, Query, Entity, Result> {
    root_entity: &'static str,
    fields: Vec<ApplicationQueryResultField>,
    relations: Vec<ApplicationQueryResultRelation>,
    _marker: PhantomData<fn() -> (Schema, Query, Entity, Result)>,
}

impl<Schema, Query, Entity, Result>
    ApplicationQueryResultShapeBuilder<Schema, Query, Entity, Result>
{
    pub fn new(root: crate::application_schema::ApplicationEntityRef<Schema, Entity>) -> Self {
        Self {
            root_entity: root.name(),
            fields: Vec::new(),
            relations: Vec::new(),
            _marker: PhantomData,
        }
    }

    pub fn field<Slot, Aspect, Field, Value, Write, Equality, Currency>(
        mut self,
        field: ApplicationQueryResultFieldRef<
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
    ) -> Self
    where
        Field: RequiredApplicationFieldValue<Value = Value>,
        Value: TypedApplicationValue,
        Currency: ApplicationFieldCurrency,
        Query: 'static,
        Slot: 'static,
    {
        self.fields.push(ApplicationQueryResultField {
            slot_key: field.slot_key(),
            query_type: field.query_type(),
            slot_type: field.slot_type(),
            entity: field.entity(),
            aspect: field.aspect(),
            field: field.field(),
            output_name: field.output_name(),
            scalar_family: Value::SCALAR_FAMILY,
            value_type: std::any::type_name::<Value>(),
            presence: ApplicationFieldPresence::Required,
        });
        self
    }

    pub fn optional_field<Slot, Aspect, Field, Value, Write, Equality, Currency>(
        mut self,
        field: ApplicationQueryOptionalResultFieldRef<
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
    ) -> Self
    where
        Field: OptionalApplicationFieldValue<Value = Value>,
        Value: TypedApplicationValue,
        Currency: ApplicationFieldCurrency,
        Query: 'static,
        Slot: 'static,
    {
        self.fields.push(ApplicationQueryResultField {
            slot_key: field.slot_key(),
            query_type: field.query_type(),
            slot_type: field.slot_type(),
            entity: field.entity(),
            aspect: field.aspect(),
            field: field.field(),
            output_name: field.output_name(),
            scalar_family: Value::SCALAR_FAMILY,
            value_type: std::any::type_name::<Value>(),
            presence: ApplicationFieldPresence::Optional,
        });
        self
    }

    pub fn relation<
        Slot,
        Relation,
        DeclaredFrom,
        DeclaredTo,
        Direction,
        Child,
        Cardinality,
        NestedResult,
    >(
        mut self,
        relation: ApplicationQueryResultRelationRef<
            Query,
            Slot,
            Schema,
            Relation,
            DeclaredFrom,
            DeclaredTo,
            Direction,
            Cardinality,
        >,
        nested: ApplicationQueryResultShapeBuilder<Schema, Query, Child, NestedResult>,
    ) -> Self
    where
        Direction:
            ApplicationQueryResultTraversalEndpoints<Entity, Child, DeclaredFrom, DeclaredTo>,
        Cardinality: ApplicationQueryResultRelationCardinality,
        Query: 'static,
        Slot: 'static,
    {
        self.relations.push(ApplicationQueryResultRelation {
            slot_key: relation.slot_key(),
            query_type: relation.query_type(),
            slot_type: relation.slot_type(),
            relation: relation.relation(),
            from: relation.from(),
            to: relation.to(),
            direction: relation.direction(),
            output_name: relation.output_name(),
            cardinality: relation.cardinality(),
            nested_shape: Box::new(nested.build().into_erased()),
        });
        self
    }

    pub fn build(mut self) -> TypedApplicationQueryResultShape<Schema, Query, Entity, Result> {
        self.fields.sort();
        self.relations.sort();
        TypedApplicationQueryResultShape {
            shape: ApplicationQueryResultShape {
                query_type: std::any::type_name::<Query>(),
                root_entity: self.root_entity,
                result_type: std::any::type_name::<Result>(),
                fields: self.fields,
                relations: self.relations,
            },
            _marker: PhantomData,
        }
    }
}
