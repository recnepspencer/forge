use worth_foundational::facade::{AspectValue, ScalarAspectType};

use crate::application_schema::{
    ApplicationFieldCurrency, ApplicationFieldRef, EqualityPredicate, TypedApplicationValue,
    WritePosture,
};

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
    pub(super) fn new<Schema, Entity, Aspect, Field, Value, Write, Currency>(
        after_step: usize,
        field: ApplicationFieldRef<
            Schema,
            Entity,
            Aspect,
            Field,
            Value,
            Write,
            EqualityPredicate,
            Currency,
        >,
        expected: Value,
    ) -> Self
    where
        Value: TypedApplicationValue,
        Write: WritePosture,
        Currency: ApplicationFieldCurrency,
    {
        Self {
            after_step,
            entity: field.entity(),
            aspect: field.aspect(),
            field: field.field(),
            scalar_family: field.scalar_family(),
            value_type: std::any::type_name::<Value>(),
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
