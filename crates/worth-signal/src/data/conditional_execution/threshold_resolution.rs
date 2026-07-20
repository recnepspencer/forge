use worth_foundational::facade::AspectValue;

use super::{
    InstalledSignalConditionDecision, SignalDeltaThresholdContract, SignalThresholdBoundary,
    SignalThresholdComparisonDomain, SignalThresholdValueFamily,
};
use crate::data::error::SignalError;

/// Evaluates a typed semantic delta. Signal owns the comparison law; bridge
/// adapters may supply only contract-validated prior and current values.
pub fn resolve_signal_delta_threshold(
    contract: &SignalDeltaThresholdContract,
    previous: Option<&AspectValue>,
    current: &AspectValue,
) -> Result<InstalledSignalConditionDecision, SignalError> {
    let Some(previous) = previous else {
        return Ok(InstalledSignalConditionDecision::Suppressed);
    };
    let eligible = match contract.comparison_domain() {
        SignalThresholdComparisonDomain::AbsoluteDifference => {
            absolute_difference_meets(contract, previous, current)?
        }
        SignalThresholdComparisonDomain::RelativeRatio => {
            relative_ratio_meets(contract, previous, current)?
        }
    };
    Ok(if eligible {
        InstalledSignalConditionDecision::Eligible
    } else {
        InstalledSignalConditionDecision::Suppressed
    })
}

fn absolute_difference_meets(
    contract: &SignalDeltaThresholdContract,
    previous: &AspectValue,
    current: &AspectValue,
) -> Result<bool, SignalError> {
    match contract.value_family() {
        SignalThresholdValueFamily::Integer => {
            let difference = integer_absolute_difference(previous, current)?;
            let threshold = nonnegative_integer(contract.threshold())?;
            Ok(compare_u128(contract.boundary(), difference, threshold))
        }
        SignalThresholdValueFamily::Float32 | SignalThresholdValueFamily::Float64 => {
            let difference = (finite_float(current)? - finite_float(previous)?).abs();
            Ok(compare_f64(
                contract.boundary(),
                difference,
                finite_float(contract.threshold())?,
            ))
        }
    }
}

fn relative_ratio_meets(
    contract: &SignalDeltaThresholdContract,
    previous: &AspectValue,
    current: &AspectValue,
) -> Result<bool, SignalError> {
    let previous = finite_numeric(previous)?;
    let current = finite_numeric(current)?;
    let delta = (current - previous).abs();
    let denominator = previous.abs();
    let ratio = if denominator == 0.0 {
        if delta == 0.0 {
            0.0
        } else {
            f64::INFINITY
        }
    } else {
        delta / denominator
    };
    Ok(compare_f64(
        contract.boundary(),
        ratio,
        finite_numeric(contract.threshold())?,
    ))
}

fn integer_absolute_difference(
    previous: &AspectValue,
    current: &AspectValue,
) -> Result<u128, SignalError> {
    match (signed_integer(previous), signed_integer(current)) {
        (Some(previous), Some(current)) => Ok(previous.abs_diff(current)),
        _ => match (unsigned_integer(previous), unsigned_integer(current)) {
            (Some(previous), Some(current)) => Ok(previous.abs_diff(current)),
            _ => Err(type_mismatch()),
        },
    }
}

fn nonnegative_integer(value: &AspectValue) -> Result<u128, SignalError> {
    signed_integer(value)
        .and_then(|value| u128::try_from(value).ok())
        .or_else(|| unsigned_integer(value))
        .ok_or_else(type_mismatch)
}

fn signed_integer(value: &AspectValue) -> Option<i128> {
    match value {
        AspectValue::Int8(value) => Some(i128::from(*value)),
        AspectValue::Int16(value) => Some(i128::from(*value)),
        AspectValue::Int32(value) => Some(i128::from(*value)),
        AspectValue::Int64(value) => Some(i128::from(*value)),
        _ => None,
    }
}

fn unsigned_integer(value: &AspectValue) -> Option<u128> {
    match value {
        AspectValue::UInt8(value) => Some(u128::from(*value)),
        AspectValue::UInt16(value) => Some(u128::from(*value)),
        AspectValue::UInt32(value) => Some(u128::from(*value)),
        AspectValue::UInt64(value) => Some(u128::from(*value)),
        _ => None,
    }
}

fn finite_float(value: &AspectValue) -> Result<f64, SignalError> {
    match value {
        AspectValue::Float32(value) => finite(f64::from(value.as_f32())),
        AspectValue::Float64(value) => finite(value.as_f64()),
        _ => Err(type_mismatch()),
    }
}

fn finite_numeric(value: &AspectValue) -> Result<f64, SignalError> {
    signed_integer(value)
        .map(|value| value as f64)
        .or_else(|| unsigned_integer(value).map(|value| value as f64))
        .map(Ok)
        .unwrap_or_else(|| finite_float(value))
}

fn finite(value: f64) -> Result<f64, SignalError> {
    value
        .is_finite()
        .then_some(value)
        .ok_or_else(|| SignalError::invalid_input("semantic threshold observations must be finite"))
}

fn compare_u128(boundary: SignalThresholdBoundary, value: u128, threshold: u128) -> bool {
    match boundary {
        SignalThresholdBoundary::Inclusive => value >= threshold,
        SignalThresholdBoundary::Exclusive => value > threshold,
    }
}

fn compare_f64(boundary: SignalThresholdBoundary, value: f64, threshold: f64) -> bool {
    match boundary {
        SignalThresholdBoundary::Inclusive => value >= threshold,
        SignalThresholdBoundary::Exclusive => value > threshold,
    }
}

fn type_mismatch() -> SignalError {
    SignalError::invalid_input("semantic threshold values drifted from their installed family")
}

#[cfg(test)]
mod tests {
    use worth_foundational::facade::{AspectValue, CanonicalF64};

    use super::*;

    #[test]
    fn first_observation_establishes_baseline_without_claiming_eligibility() {
        let contract = float_contract(0.5, SignalThresholdBoundary::Inclusive);
        assert_eq!(
            resolve_signal_delta_threshold(
                &contract,
                None,
                &AspectValue::Float64(CanonicalF64::from_f64(10.0)),
            )
            .unwrap(),
            InstalledSignalConditionDecision::Suppressed
        );
    }

    #[test]
    fn typed_float_boundary_distinguishes_inclusive_and_exclusive() {
        let previous = AspectValue::Float64(CanonicalF64::from_f64(10.0));
        let current = AspectValue::Float64(CanonicalF64::from_f64(10.5));
        assert_eq!(
            resolve_signal_delta_threshold(
                &float_contract(0.5, SignalThresholdBoundary::Inclusive),
                Some(&previous),
                &current,
            )
            .unwrap(),
            InstalledSignalConditionDecision::Eligible
        );
        assert_eq!(
            resolve_signal_delta_threshold(
                &float_contract(0.5, SignalThresholdBoundary::Exclusive),
                Some(&previous),
                &current,
            )
            .unwrap(),
            InstalledSignalConditionDecision::Suppressed
        );
    }

    #[test]
    fn integer_absolute_difference_does_not_round_through_float() {
        let contract = SignalDeltaThresholdContract::new(
            AspectValue::UInt64(1),
            "worth.tests.units.count",
            SignalThresholdValueFamily::Integer,
            SignalThresholdComparisonDomain::AbsoluteDifference,
            SignalThresholdBoundary::Inclusive,
        );
        assert_eq!(
            resolve_signal_delta_threshold(
                &contract,
                Some(&AspectValue::UInt64(u64::MAX - 1)),
                &AspectValue::UInt64(u64::MAX),
            )
            .unwrap(),
            InstalledSignalConditionDecision::Eligible
        );
    }

    fn float_contract(
        threshold: f64,
        boundary: SignalThresholdBoundary,
    ) -> SignalDeltaThresholdContract {
        SignalDeltaThresholdContract::new(
            AspectValue::Float64(CanonicalF64::from_f64(threshold)),
            "worth.tests.units.millimeters",
            SignalThresholdValueFamily::Float64,
            SignalThresholdComparisonDomain::AbsoluteDifference,
            boundary,
        )
    }
}
