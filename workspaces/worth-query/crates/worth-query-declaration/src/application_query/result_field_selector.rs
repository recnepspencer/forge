use std::marker::PhantomData;

use worth_foundational::facade::ScalarAspectType;

use super::result_slot_key::ApplicationQueryResultFieldSlotContract;
use super::ApplicationQueryMarkerIdentity;
use super::ApplicationQueryResultSlotKey;
use crate::application_schema::{
    ApplicationFieldPresence, ApplicationFieldRef, ApplicationFieldUnit,
    OptionalApplicationFieldValue, RequiredApplicationFieldValue, TypedApplicationValue,
};
use crate::portable_identity::WorthQueryPortableType;

/// Selector for a schema-required result field.
///
/// An optional schema field cannot be smuggled into the required result API:
///
/// ```compile_fail
/// use worth_query_declaration::facade::{
///     application_query::ApplicationQueryResultFieldRef,
///     application_schema::{
///         EqualityPredicate, NoApplicationUnit, ReadWrite,
///     },
/// };
/// struct Schema;
/// struct Query;
/// struct Slot;
/// worth_query_declaration::worth_query_entity!(pub Record in Schema);
/// worth_query_declaration::worth_query_aspect!(pub Facts in Schema, Record; identity = AspectIdentity(0x9161102b), revision = AspectContractRevision(1),);
/// worth_query_declaration::worth_query_field!(
///     pub OptionalText in Schema, Record, Facts:
///     optional String, read_write, equality
/// );
/// let _ = ApplicationQueryResultFieldRef::<
///     Query, Slot, Schema, Record, Facts, OptionalText, String,
///     ReadWrite, EqualityPredicate, NoApplicationUnit,
/// >::new("text", OptionalText::reference());
/// ```
pub struct ApplicationQueryResultFieldRef<
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
> {
    output_name: &'static str,
    entity: &'static str,
    aspect: &'static str,
    field: &'static str,
    _query_position: PhantomData<fn() -> (Query, Slot, Schema, Entity, Aspect)>,
    _field_contract: PhantomData<fn() -> (Field, Value, Write, Equality, Unit)>,
}

/// Selector for a schema-optional result field.
///
/// A required schema field cannot be reinterpreted as lawfully absent:
///
/// ```compile_fail
/// use worth_query_declaration::facade::{
///     application_query::ApplicationQueryOptionalResultFieldRef,
///     application_schema::{
///         EqualityPredicate, NoApplicationUnit, ReadWrite,
///     },
/// };
/// struct Schema;
/// struct Query;
/// struct Slot;
/// worth_query_declaration::worth_query_entity!(pub Record in Schema);
/// worth_query_declaration::worth_query_aspect!(pub Facts in Schema, Record; identity = AspectIdentity(0x9161102c), revision = AspectContractRevision(1),);
/// worth_query_declaration::worth_query_field!(
///     pub RequiredText in Schema, Record, Facts:
///     String, read_write, equality
/// );
/// let _ = ApplicationQueryOptionalResultFieldRef::<
///     Query, Slot, Schema, Record, Facts, RequiredText, String,
///     ReadWrite, EqualityPredicate, NoApplicationUnit,
/// >::new("text", RequiredText::reference());
/// ```
pub struct ApplicationQueryOptionalResultFieldRef<
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
> {
    output_name: &'static str,
    entity: &'static str,
    aspect: &'static str,
    field: &'static str,
    _query_position: PhantomData<fn() -> (Query, Slot, Schema, Entity, Aspect)>,
    _field_contract: PhantomData<fn() -> (Field, Value, Write, Equality, Unit)>,
}

impl<Query, Slot, Schema, Entity, Aspect, Field, Value, Write, Equality, Unit>
    ApplicationQueryResultFieldRef<
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
    >
where
    Value: TypedApplicationValue + WorthQueryPortableType,
    Unit: ApplicationFieldUnit,
{
    pub fn new(
        output_name: &'static str,
        field: ApplicationFieldRef<Schema, Entity, Aspect, Field, Value, Write, Equality, Unit>,
    ) -> Self
    where
        Field: RequiredApplicationFieldValue<Value = Value>,
    {
        Self {
            output_name,
            entity: field.entity(),
            aspect: field.aspect(),
            field: field.field(),
            _query_position: PhantomData,
            _field_contract: PhantomData,
        }
    }

    pub const fn output_name(&self) -> &'static str {
        self.output_name
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

    pub const fn scalar_family(&self) -> ScalarAspectType {
        Value::SCALAR_FAMILY
    }

    pub const fn presence(&self) -> ApplicationFieldPresence {
        ApplicationFieldPresence::Required
    }

    pub fn value_type(&self) -> &'static str {
        Value::PORTABLE_TYPE_NAME
    }

    pub fn query_type(&self) -> &'static str
    where
        Query: ApplicationQueryMarkerIdentity,
    {
        Query::QUERY_TYPE_NAME
    }

    pub fn slot_type(&self) -> &'static str
    where
        Slot: WorthQueryPortableType,
    {
        Slot::PORTABLE_TYPE_NAME
    }

    pub fn slot_key(&self) -> ApplicationQueryResultSlotKey
    where
        Query: ApplicationQueryMarkerIdentity,
        Slot: WorthQueryPortableType,
    {
        ApplicationQueryResultSlotKey::field(
            Query::QUERY_TYPE_IDENTITY,
            Slot::PORTABLE_TYPE_IDENTITY,
            ApplicationQueryResultFieldSlotContract {
                entity: self.entity,
                aspect: self.aspect,
                field: self.field,
                output_name: self.output_name,
                scalar_family: Value::SCALAR_FAMILY,
                value_type: Value::PORTABLE_TYPE_IDENTITY,
                presence: ApplicationFieldPresence::Required,
            },
        )
    }
}

impl<Query, Slot, Schema, Entity, Aspect, Field, Value, Write, Equality, Unit>
    ApplicationQueryOptionalResultFieldRef<
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
    >
where
    Field: OptionalApplicationFieldValue<Value = Value>,
    Value: TypedApplicationValue + WorthQueryPortableType,
    Unit: ApplicationFieldUnit,
{
    pub fn new(
        output_name: &'static str,
        field: ApplicationFieldRef<Schema, Entity, Aspect, Field, Value, Write, Equality, Unit>,
    ) -> Self {
        Self {
            output_name,
            entity: field.entity(),
            aspect: field.aspect(),
            field: field.field(),
            _query_position: PhantomData,
            _field_contract: PhantomData,
        }
    }

    pub const fn output_name(&self) -> &'static str {
        self.output_name
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

    pub const fn scalar_family(&self) -> ScalarAspectType {
        Value::SCALAR_FAMILY
    }

    pub const fn presence(&self) -> ApplicationFieldPresence {
        ApplicationFieldPresence::Optional
    }

    pub fn value_type(&self) -> &'static str {
        Value::PORTABLE_TYPE_NAME
    }

    pub fn query_type(&self) -> &'static str
    where
        Query: ApplicationQueryMarkerIdentity,
    {
        Query::QUERY_TYPE_NAME
    }

    pub fn slot_type(&self) -> &'static str
    where
        Slot: WorthQueryPortableType,
    {
        Slot::PORTABLE_TYPE_NAME
    }

    pub fn slot_key(&self) -> ApplicationQueryResultSlotKey
    where
        Query: ApplicationQueryMarkerIdentity,
        Slot: WorthQueryPortableType,
    {
        ApplicationQueryResultSlotKey::field(
            Query::QUERY_TYPE_IDENTITY,
            Slot::PORTABLE_TYPE_IDENTITY,
            ApplicationQueryResultFieldSlotContract {
                entity: self.entity,
                aspect: self.aspect,
                field: self.field,
                output_name: self.output_name,
                scalar_family: Value::SCALAR_FAMILY,
                value_type: Value::PORTABLE_TYPE_IDENTITY,
                presence: ApplicationFieldPresence::Optional,
            },
        )
    }
}

impl<Query, Slot, Schema, Entity, Aspect, Field, Value, Write, Equality, Unit> Clone
    for ApplicationQueryResultFieldRef<
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
    >
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<Query, Slot, Schema, Entity, Aspect, Field, Value, Write, Equality, Unit> Copy
    for ApplicationQueryResultFieldRef<
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
    >
{
}

impl<Query, Slot, Schema, Entity, Aspect, Field, Value, Write, Equality, Unit> Clone
    for ApplicationQueryOptionalResultFieldRef<
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
    >
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<Query, Slot, Schema, Entity, Aspect, Field, Value, Write, Equality, Unit> Copy
    for ApplicationQueryOptionalResultFieldRef<
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
    >
{
}
