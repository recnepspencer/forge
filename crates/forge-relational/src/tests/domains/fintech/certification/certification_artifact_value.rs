use serde_json::{Number, Value};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum CertificationArtifactValue {
    Null,
    Bool(bool),
    Unsigned(u64),
    String(String),
    Array(Vec<CertificationArtifactValue>),
    Object(Vec<(&'static str, CertificationArtifactValue)>),
    DynamicObject(Vec<(String, CertificationArtifactValue)>),
}

impl CertificationArtifactValue {
    pub(super) fn into_json(self) -> Value {
        match self {
            Self::Null => Value::Null,
            Self::Bool(value) => Value::Bool(value),
            Self::Unsigned(value) => Value::Number(Number::from(value)),
            Self::String(value) => Value::String(value),
            Self::Array(values) => Value::Array(values.into_iter().map(Self::into_json).collect()),
            Self::Object(fields) => Value::Object(
                fields
                    .into_iter()
                    .map(|(field, value)| (field.to_string(), value.into_json()))
                    .collect(),
            ),
            Self::DynamicObject(fields) => Value::Object(
                fields
                    .into_iter()
                    .map(|(field, value)| (field, value.into_json()))
                    .collect(),
            ),
        }
    }
}

pub(super) fn artifact_object(
    fields: impl IntoIterator<Item = (&'static str, CertificationArtifactValue)>,
) -> CertificationArtifactValue {
    CertificationArtifactValue::Object(fields.into_iter().collect())
}

pub(super) fn dynamic_artifact_object(
    fields: impl IntoIterator<Item = (String, CertificationArtifactValue)>,
) -> CertificationArtifactValue {
    CertificationArtifactValue::DynamicObject(fields.into_iter().collect())
}

pub(super) fn artifact_field(
    field: &'static str,
    value: CertificationArtifactValue,
) -> (&'static str, CertificationArtifactValue) {
    (field, value)
}

pub(super) fn bool_field(
    field: &'static str,
    value: bool,
) -> (&'static str, CertificationArtifactValue) {
    (field, CertificationArtifactValue::Bool(value))
}

pub(super) fn string_field(
    field: &'static str,
    value: String,
) -> (&'static str, CertificationArtifactValue) {
    (field, CertificationArtifactValue::String(value))
}

pub(super) fn string_array_field(
    field: &'static str,
    values: impl IntoIterator<Item = String>,
) -> (&'static str, CertificationArtifactValue) {
    (
        field,
        CertificationArtifactValue::Array(
            values
                .into_iter()
                .map(CertificationArtifactValue::String)
                .collect(),
        ),
    )
}

pub(super) fn usize_field(
    field: &'static str,
    value: usize,
) -> (&'static str, CertificationArtifactValue) {
    u64_field(field, value as u64)
}

pub(super) fn u64_field(
    field: &'static str,
    value: u64,
) -> (&'static str, CertificationArtifactValue) {
    (field, CertificationArtifactValue::Unsigned(value))
}

pub(super) fn optional_u64_field(
    field: &'static str,
    value: Option<u64>,
) -> (&'static str, CertificationArtifactValue) {
    (
        field,
        value
            .map(CertificationArtifactValue::Unsigned)
            .unwrap_or(CertificationArtifactValue::Null),
    )
}

pub(super) fn optional_usize_field(
    field: &'static str,
    value: Option<usize>,
) -> (&'static str, CertificationArtifactValue) {
    optional_u64_field(field, value.map(|number| number as u64))
}

pub(super) fn optional_string_field(
    field: &'static str,
    value: Option<String>,
) -> (&'static str, CertificationArtifactValue) {
    (
        field,
        value
            .map(CertificationArtifactValue::String)
            .unwrap_or(CertificationArtifactValue::Null),
    )
}
