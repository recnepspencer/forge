use worth_foundational::facade::{AspectValue, InternedString};
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
            family.kind().as_str() == "display_field"
                || family.kind().as_str() == "derived_scalar_field"
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
        .chain(authority.facts().derived_scalar_fields().iter())
    {
        let bits = scalar_extent_bits(fact.value())
            .ok_or(WorthUiQueryMeasurementFactObservationError::UnsupportedObservedValue(family))?;
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

fn scalar_extent_bits(value: &AspectValue) -> Option<u32> {
    match value {
        AspectValue::Int8(value) => Some((*value as f32).to_bits()),
        AspectValue::Int16(value) => Some((*value as f32).to_bits()),
        AspectValue::Int32(value) => Some((*value as f32).to_bits()),
        AspectValue::Int64(value) => Some((*value as f32).to_bits()),
        AspectValue::UInt8(value) => Some((*value as f32).to_bits()),
        AspectValue::UInt16(value) => Some((*value as f32).to_bits()),
        AspectValue::UInt32(value) => Some((*value as f32).to_bits()),
        AspectValue::UInt64(value) => Some((*value as f32).to_bits()),
        AspectValue::Float32(value) => Some(value.bits()),
        AspectValue::Float64(value) => Some((f64::from_bits(value.bits()) as f32).to_bits()),
        AspectValue::String(InternedString::Raw(value)) => {
            value.parse::<f32>().ok().map(f32::to_bits)
        }
        AspectValue::String(InternedString::Symbol(_)) => None,
        _ => None,
    }
}
