use crate::ForgeQueryEvidenceIdentity;

use super::error::{EvidenceReportError, EvidenceReportErrorKind};
use super::participation::EvidenceReportFieldParticipation;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceReportField {
    name: String,
    value: EvidenceReportFieldValue,
    participation: EvidenceReportFieldParticipation,
}

impl EvidenceReportField {
    pub(crate) fn new(
        name: impl Into<String>,
        value: EvidenceReportFieldValue,
        participation: EvidenceReportFieldParticipation,
    ) -> Result<Self, EvidenceReportError> {
        let name = name.into();
        validate_field_name(&name)?;
        Ok(Self {
            name,
            value,
            participation,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn kind(&self) -> EvidenceReportFieldKind {
        self.value.kind()
    }

    pub fn value(&self) -> &EvidenceReportFieldValue {
        &self.value
    }

    pub fn participation(&self) -> EvidenceReportFieldParticipation {
        self.participation
    }

    pub fn as_shape(&self) -> Option<&str> {
        match &self.value {
            EvidenceReportFieldValue::Shape(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_value(&self) -> Option<&str> {
        match &self.value {
            EvidenceReportFieldValue::Value(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self.value {
            EvidenceReportFieldValue::Bool(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_usize(&self) -> Option<usize> {
        match self.value {
            EvidenceReportFieldValue::Usize(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_evidence_identity(&self) -> Option<&ForgeQueryEvidenceIdentity> {
        match &self.value {
            EvidenceReportFieldValue::EvidenceIdentity(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_value_sequence(&self) -> Option<&[String]> {
        match &self.value {
            EvidenceReportFieldValue::ValueSequence(value) => Some(value),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EvidenceReportFieldValue {
    Shape(String),
    Value(String),
    Bool(bool),
    Usize(usize),
    EvidenceIdentity(ForgeQueryEvidenceIdentity),
    ValueSequence(Vec<String>),
    EvidenceIdentitySequence(Vec<ForgeQueryEvidenceIdentity>),
    OptionalValue(Option<String>),
    OptionalEvidenceIdentity(Option<ForgeQueryEvidenceIdentity>),
}

impl EvidenceReportFieldValue {
    pub fn kind(&self) -> EvidenceReportFieldKind {
        match self {
            Self::Shape(_) => EvidenceReportFieldKind::Shape,
            Self::Value(_) => EvidenceReportFieldKind::Value,
            Self::Bool(_) => EvidenceReportFieldKind::Bool,
            Self::Usize(_) => EvidenceReportFieldKind::Usize,
            Self::EvidenceIdentity(_) => EvidenceReportFieldKind::EvidenceIdentity,
            Self::ValueSequence(_) => EvidenceReportFieldKind::ValueSequence,
            Self::EvidenceIdentitySequence(_) => EvidenceReportFieldKind::EvidenceIdentitySequence,
            Self::OptionalValue(_) => EvidenceReportFieldKind::OptionalValue,
            Self::OptionalEvidenceIdentity(_) => EvidenceReportFieldKind::OptionalEvidenceIdentity,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum EvidenceReportFieldKind {
    Shape,
    Value,
    Bool,
    Usize,
    EvidenceIdentity,
    ValueSequence,
    EvidenceIdentitySequence,
    OptionalValue,
    OptionalEvidenceIdentity,
}

impl EvidenceReportFieldKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Shape => "shape",
            Self::Value => "value",
            Self::Bool => "bool",
            Self::Usize => "usize",
            Self::EvidenceIdentity => "evidence-identity",
            Self::ValueSequence => "value-sequence",
            Self::EvidenceIdentitySequence => "evidence-identity-sequence",
            Self::OptionalValue => "optional-value",
            Self::OptionalEvidenceIdentity => "optional-evidence-identity",
        }
    }
}

fn validate_field_name(name: &str) -> Result<(), EvidenceReportError> {
    if name.is_empty() {
        return Err(EvidenceReportError::new(
            EvidenceReportErrorKind::EmptyFieldName,
            "evidence report field name must not be empty",
        ));
    }

    if !name
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-')
    {
        return Err(EvidenceReportError::new(
            EvidenceReportErrorKind::InvalidFieldName,
            format!("invalid evidence report field name `{name}`"),
        ));
    }

    Ok(())
}
