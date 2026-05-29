use serde_json::{Number as ExternalHarnessJsonNumber, Value as ExternalHarnessJson};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum CertificationArtifactProjectionValue {
    Null,
    Bool(bool),
    Unsigned(u64),
    String(String),
    Array(Vec<CertificationArtifactProjectionValue>),
    Object(Vec<(&'static str, CertificationArtifactProjectionValue)>),
    DynamicObject(Vec<(String, CertificationArtifactProjectionValue)>),
}

impl CertificationArtifactProjectionValue {
    pub(super) fn into_external_harness_json(self) -> ExternalHarnessJson {
        match self {
            Self::Null => ExternalHarnessJson::Null,
            Self::Bool(value) => ExternalHarnessJson::Bool(value),
            Self::Unsigned(value) => {
                ExternalHarnessJson::Number(ExternalHarnessJsonNumber::from(value))
            }
            Self::String(value) => ExternalHarnessJson::String(value),
            Self::Array(values) => ExternalHarnessJson::Array(
                values
                    .into_iter()
                    .map(Self::into_external_harness_json)
                    .collect(),
            ),
            Self::Object(fields) => ExternalHarnessJson::Object(
                fields
                    .into_iter()
                    .map(|(field, value)| (field.to_string(), value.into_external_harness_json()))
                    .collect(),
            ),
            Self::DynamicObject(fields) => ExternalHarnessJson::Object(
                fields
                    .into_iter()
                    .map(|(field, value)| (field, value.into_external_harness_json()))
                    .collect(),
            ),
        }
    }
}

pub(super) fn artifact_object(
    fields: impl IntoIterator<Item = (&'static str, CertificationArtifactProjectionValue)>,
) -> CertificationArtifactProjectionValue {
    CertificationArtifactProjectionValue::Object(fields.into_iter().collect())
}

pub(super) fn dynamic_artifact_object(
    fields: impl IntoIterator<Item = (String, CertificationArtifactProjectionValue)>,
) -> CertificationArtifactProjectionValue {
    CertificationArtifactProjectionValue::DynamicObject(fields.into_iter().collect())
}

pub(super) fn artifact_field(
    field: &'static str,
    value: CertificationArtifactProjectionValue,
) -> (&'static str, CertificationArtifactProjectionValue) {
    (field, value)
}

pub(super) fn bool_field(
    field: &'static str,
    value: bool,
) -> (&'static str, CertificationArtifactProjectionValue) {
    (field, CertificationArtifactProjectionValue::Bool(value))
}

pub(super) fn string_field(
    field: &'static str,
    value: String,
) -> (&'static str, CertificationArtifactProjectionValue) {
    (field, CertificationArtifactProjectionValue::String(value))
}

pub(super) fn string_array_field(
    field: &'static str,
    values: impl IntoIterator<Item = String>,
) -> (&'static str, CertificationArtifactProjectionValue) {
    (
        field,
        CertificationArtifactProjectionValue::Array(
            values
                .into_iter()
                .map(CertificationArtifactProjectionValue::String)
                .collect(),
        ),
    )
}

pub(super) fn usize_field(
    field: &'static str,
    value: usize,
) -> (&'static str, CertificationArtifactProjectionValue) {
    u64_field(field, value as u64)
}

pub(super) fn u64_field(
    field: &'static str,
    value: u64,
) -> (&'static str, CertificationArtifactProjectionValue) {
    (field, CertificationArtifactProjectionValue::Unsigned(value))
}

pub(super) fn optional_u64_field(
    field: &'static str,
    value: Option<u64>,
) -> (&'static str, CertificationArtifactProjectionValue) {
    (
        field,
        value
            .map(CertificationArtifactProjectionValue::Unsigned)
            .unwrap_or(CertificationArtifactProjectionValue::Null),
    )
}

pub(super) fn optional_usize_field(
    field: &'static str,
    value: Option<usize>,
) -> (&'static str, CertificationArtifactProjectionValue) {
    optional_u64_field(field, value.map(|number| number as u64))
}

pub(super) fn optional_string_field(
    field: &'static str,
    value: Option<String>,
) -> (&'static str, CertificationArtifactProjectionValue) {
    (
        field,
        value
            .map(CertificationArtifactProjectionValue::String)
            .unwrap_or(CertificationArtifactProjectionValue::Null),
    )
}
