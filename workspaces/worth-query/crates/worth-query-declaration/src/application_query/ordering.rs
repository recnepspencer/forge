use worth_foundational::facade::ScalarAspectType;

use crate::application_schema::{ApplicationFieldCurrency, TypedApplicationValue};

use super::ApplicationQueryResultFieldRef;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ApplicationQueryOrderingDirection {
    Ascending,
    Descending,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationQueryOrderingTerm {
    query_type: &'static str,
    slot_type: &'static str,
    entity: &'static str,
    aspect: &'static str,
    field: &'static str,
    output_name: &'static str,
    scalar_family: ScalarAspectType,
    value_type: &'static str,
    direction: ApplicationQueryOrderingDirection,
}

impl ApplicationQueryOrderingTerm {
    pub(super) fn from_result_field<
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
    >(
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
        direction: ApplicationQueryOrderingDirection,
    ) -> Self
    where
        Value: TypedApplicationValue,
        Currency: ApplicationFieldCurrency,
    {
        Self {
            query_type: selector.query_type(),
            slot_type: selector.slot_type(),
            entity: selector.entity(),
            aspect: selector.aspect(),
            field: selector.field(),
            output_name: selector.output_name(),
            scalar_family: selector.scalar_family(),
            value_type: selector.value_type(),
            direction,
        }
    }

    pub const fn query_type(&self) -> &'static str {
        self.query_type
    }

    pub const fn slot_type(&self) -> &'static str {
        self.slot_type
    }

    pub const fn field(&self) -> (&'static str, &'static str, &'static str) {
        (self.entity, self.aspect, self.field)
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

    pub const fn direction(&self) -> ApplicationQueryOrderingDirection {
        self.direction
    }
}
