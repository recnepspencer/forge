use std::marker::PhantomData;

use super::{AuthoredResultShapeField, AuthoringError, RawAuthoredResultShape};

pub trait ResultShapeAuthoringFamily: Clone + std::fmt::Debug + Eq + PartialEq {
    fn initialize() -> RawAuthoredResultShape;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthoredResultShape<F: ResultShapeAuthoringFamily> {
    raw: RawAuthoredResultShape,
    _family: PhantomData<F>,
}

impl<F: ResultShapeAuthoringFamily> AuthoredResultShape<F> {
    pub fn into_raw(self) -> RawAuthoredResultShape {
        self.raw
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResultShapeBuilder<F: ResultShapeAuthoringFamily> {
    shape: RawAuthoredResultShape,
    _family: PhantomData<F>,
}

impl<F: ResultShapeAuthoringFamily> ResultShapeBuilder<F> {
    pub fn new() -> Self {
        Self {
            shape: F::initialize(),
            _family: PhantomData,
        }
    }

    pub fn field(mut self, field: AuthoredResultShapeField) -> Self {
        self.shape = self.shape.with_field(field);
        self
    }

    pub fn build(self) -> Result<AuthoredResultShape<F>, AuthoringError> {
        if self.shape.fields().is_empty() {
            return Err(AuthoringError::EmptyResultShapeFieldSet);
        }

        Ok(AuthoredResultShape {
            raw: self.shape,
            _family: PhantomData,
        })
    }
}

impl<F: ResultShapeAuthoringFamily> Default for ResultShapeBuilder<F> {
    fn default() -> Self {
        Self::new()
    }
}
