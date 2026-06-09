use super::super::super::request::{PrimitiveConstructionFamily, PrimitiveConstructionPhaseError};

pub(super) fn decode_positive_scalar(
    family: PrimitiveConstructionFamily,
    bits: u64,
    reason: &'static str,
) -> Result<f64, PrimitiveConstructionPhaseError> {
    let value = f64::from_bits(bits);
    if !value.is_finite() || value <= 0.0 {
        return Err(PrimitiveConstructionPhaseError::InvalidRequest { family, reason });
    }
    Ok(value)
}

pub(super) fn decode_non_negative_scalar(
    family: PrimitiveConstructionFamily,
    bits: u64,
    reason: &'static str,
) -> Result<f64, PrimitiveConstructionPhaseError> {
    let value = f64::from_bits(bits);
    if !value.is_finite() || value < 0.0 {
        return Err(PrimitiveConstructionPhaseError::InvalidRequest { family, reason });
    }
    Ok(value)
}

pub(super) fn admit_polygon_edge_count(
    family: PrimitiveConstructionFamily,
    count: u32,
) -> Result<u32, PrimitiveConstructionPhaseError> {
    if count < 3 {
        return Err(PrimitiveConstructionPhaseError::InvalidRequest {
            family,
            reason: "polygonal construction families require at least three edges",
        });
    }
    Ok(count)
}

pub(super) fn decode_positive_triplet(
    family: PrimitiveConstructionFamily,
    bits: [u64; 3],
    reason: &'static str,
) -> Result<[f64; 3], PrimitiveConstructionPhaseError> {
    let values = bits.map(f64::from_bits);
    if values
        .iter()
        .any(|value| !value.is_finite() || *value <= 0.0)
    {
        return Err(PrimitiveConstructionPhaseError::InvalidRequest { family, reason });
    }
    Ok(values)
}
