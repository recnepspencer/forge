use std::marker::PhantomData;

use worth_foundational::facade::AspectValue;

use super::capabilities::{ApplicationFieldUnit, OperationExpectsFact, OperationExpectsVersion};
use super::references::ApplicationFieldRef;
use super::values::TypedApplicationValue;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ApplicationMutationPreconditionFamily {
    ExpectedVersion,
    ExpectedFact,
}

impl ApplicationMutationPreconditionFamily {
    pub const fn canonical_name(self) -> &'static str {
        match self {
            Self::ExpectedVersion => "expected-version",
            Self::ExpectedFact => "expected-fact",
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ApplicationMutationPreconditionTarget {
    family: ApplicationMutationPreconditionFamily,
    entity: String,
    aspect: String,
    field: String,
}

impl ApplicationMutationPreconditionTarget {
    pub(crate) fn field(
        family: ApplicationMutationPreconditionFamily,
        entity: &str,
        aspect: &str,
        field: &str,
    ) -> Self {
        Self {
            family,
            entity: entity.to_owned(),
            aspect: aspect.to_owned(),
            field: field.to_owned(),
        }
    }

    pub const fn family(&self) -> ApplicationMutationPreconditionFamily {
        self.family
    }

    pub fn entity(&self) -> &str {
        &self.entity
    }

    pub fn aspect(&self) -> &str {
        &self.aspect
    }

    pub fn field_name(&self) -> &str {
        &self.field
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypedMutationPrecondition {
    target: ApplicationMutationPreconditionTarget,
    expected_value: AspectValue,
}

impl TypedMutationPrecondition {
    pub const fn target(&self) -> &ApplicationMutationPreconditionTarget {
        &self.target
    }

    pub const fn expected_value(&self) -> &AspectValue {
        &self.expected_value
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypedMutationPreconditions<Schema, Operation, Scope> {
    entries: Vec<TypedMutationPrecondition>,
    _marker: PhantomData<fn() -> (Schema, Operation, Scope)>,
}

impl<Schema, Operation, Scope> Default for TypedMutationPreconditions<Schema, Operation, Scope> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Schema, Operation, Scope> TypedMutationPreconditions<Schema, Operation, Scope> {
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
            _marker: PhantomData,
        }
    }

    pub fn expect_version<Aspect, Field, Value, Write, Equality, Unit>(
        self,
        field: ApplicationFieldRef<Schema, Scope, Aspect, Field, Value, Write, Equality, Unit>,
        expected: Value,
    ) -> Self
    where
        Field: OperationExpectsVersion<Operation>,
        Value: TypedApplicationValue,
        Unit: ApplicationFieldUnit,
    {
        self.push(
            ApplicationMutationPreconditionFamily::ExpectedVersion,
            field,
            expected,
        )
    }

    pub fn expect_fact<Aspect, Field, Value, Write, Equality, Unit>(
        self,
        field: ApplicationFieldRef<Schema, Scope, Aspect, Field, Value, Write, Equality, Unit>,
        expected: Value,
    ) -> Self
    where
        Field: OperationExpectsFact<Operation>,
        Value: TypedApplicationValue,
        Unit: ApplicationFieldUnit,
    {
        self.push(
            ApplicationMutationPreconditionFamily::ExpectedFact,
            field,
            expected,
        )
    }

    pub fn entries(&self) -> &[TypedMutationPrecondition] {
        &self.entries
    }

    #[doc(hidden)]
    pub fn into_entries(self) -> Vec<TypedMutationPrecondition> {
        self.entries
    }

    fn push<Aspect, Field, Value, Write, Equality, Unit>(
        mut self,
        family: ApplicationMutationPreconditionFamily,
        field: ApplicationFieldRef<Schema, Scope, Aspect, Field, Value, Write, Equality, Unit>,
        expected: Value,
    ) -> Self
    where
        Value: TypedApplicationValue,
        Unit: ApplicationFieldUnit,
    {
        let expected_value = expected.into_foundational_value();
        self.entries.push(TypedMutationPrecondition {
            target: ApplicationMutationPreconditionTarget::field(
                family,
                field.entity(),
                field.aspect(),
                field.field(),
            ),
            expected_value,
        });
        self
    }
}
