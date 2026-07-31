use crate::application_schema::ApplicationAuthorizationPath;
use crate::application_schema::{
    ApplicationFieldCurrency, ApplicationFieldRef, TypedApplicationValue, WritePosture,
};
use worth_foundational::facade::AspectValue;

use super::ApplicationCapabilityFieldBinding;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationCapabilityAcceptedValues {
    field: ApplicationCapabilityFieldBinding,
    values: Vec<AspectValue>,
}

impl ApplicationCapabilityAcceptedValues {
    pub fn one_of<Schema, Entity, Aspect, Field, Value, Write, Equality, Currency>(
        field: ApplicationFieldRef<Schema, Entity, Aspect, Field, Value, Write, Equality, Currency>,
        values: impl IntoIterator<Item = Value>,
    ) -> Self
    where
        Value: TypedApplicationValue,
        Write: WritePosture,
        Currency: ApplicationFieldCurrency,
    {
        let mut values = values
            .into_iter()
            .map(TypedApplicationValue::into_foundational_value)
            .collect::<Vec<_>>();
        values.sort();
        values.dedup();
        Self {
            field: ApplicationCapabilityFieldBinding::from_reference(field),
            values,
        }
    }

    pub const fn field(&self) -> &ApplicationCapabilityFieldBinding {
        &self.field
    }

    pub fn values(&self) -> &[AspectValue] {
        &self.values
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationCapabilityScopeGuard {
    requirements: Vec<ApplicationCapabilityAcceptedValues>,
}

impl ApplicationCapabilityScopeGuard {
    pub const fn unconditional() -> Self {
        Self {
            requirements: Vec::new(),
        }
    }

    pub fn requiring(
        requirements: impl IntoIterator<Item = ApplicationCapabilityAcceptedValues>,
    ) -> Self {
        let mut requirements = requirements.into_iter().collect::<Vec<_>>();
        requirements.sort();
        requirements.dedup();
        Self { requirements }
    }

    pub fn requirements(&self) -> &[ApplicationCapabilityAcceptedValues] {
        &self.requirements
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationCapabilityGraphClause {
    path: ApplicationAuthorizationPath,
    guard: ApplicationCapabilityScopeGuard,
}

impl ApplicationCapabilityGraphClause {
    pub const fn new(path: ApplicationAuthorizationPath) -> Self {
        Self {
            path,
            guard: ApplicationCapabilityScopeGuard::unconditional(),
        }
    }

    pub fn when(
        path: ApplicationAuthorizationPath,
        requirements: impl IntoIterator<Item = ApplicationCapabilityAcceptedValues>,
    ) -> Self {
        Self {
            path,
            guard: ApplicationCapabilityScopeGuard::requiring(requirements),
        }
    }

    pub const fn path(&self) -> &ApplicationAuthorizationPath {
        &self.path
    }

    pub const fn guard(&self) -> &ApplicationCapabilityScopeGuard {
        &self.guard
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationCapabilityGraphRule {
    clauses: Vec<ApplicationCapabilityGraphClause>,
}

impl ApplicationCapabilityGraphRule {
    pub fn any(clauses: impl IntoIterator<Item = ApplicationCapabilityGraphClause>) -> Self {
        let mut clauses = clauses.into_iter().collect::<Vec<_>>();
        clauses.sort();
        clauses.dedup();
        Self { clauses }
    }

    pub fn clauses(&self) -> &[ApplicationCapabilityGraphClause] {
        &self.clauses
    }
}
