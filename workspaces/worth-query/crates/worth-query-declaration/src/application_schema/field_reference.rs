//! Exact typed application-field reference and marker ownership.

use std::marker::PhantomData;

use worth_foundational::facade::ScalarAspectType;

use super::capabilities::{ApplicationFieldUnit, NoApplicationUnit, NoEqualityPredicate, ReadOnly};
use super::values::TypedApplicationValue;
use super::ApplicationAspectMarkerIdentity;
use crate::portable_identity::WorthQueryPortableType;

/// Schema-declared entity marker identity used to mint exact typed field references.
pub trait ApplicationEntityMarkerIdentity {
    type Schema;
    const IDENTIFIER: &'static str;
}

/// Schema-declared field marker identity used to mint exact typed field references.
pub trait ApplicationFieldMarkerIdentity {
    type Schema;
    type Entity;
    type Aspect;
    const IDENTIFIER: &'static str;
}

pub struct ApplicationFieldRef<
    Schema,
    Entity,
    Aspect,
    Field,
    Value,
    Write = ReadOnly,
    Equality = NoEqualityPredicate,
    Unit = NoApplicationUnit,
> {
    entity: &'static str,
    aspect: &'static str,
    field: &'static str,
    _marker: PhantomData<fn() -> (Schema, Entity, Aspect, Field, Value, Write, Equality, Unit)>,
}

impl<Schema, Entity, Aspect, Field, Value, Write, Equality, Unit> Copy
    for ApplicationFieldRef<Schema, Entity, Aspect, Field, Value, Write, Equality, Unit>
{
}

impl<Schema, Entity, Aspect, Field, Value, Write, Equality, Unit> Clone
    for ApplicationFieldRef<Schema, Entity, Aspect, Field, Value, Write, Equality, Unit>
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<Schema, Entity, Aspect, Field, Value, Write, Equality, Unit> std::fmt::Debug
    for ApplicationFieldRef<Schema, Entity, Aspect, Field, Value, Write, Equality, Unit>
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ApplicationFieldRef")
            .field("entity", &self.entity)
            .field("aspect", &self.aspect)
            .field("field", &self.field)
            .finish_non_exhaustive()
    }
}

impl<Schema, Entity, Aspect, Field, Value, Write, Equality, Unit> PartialEq
    for ApplicationFieldRef<Schema, Entity, Aspect, Field, Value, Write, Equality, Unit>
{
    fn eq(&self, other: &Self) -> bool {
        (self.entity, self.aspect, self.field) == (other.entity, other.aspect, other.field)
    }
}

impl<Schema, Entity, Aspect, Field, Value, Write, Equality, Unit> Eq
    for ApplicationFieldRef<Schema, Entity, Aspect, Field, Value, Write, Equality, Unit>
{
}

impl<Schema, Entity, Aspect, Field, Value, Write, Equality, Unit>
    ApplicationFieldRef<Schema, Entity, Aspect, Field, Value, Write, Equality, Unit>
where
    Value: TypedApplicationValue + WorthQueryPortableType,
    Unit: ApplicationFieldUnit,
{
    #[doc(hidden)]
    #[cfg(test)]
    pub(crate) const fn from_schema_identifiers(
        entity: &'static str,
        aspect: &'static str,
        field: &'static str,
    ) -> Self {
        Self {
            entity,
            aspect,
            field,
            _marker: PhantomData,
        }
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

    pub fn value_type_name(&self) -> &'static str {
        Value::PORTABLE_TYPE_IDENTITY.as_str()
    }

    pub const fn unit(&self) -> Option<&'static str> {
        Unit::NAME
    }
}

impl<Schema, Entity, Aspect, Field, Value, Write, Equality, Unit>
    ApplicationFieldRef<Schema, Entity, Aspect, Field, Value, Write, Equality, Unit>
where
    Entity: ApplicationEntityMarkerIdentity<Schema = Schema>,
    Aspect: ApplicationAspectMarkerIdentity<Schema = Schema, Entity = Entity>,
    Field: ApplicationFieldMarkerIdentity<Schema = Schema, Entity = Entity, Aspect = Aspect>,
    Value: TypedApplicationValue + WorthQueryPortableType,
    Unit: ApplicationFieldUnit,
{
    /// Mint a field reference whose semantic axes are fixed by its declared marker types.
    pub const fn from_schema_types() -> Self {
        Self {
            entity: Entity::IDENTIFIER,
            aspect: Aspect::IDENTIFIER,
            field: Field::IDENTIFIER,
            _marker: PhantomData,
        }
    }
}
