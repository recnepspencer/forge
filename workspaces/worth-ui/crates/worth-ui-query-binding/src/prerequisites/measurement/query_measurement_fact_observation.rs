use worth_foundational::CanonicalF32;
use worth_query::facade::foundation::WorthQueryConsumedProjectionAuthority;

use super::WorthUiQueryMeasurementFactFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryMeasurementFactObservationError {
    ProjectionConsumptionNotAdmitted,
    MissingObservedValue(WorthUiQueryMeasurementFactFamily),
    AmbiguousObservedValue(WorthUiQueryMeasurementFactFamily),
    UnsupportedObservedValue(WorthUiQueryMeasurementFactFamily),
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
    pub(crate) fn from_query_authority(
        authority: &WorthQueryConsumedProjectionAuthority,
    ) -> Result<
        (Box<[Self]>, WorthUiQueryMeasurementRefinementCounters),
        WorthUiQueryMeasurementFactObservationError,
    > {
        let mut observations = Vec::new();
        let declared_measurement_fact_count = authority
            .consumer_contract()
            .requested_facts()
            .filter(|request| {
                request.kind().as_str() == "display_field"
                    || request.kind().as_str() == "derived_field"
            })
            .count();
        let projected_measurement_fact_count =
            authority.facts().display_fields().len() + authority.facts().derived_fields().len();

        if authority.contract().fact_families().iter().any(|family| {
            family.kind().as_str() == "display_field" || family.kind().as_str() == "derived_field"
        }) {
            let family = WorthUiQueryMeasurementFactFamily::ScrollContentExtent;
            let extent = extract_single_extent(authority, family)?;
            observations.push(Self { family, extent });
        }

        let admitted_observation_count = observations.len();
        Ok((
            observations.into_boxed_slice(),
            WorthUiQueryMeasurementRefinementCounters {
                declared_measurement_fact_count,
                projected_measurement_fact_count,
                refinement_attempt_count: projected_measurement_fact_count,
                admitted_observation_count,
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

fn extract_single_extent(
    authority: &WorthQueryConsumedProjectionAuthority,
    family: WorthUiQueryMeasurementFactFamily,
) -> Result<CanonicalF32, WorthUiQueryMeasurementFactObservationError> {
    let mut observed = None;
    for fact in authority
        .facts()
        .display_fields()
        .iter()
        .chain(authority.facts().derived_fields().iter())
    {
        let value = fact.as_float32().copied().map_err(|_| {
            WorthUiQueryMeasurementFactObservationError::UnsupportedObservedValue(family)
        })?;
        match observed {
            None => observed = Some(value),
            Some(existing) if existing == value => {}
            Some(_) => {
                return Err(
                    WorthUiQueryMeasurementFactObservationError::AmbiguousObservedValue(family),
                );
            }
        }
    }

    observed.ok_or(WorthUiQueryMeasurementFactObservationError::MissingObservedValue(family))
}
