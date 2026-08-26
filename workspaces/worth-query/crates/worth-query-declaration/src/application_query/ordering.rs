use worth_foundational::facade::ScalarAspectType;

use crate::application_schema::{ApplicationFieldUnit, TypedApplicationValue};

use super::{ApplicationQueryMarkerIdentity, ApplicationQueryResultFieldRef};
use crate::portable_identity::{WorthQueryPortableType, WorthQueryPortableTypeIdentity};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ApplicationQueryOrderingDirection {
    Ascending,
    Descending,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationQueryOrderingTerm {
    query_type: WorthQueryPortableTypeIdentity,
    slot_type: WorthQueryPortableTypeIdentity,
    entity: &'static str,
    aspect: &'static str,
    field: &'static str,
    output_name: &'static str,
    scalar_family: ScalarAspectType,
    value_type: WorthQueryPortableTypeIdentity,
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
        Unit,
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
            Unit,
        >,
        direction: ApplicationQueryOrderingDirection,
    ) -> Self
    where
        Value: TypedApplicationValue + WorthQueryPortableType,
        Unit: ApplicationFieldUnit,
        Query: ApplicationQueryMarkerIdentity,
        Slot: WorthQueryPortableType,
    {
        Self {
            query_type: selector.slot_key().query_identity(),
            slot_type: selector.slot_key().slot_identity(),
            entity: selector.entity(),
            aspect: selector.aspect(),
            field: selector.field(),
            output_name: selector.output_name(),
            scalar_family: selector.scalar_family(),
            value_type: Value::PORTABLE_TYPE_IDENTITY,
            direction,
        }
    }

    pub const fn query_type(&self) -> &'static str {
        self.query_type.as_str()
    }

    pub const fn slot_type(&self) -> &'static str {
        self.slot_type.as_str()
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
        self.value_type.as_str()
    }

    pub const fn direction(&self) -> ApplicationQueryOrderingDirection {
        self.direction
    }
}
