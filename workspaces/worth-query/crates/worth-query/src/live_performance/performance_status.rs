use std::marker::PhantomData;

pub trait PerformanceStatus {
    const LABEL: &'static str;
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct VerifiedPerformance;

impl PerformanceStatus for VerifiedPerformance {
    const LABEL: &'static str = "verified";
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DebtPerformance;

impl PerformanceStatus for DebtPerformance {
    const LABEL: &'static str = "debt";
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ForbiddenPerformance;

impl PerformanceStatus for ForbiddenPerformance {
    const LABEL: &'static str = "forbidden";
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PerformanceStatusMarker<S: PerformanceStatus> {
    _marker: PhantomData<S>,
}

impl<S: PerformanceStatus> PerformanceStatusMarker<S> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    pub fn label(&self) -> &'static str {
        S::LABEL
    }
}

impl<S: PerformanceStatus> Default for PerformanceStatusMarker<S> {
    fn default() -> Self {
        Self::new()
    }
}
