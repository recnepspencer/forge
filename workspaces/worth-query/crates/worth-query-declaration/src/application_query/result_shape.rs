use std::marker::PhantomData;

use worth_foundational::facade::ScalarAspectType;

use crate::application_schema::{
    ApplicationFieldPresence, ApplicationFieldUnit, OptionalApplicationFieldValue,
    RequiredApplicationFieldValue, TypedApplicationValue,
};

use super::{
    ApplicationQueryCardinality, ApplicationQueryMarkerIdentity,
    ApplicationQueryOptionalResultFieldRef, ApplicationQueryResultFieldRef,
    ApplicationQueryResultRelationCardinality, ApplicationQueryResultRelationRef,
    ApplicationQueryResultSlotKey, ApplicationQueryResultTraversalDirection,
    ApplicationQueryResultTraversalEndpoints,
};
use crate::portable_identity::{WorthQueryPortableType, WorthQueryPortableTypeIdentity};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationQueryResultField {
    slot_key: ApplicationQueryResultSlotKey,
    query_type: WorthQueryPortableTypeIdentity,
    slot_type: WorthQueryPortableTypeIdentity,
    entity: &'static str,
    aspect: &'static str,
    field: &'static str,
    output_name: &'static str,
    scalar_family: ScalarAspectType,
    value_type: WorthQueryPortableTypeIdentity,
    presence: ApplicationFieldPresence,
}

impl ApplicationQueryResultField {
    pub const fn slot_key(&self) -> ApplicationQueryResultSlotKey {
        self.slot_key
    }

    pub const fn query_type(&self) -> &'static str {
        self.query_type.as_str()
    }

    pub const fn query_identity(&self) -> WorthQueryPortableTypeIdentity {
        self.query_type
    }

    pub const fn slot_type(&self) -> &'static str {
        self.slot_type.as_str()
    }

    pub const fn slot_identity(&self) -> WorthQueryPortableTypeIdentity {
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
        self.value_type.as_str()
    }

    pub const fn value_identity(&self) -> WorthQueryPortableTypeIdentity {
        self.value_type
    }

    pub const fn presence(&self) -> ApplicationFieldPresence {
        self.presence
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationQueryResultRelation {
    slot_key: ApplicationQueryResultSlotKey,
    query_type: WorthQueryPortableTypeIdentity,
    slot_type: WorthQueryPortableTypeIdentity,
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
        self.query_type.as_str()
    }

    pub const fn query_identity(&self) -> WorthQueryPortableTypeIdentity {
        self.query_type
    }

    pub const fn slot_type(&self) -> &'static str {
        self.slot_type.as_str()
    }

    pub const fn slot_identity(&self) -> WorthQueryPortableTypeIdentity {
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
    query_type: WorthQueryPortableTypeIdentity,
    root_entity: &'static str,
    result_type: WorthQueryPortableTypeIdentity,
    fields: Vec<ApplicationQueryResultField>,
    relations: Vec<ApplicationQueryResultRelation>,
}

impl ApplicationQueryResultShape {
    pub const fn query_type(&self) -> &'static str {
        self.query_type.as_str()
    }

    pub const fn query_identity(&self) -> WorthQueryPortableTypeIdentity {
        self.query_type
    }

    pub const fn root_entity(&self) -> &'static str {
        self.root_entity
    }

    pub const fn result_type(&self) -> &'static str {
        self.result_type.as_str()
    }

    pub const fn result_identity(&self) -> WorthQueryPortableTypeIdentity {
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

    pub fn field<Slot, Aspect, Field, Value, Write, Equality, Unit>(
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
            Unit,
        >,
    ) -> Self
    where
        Field: RequiredApplicationFieldValue<Value = Value>,
        Value: TypedApplicationValue + WorthQueryPortableType,
        Unit: ApplicationFieldUnit,
        Query: ApplicationQueryMarkerIdentity,
        Slot: WorthQueryPortableType,
    {
        self.fields.push(ApplicationQueryResultField {
            slot_key: field.slot_key(),
            query_type: field.slot_key().query_identity(),
            slot_type: field.slot_key().slot_identity(),
            entity: field.entity(),
            aspect: field.aspect(),
            field: field.field(),
            output_name: field.output_name(),
            scalar_family: Value::SCALAR_FAMILY,
            value_type: Value::PORTABLE_TYPE_IDENTITY,
            presence: ApplicationFieldPresence::Required,
        });
        self
    }

    pub fn optional_field<Slot, Aspect, Field, Value, Write, Equality, Unit>(
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
            Unit,
        >,
    ) -> Self
    where
        Field: OptionalApplicationFieldValue<Value = Value>,
        Value: TypedApplicationValue + WorthQueryPortableType,
        Unit: ApplicationFieldUnit,
        Query: ApplicationQueryMarkerIdentity,
        Slot: WorthQueryPortableType,
    {
        self.fields.push(ApplicationQueryResultField {
            slot_key: field.slot_key(),
            query_type: field.slot_key().query_identity(),
            slot_type: field.slot_key().slot_identity(),
            entity: field.entity(),
            aspect: field.aspect(),
            field: field.field(),
            output_name: field.output_name(),
            scalar_family: Value::SCALAR_FAMILY,
            value_type: Value::PORTABLE_TYPE_IDENTITY,
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
        Query: ApplicationQueryMarkerIdentity,
        Slot: WorthQueryPortableType,
        NestedResult: WorthQueryPortableType,
    {
        self.relations.push(ApplicationQueryResultRelation {
            slot_key: relation.slot_key(),
            query_type: relation.slot_key().query_identity(),
            slot_type: relation.slot_key().slot_identity(),
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

    pub fn build(mut self) -> TypedApplicationQueryResultShape<Schema, Query, Entity, Result>
    where
        Query: ApplicationQueryMarkerIdentity,
        Result: WorthQueryPortableType,
    {
        self.fields.sort();
        self.relations.sort();
        TypedApplicationQueryResultShape {
            shape: ApplicationQueryResultShape {
                query_type: Query::QUERY_TYPE_IDENTITY,
                root_entity: self.root_entity,
                result_type: Result::PORTABLE_TYPE_IDENTITY,
                fields: self.fields,
                relations: self.relations,
            },
            _marker: PhantomData,
        }
    }
}
