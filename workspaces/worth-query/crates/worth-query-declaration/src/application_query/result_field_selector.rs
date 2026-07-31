use std::marker::PhantomData;

use worth_foundational::facade::ScalarAspectType;

use super::result_slot_key::ApplicationQueryResultFieldSlotContract;
use super::ApplicationQueryResultSlotKey;
use crate::application_schema::{
    ApplicationFieldCurrency, ApplicationFieldRef, TypedApplicationValue,
};

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
    Currency,
> {
    output_name: &'static str,
    entity: &'static str,
    aspect: &'static str,
    field: &'static str,
    _query_position: PhantomData<fn() -> (Query, Slot, Schema, Entity, Aspect)>,
    _field_contract: PhantomData<fn() -> (Field, Value, Write, Equality, Currency)>,
}

impl<Query, Slot, Schema, Entity, Aspect, Field, Value, Write, Equality, Currency>
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
        Currency,
    >
where
    Value: TypedApplicationValue,
    Currency: ApplicationFieldCurrency,
{
    pub fn new(
        output_name: &'static str,
        field: ApplicationFieldRef<Schema, Entity, Aspect, Field, Value, Write, Equality, Currency>,
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

    pub fn value_type(&self) -> &'static str {
        std::any::type_name::<Value>()
    }

    pub fn query_type(&self) -> &'static str {
        std::any::type_name::<Query>()
    }

    pub fn slot_type(&self) -> &'static str {
        std::any::type_name::<Slot>()
    }

    pub fn slot_key(&self) -> ApplicationQueryResultSlotKey
    where
        Query: 'static,
        Slot: 'static,
    {
        ApplicationQueryResultSlotKey::field::<Query, Slot>(
            ApplicationQueryResultFieldSlotContract {
                entity: self.entity,
                aspect: self.aspect,
                field: self.field,
                output_name: self.output_name,
                scalar_family: Value::SCALAR_FAMILY,
                value_type: std::any::type_name::<Value>(),
            },
        )
    }
}

impl<Query, Slot, Schema, Entity, Aspect, Field, Value, Write, Equality, Currency> Clone
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
        Currency,
    >
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<Query, Slot, Schema, Entity, Aspect, Field, Value, Write, Equality, Currency> Copy
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
        Currency,
    >
{
}
