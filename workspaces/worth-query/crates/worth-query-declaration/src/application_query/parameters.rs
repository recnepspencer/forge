use std::marker::PhantomData;

use worth_foundational::facade::{AspectValue, ScalarAspectType};

use crate::application_schema::TypedApplicationValue;

pub struct ApplicationQueryParameterRef<Query, Parameter, Value> {
    name: &'static str,
    _marker: PhantomData<fn(Value) -> (Query, Parameter)>,
}

impl<Query, Parameter, Value> ApplicationQueryParameterRef<Query, Parameter, Value> {
    #[doc(hidden)]
    pub const fn from_query_identifier(name: &'static str) -> Self {
        Self {
            name,
            _marker: PhantomData,
        }
    }

    pub const fn name(self) -> &'static str {
        self.name
    }
}

impl<Query, Parameter, Value> Clone for ApplicationQueryParameterRef<Query, Parameter, Value> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Query, Parameter, Value> Copy for ApplicationQueryParameterRef<Query, Parameter, Value> {}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationQueryParameterDefinition {
    name: &'static str,
    scalar_family: ScalarAspectType,
    value_type: &'static str,
}

impl ApplicationQueryParameterDefinition {
    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn scalar_family(&self) -> ScalarAspectType {
        self.scalar_family
    }

    pub const fn value_type(&self) -> &'static str {
        self.value_type
    }

    pub(super) fn typed<Query, Parameter, Value>(
        parameter: ApplicationQueryParameterRef<Query, Parameter, Value>,
    ) -> Self
    where
        Value: TypedApplicationValue,
    {
        Self {
            name: parameter.name(),
            scalar_family: Value::SCALAR_FAMILY,
            value_type: std::any::type_name::<Value>(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ApplicationQueryParameterSet<Query> {
    bindings: Vec<(&'static str, AspectValue)>,
    _query: PhantomData<fn() -> Query>,
}

impl<Query> Clone for ApplicationQueryParameterSet<Query> {
    fn clone(&self) -> Self {
        Self {
            bindings: self.bindings.clone(),
            _query: PhantomData,
        }
    }
}

impl<Query> ApplicationQueryParameterSet<Query> {
    pub fn new() -> Self {
        Self {
            bindings: Vec::new(),
            _query: PhantomData,
        }
    }

    pub fn bind<Parameter, Value>(
        mut self,
        parameter: ApplicationQueryParameterRef<Query, Parameter, Value>,
        value: Value,
    ) -> Self
    where
        Value: TypedApplicationValue,
    {
        self.bindings
            .push((parameter.name(), value.into_foundational_value()));
        self
    }

    pub fn bindings(&self) -> &[(&'static str, AspectValue)] {
        &self.bindings
    }
}

impl<Query> Default for ApplicationQueryParameterSet<Query> {
    fn default() -> Self {
        Self::new()
    }
}
