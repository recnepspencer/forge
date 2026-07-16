use worth_query::facade::foundation::WorthQueryConsumedProjectionAuthority;

use super::{WorthUiQueryMeasurementFactFamily, WorthUiQueryPrerequisiteEvidence};

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
    extent_bits: u32,
}

impl WorthUiQueryMeasurementFactObservation {
    pub(crate) fn from_query_authority(
        prerequisites: WorthUiQueryPrerequisiteEvidence,
        authority: &WorthQueryConsumedProjectionAuthority,
    ) -> Result<Box<[Self]>, WorthUiQueryMeasurementFactObservationError> {
        let _ = prerequisites;
        let mut observations = Vec::new();

        if authority.contract().fact_families().iter().any(|family| {
            family.kind().as_str() == "display_field" || family.kind().as_str() == "derived_field"
        }) {
            let family = WorthUiQueryMeasurementFactFamily::ScrollContentExtent;
            let extent_bits = extract_single_extent_bits(authority, family)?;
            observations.push(Self {
                family,
                extent_bits,
            });
        }

        Ok(observations.into_boxed_slice())
    }

    pub fn family(&self) -> WorthUiQueryMeasurementFactFamily {
        self.family
    }

    pub fn extent(&self) -> f32 {
        f32::from_bits(self.extent_bits)
    }
}

fn extract_single_extent_bits(
    authority: &WorthQueryConsumedProjectionAuthority,
    family: WorthUiQueryMeasurementFactFamily,
) -> Result<u32, WorthUiQueryMeasurementFactObservationError> {
    let mut observed_bits = None;
    for fact in authority
        .facts()
        .display_fields()
        .iter()
        .chain(authority.facts().derived_fields().iter())
    {
        let bits = fact.as_float32().map(|value| value.bits()).map_err(|_| {
            WorthUiQueryMeasurementFactObservationError::UnsupportedObservedValue(family)
        })?;
        match observed_bits {
            None => observed_bits = Some(bits),
            Some(existing) if existing == bits => {}
            Some(_) => {
                return Err(
                    WorthUiQueryMeasurementFactObservationError::AmbiguousObservedValue(family),
                );
            }
        }
    }

    observed_bits.ok_or(WorthUiQueryMeasurementFactObservationError::MissingObservedValue(family))
}
