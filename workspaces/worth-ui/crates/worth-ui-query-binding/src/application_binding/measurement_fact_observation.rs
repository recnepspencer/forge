use worth_foundational::CanonicalF32;
use worth_query::facade::{installed::operation, read};

use super::WorthUiQueryMeasurementFactFamily;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiQueryMeasurementFactObservationError {
    NativeAccess(Box<operation::WorthQueryNativeAccessDenial>),
    NativeRefinement(Box<read::ConsumedNativeRefinementDenial>),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiQueryMeasurementFactObservation {
    family: WorthUiQueryMeasurementFactFamily,
    extent: CanonicalF32,
}

/// Exact work performed while refining Query-native facts into UI measurement
/// observations. Query execution and fact extraction retain separate counters.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiQueryMeasurementRefinementCounters {
    declared_measurement_fact_count: usize,
    projected_measurement_fact_count: usize,
    refinement_attempt_count: usize,
    admitted_observation_count: usize,
}

impl WorthUiQueryMeasurementFactObservation {
    pub(crate) fn from_native_access(
        access: &operation::WorthQueryNativeFieldAccess<'_>,
    ) -> Result<
        (Box<[Self]>, WorthUiQueryMeasurementRefinementCounters),
        WorthUiQueryMeasurementFactObservationError,
    > {
        let family = WorthUiQueryMeasurementFactFamily::ScrollContentExtent;
        let extent = access.fact().as_float32().copied().map_err(|denial| {
            WorthUiQueryMeasurementFactObservationError::NativeRefinement(Box::new(denial))
        })?;
        Ok((
            Box::new([Self { family, extent }]),
            WorthUiQueryMeasurementRefinementCounters {
                declared_measurement_fact_count: 1,
                projected_measurement_fact_count: 1,
                refinement_attempt_count: 1,
                admitted_observation_count: 1,
            },
        ))
    }

    pub fn family(&self) -> WorthUiQueryMeasurementFactFamily {
        self.family
    }

    pub fn extent(&self) -> CanonicalF32 {
        self.extent
    }
}

impl WorthUiQueryMeasurementRefinementCounters {
    pub fn declared_measurement_fact_count(self) -> usize {
        self.declared_measurement_fact_count
    }

    pub fn projected_measurement_fact_count(self) -> usize {
        self.projected_measurement_fact_count
    }

    pub fn refinement_attempt_count(self) -> usize {
        self.refinement_attempt_count
    }

    pub fn admitted_observation_count(self) -> usize {
        self.admitted_observation_count
    }
}
