use worth_foundational::facade::{AspectValue, ScalarAspectType};

use crate::application_schema::{
    ApplicationFieldRef, ApplicationFieldUnit, EqualityPredicate, TypedApplicationValue,
    WritePosture,
};
use crate::portable_identity::WorthQueryPortableType;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationQueryRootPathGuard {
    after_step: usize,
    entity: &'static str,
    aspect: &'static str,
    field: &'static str,
    scalar_family: ScalarAspectType,
    value_type: &'static str,
    expected: AspectValue,
}

impl ApplicationQueryRootPathGuard {
    pub(super) fn new<Schema, Entity, Aspect, Field, Value, Write, Unit>(
        after_step: usize,
        field: ApplicationFieldRef<
            Schema,
            Entity,
            Aspect,
            Field,
            Value,
            Write,
            EqualityPredicate,
            Unit,
        >,
        expected: Value,
    ) -> Self
    where
        Value: TypedApplicationValue + WorthQueryPortableType,
        Write: WritePosture,
        Unit: ApplicationFieldUnit,
    {
        Self {
            after_step,
            entity: field.entity(),
            aspect: field.aspect(),
            field: field.field(),
            scalar_family: field.scalar_family(),
            value_type: Value::PORTABLE_TYPE_IDENTITY.as_str(),
            expected: expected.into_foundational_value(),
        }
    }

    pub const fn after_step(&self) -> usize {
        self.after_step
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
        self.scalar_family
    }

    pub const fn value_type(&self) -> &'static str {
        self.value_type
    }

    pub const fn expected(&self) -> &AspectValue {
        &self.expected
    }
}
