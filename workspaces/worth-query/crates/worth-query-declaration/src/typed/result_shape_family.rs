use std::marker::PhantomData;

use crate::authoring::{
    AuthoringError, DeliveredFieldName, ResultShapeAuthoringFamily, ResultShapeBuilder,
};

use super::field_construction::result_shape_field_for;
use super::traits::{TypedProjectableField, TypedSchemaRoot};

pub trait TypedResultShapeSurfaceFamily: Clone + std::fmt::Debug + Eq + PartialEq {
    type AuthoringFamily: ResultShapeAuthoringFamily;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypedAuthoredResultShape<S: TypedSchemaRoot, F: TypedResultShapeSurfaceFamily> {
    pub(super) inner: crate::authoring::AuthoredResultShape<F::AuthoringFamily>,
    pub(super) _schema: PhantomData<S>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypedResultShapeBuilder<S: TypedSchemaRoot, F: TypedResultShapeSurfaceFamily> {
    inner: ResultShapeBuilder<F::AuthoringFamily>,
    _schema: PhantomData<S>,
}

impl<S: TypedSchemaRoot, F: TypedResultShapeSurfaceFamily> TypedResultShapeBuilder<S, F> {
    pub fn new() -> Self {
        Self {
            inner: ResultShapeBuilder::new(),
            _schema: PhantomData,
        }
    }

    pub fn field<P: TypedProjectableField<Schema = S>>(mut self) -> Self {
        self.inner = self
            .inner
            .field(result_shape_field_for::<P>(P::default_delivered_name()));
        self
    }

    pub fn field_as<P: TypedProjectableField<Schema = S>>(
        mut self,
        delivered_name: impl Into<String>,
    ) -> Self {
        let delivered_name = DeliveredFieldName::new(delivered_name)
            .expect("typed delivered field alias must be non-empty");
        self.inner = self
            .inner
            .field(result_shape_field_for::<P>(delivered_name.as_str()));
        self
    }

    pub fn build(self) -> Result<TypedAuthoredResultShape<S, F>, AuthoringError> {
        Ok(TypedAuthoredResultShape {
            inner: self.inner.build()?,
            _schema: PhantomData,
        })
    }
}

impl<S: TypedSchemaRoot, F: TypedResultShapeSurfaceFamily> Default
    for TypedResultShapeBuilder<S, F>
{
    fn default() -> Self {
        Self::new()
    }
}
