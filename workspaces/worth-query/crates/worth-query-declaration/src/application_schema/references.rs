use std::marker::PhantomData;

use worth_foundational::facade::ScalarAspectType;

use super::capabilities::{ApplicationFieldUnit, NoApplicationUnit, NoEqualityPredicate, ReadOnly};
use super::values::TypedApplicationValue;

macro_rules! named_reference {
    ($name:ident, $($marker:ident),+) => {
        pub struct $name<$($marker),+> {
            name: &'static str,
            _marker: PhantomData<fn() -> ($($marker),+)>,
        }

        impl<$($marker),+> $name<$($marker),+> {
            #[doc(hidden)]
            pub const fn from_schema_identifier(name: &'static str) -> Self {
                Self {
                    name,
                    _marker: PhantomData,
                }
            }

            pub const fn name(&self) -> &'static str {
                self.name
            }
        }

        impl<$($marker),+> Copy for $name<$($marker),+> {}

        impl<$($marker),+> Clone for $name<$($marker),+> {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<$($marker),+> std::fmt::Debug for $name<$($marker),+> {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .debug_struct(stringify!($name))
                    .field("name", &self.name)
                    .finish_non_exhaustive()
            }
        }

        impl<$($marker),+> PartialEq for $name<$($marker),+> {
            fn eq(&self, other: &Self) -> bool {
                self.name == other.name
            }
        }

        impl<$($marker),+> Eq for $name<$($marker),+> {}
    };
}

named_reference!(ApplicationEntityRef, Schema, Entity);
named_reference!(ApplicationAspectRef, Schema, Entity, Aspect);
named_reference!(ApplicationPolicyRef, Schema, Policy);
named_reference!(ApplicationUnitRef, Schema, Unit);

pub struct ApplicationAbilityRef<Schema, Ability, Scope> {
    name: &'static str,
    scope: &'static str,
    _marker: PhantomData<fn() -> (Schema, Ability, Scope)>,
}

impl<Schema, Ability, Scope> Copy for ApplicationAbilityRef<Schema, Ability, Scope> {}

impl<Schema, Ability, Scope> Clone for ApplicationAbilityRef<Schema, Ability, Scope> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Schema, Ability, Scope> ApplicationAbilityRef<Schema, Ability, Scope> {
    #[doc(hidden)]
    pub const fn from_schema_identifiers(name: &'static str, scope: &'static str) -> Self {
        Self {
            name,
            scope,
            _marker: PhantomData,
        }
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn scope(&self) -> &'static str {
        self.scope
    }
}

impl<Schema, Ability, Scope> std::fmt::Debug for ApplicationAbilityRef<Schema, Ability, Scope> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ApplicationAbilityRef")
            .field("name", &self.name)
            .field("scope", &self.scope)
            .finish_non_exhaustive()
    }
}

impl<Schema, Ability, Scope> PartialEq for ApplicationAbilityRef<Schema, Ability, Scope> {
    fn eq(&self, other: &Self) -> bool {
        (self.name, self.scope) == (other.name, other.scope)
    }
}

impl<Schema, Ability, Scope> Eq for ApplicationAbilityRef<Schema, Ability, Scope> {}

pub struct ApplicationRelationRef<Schema, Relation, From, To> {
    name: &'static str,
    from: &'static str,
    to: &'static str,
    _marker: PhantomData<fn() -> (Schema, Relation, From, To)>,
}

impl<Schema, Relation, From, To> Copy for ApplicationRelationRef<Schema, Relation, From, To> {}

impl<Schema, Relation, From, To> Clone for ApplicationRelationRef<Schema, Relation, From, To> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Schema, Relation, From, To> std::fmt::Debug
    for ApplicationRelationRef<Schema, Relation, From, To>
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ApplicationRelationRef")
            .field("name", &self.name)
            .field("from", &self.from)
            .field("to", &self.to)
            .finish_non_exhaustive()
    }
}

impl<Schema, Relation, From, To> PartialEq for ApplicationRelationRef<Schema, Relation, From, To> {
    fn eq(&self, other: &Self) -> bool {
        (self.name, self.from, self.to) == (other.name, other.from, other.to)
    }
}

impl<Schema, Relation, From, To> Eq for ApplicationRelationRef<Schema, Relation, From, To> {}

impl<Schema, Relation, From, To> ApplicationRelationRef<Schema, Relation, From, To> {
    #[doc(hidden)]
    pub const fn from_schema_identifiers(
        name: &'static str,
        from: &'static str,
        to: &'static str,
    ) -> Self {
        Self {
            name,
            from,
            to,
            _marker: PhantomData,
        }
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn from(&self) -> &'static str {
        self.from
    }

    pub const fn to(&self) -> &'static str {
        self.to
    }
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
    Value: TypedApplicationValue,
    Unit: ApplicationFieldUnit,
{
    #[doc(hidden)]
    pub const fn from_schema_identifiers(
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
        std::any::type_name::<Value>()
    }

    pub const fn unit(&self) -> Option<&'static str> {
        Unit::NAME
    }
}

pub struct ApplicationOperationRef<Schema, Operation, Input> {
    name: &'static str,
    _marker: PhantomData<fn(Input) -> (Schema, Operation)>,
}

impl<Schema, Operation, Input> Copy for ApplicationOperationRef<Schema, Operation, Input> {}

impl<Schema, Operation, Input> Clone for ApplicationOperationRef<Schema, Operation, Input> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Schema, Operation, Input> std::fmt::Debug
    for ApplicationOperationRef<Schema, Operation, Input>
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ApplicationOperationRef")
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

impl<Schema, Operation, Input> PartialEq for ApplicationOperationRef<Schema, Operation, Input> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<Schema, Operation, Input> Eq for ApplicationOperationRef<Schema, Operation, Input> {}

impl<Schema, Operation, Input> ApplicationOperationRef<Schema, Operation, Input> {
    #[doc(hidden)]
    pub const fn from_schema_identifier(name: &'static str) -> Self {
        Self {
            name,
            _marker: PhantomData,
        }
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }
}

pub struct ApplicationEffectRef<Schema, Effect, Payload> {
    name: &'static str,
    _marker: PhantomData<fn(Payload) -> (Schema, Effect)>,
}

impl<Schema, Effect, Payload> Copy for ApplicationEffectRef<Schema, Effect, Payload> {}

impl<Schema, Effect, Payload> Clone for ApplicationEffectRef<Schema, Effect, Payload> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Schema, Effect, Payload> std::fmt::Debug for ApplicationEffectRef<Schema, Effect, Payload> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ApplicationEffectRef")
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

impl<Schema, Effect, Payload> PartialEq for ApplicationEffectRef<Schema, Effect, Payload> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<Schema, Effect, Payload> Eq for ApplicationEffectRef<Schema, Effect, Payload> {}

impl<Schema, Effect, Payload> ApplicationEffectRef<Schema, Effect, Payload> {
    #[doc(hidden)]
    pub const fn from_schema_identifier(name: &'static str) -> Self {
        Self {
            name,
            _marker: PhantomData,
        }
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }
}
