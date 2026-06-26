use crate::capability::{CapabilityIdError, MosaicSizingContractId};

use super::{MeasurementConstraint, MeasurementValue};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct NamedMeasurementToken {
    text: String,
}

impl NamedMeasurementToken {
    pub fn new(raw_text: impl AsRef<str>) -> Result<Self, CapabilityIdError> {
        let raw_text = raw_text.as_ref();
        MosaicSizingContractId::new(raw_text)?;
        Ok(Self {
            text: raw_text.to_owned(),
        })
    }

    pub fn as_str(&self) -> &str {
        &self.text
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NamedMeasurementDefinition {
    token: NamedMeasurementToken,
    value: MeasurementValue,
    constraint: MeasurementConstraint,
}

impl NamedMeasurementDefinition {
    pub fn new(
        token: NamedMeasurementToken,
        value: MeasurementValue,
        constraint: MeasurementConstraint,
    ) -> Self {
        Self {
            token,
            value,
            constraint,
        }
    }

    pub fn token(&self) -> &NamedMeasurementToken {
        &self.token
    }

    pub fn value(&self) -> &MeasurementValue {
        &self.value
    }

    pub fn constraint(&self) -> &MeasurementConstraint {
        &self.constraint
    }

    pub(crate) fn has_unitless_value_or_constraint(&self) -> bool {
        self.value.is_unitless() || self.constraint.has_unitless_value()
    }

    pub(crate) fn has_invalid_constraint_bounds(&self) -> bool {
        self.constraint.has_invalid_bounds()
    }

    pub(crate) fn digest_basis(&self) -> String {
        format!(
            "{}:{}:{}",
            self.token.as_str(),
            self.value.digest_basis(),
            self.constraint.digest_basis()
        )
    }
}
