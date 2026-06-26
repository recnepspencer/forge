use super::MeasurementValue;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MeasurementConstraint {
    Unconstrained,
    AtLeast(MeasurementValue),
    AtMost(MeasurementValue),
    Between {
        minimum: MeasurementValue,
        maximum: MeasurementValue,
    },
}

impl MeasurementConstraint {
    pub fn unconstrained() -> Self {
        Self::Unconstrained
    }

    pub fn at_least(value: MeasurementValue) -> Self {
        Self::AtLeast(value)
    }

    pub fn at_most(value: MeasurementValue) -> Self {
        Self::AtMost(value)
    }

    pub fn between(minimum: MeasurementValue, maximum: MeasurementValue) -> Self {
        Self::Between { minimum, maximum }
    }

    pub(crate) fn digest_basis(&self) -> String {
        match self {
            Self::Unconstrained => "unconstrained".to_owned(),
            Self::AtLeast(value) => format!("at_least:{}", value.digest_basis()),
            Self::AtMost(value) => format!("at_most:{}", value.digest_basis()),
            Self::Between { minimum, maximum } => {
                format!(
                    "between:{}:{}",
                    minimum.digest_basis(),
                    maximum.digest_basis()
                )
            }
        }
    }

    pub(crate) fn has_unitless_value(&self) -> bool {
        match self {
            Self::Unconstrained => false,
            Self::AtLeast(value) | Self::AtMost(value) => value.is_unitless(),
            Self::Between { minimum, maximum } => minimum.is_unitless() || maximum.is_unitless(),
        }
    }

    pub(crate) fn has_invalid_bounds(&self) -> bool {
        match self {
            Self::Unconstrained | Self::AtLeast(_) | Self::AtMost(_) => false,
            Self::Between { minimum, maximum } => {
                if minimum.is_unitless() || maximum.is_unitless() {
                    return false;
                }
                minimum
                    .comparable_order_key()
                    .zip(maximum.comparable_order_key())
                    .is_none_or(|(minimum, maximum)| {
                        minimum.is_not_ordered_before_or_equal_to(maximum)
                    })
            }
        }
    }
}
