use serde::{Deserialize, Serialize};

use crate::data::performance::{
    ResolvedExecutionStrategy, ResolvedMaintenanceStrategy, ResolvedPerformancePolicy,
};

pub trait CanonicalForm {}

pub trait LoweredForm {}

pub trait ResolvedForm {}

pub trait DeltaForm {}

pub trait SummaryForm {}

impl ResolvedForm for ResolvedExecutionStrategy {}
impl ResolvedForm for ResolvedMaintenanceStrategy {}
impl ResolvedForm for ResolvedPerformancePolicy {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesiredState<T> {
    value: T,
}

impl<T> DesiredState<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn into_inner(self) -> T {
        self.value
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SingleConsumer<T>(T);

impl<T> SingleConsumer<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> AsRef<T> for SingleConsumer<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for SingleConsumer<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
