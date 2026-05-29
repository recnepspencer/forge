use serde_json::{Number as ExternalHarnessJsonNumber, Value as ExternalHarnessJson};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ExternalHarnessArtifactJson {
    Null,
    Bool(bool),
    Unsigned(u64),
    String(String),
    Array(Vec<ExternalHarnessArtifactJson>),
    Object(Vec<(&'static str, ExternalHarnessArtifactJson)>),
    DynamicObject(Vec<(String, ExternalHarnessArtifactJson)>),
}

impl ExternalHarnessArtifactJson {
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

pub(super) fn external_harness_artifact_object(
    fields: impl IntoIterator<Item = (&'static str, ExternalHarnessArtifactJson)>,
) -> ExternalHarnessArtifactJson {
    ExternalHarnessArtifactJson::Object(fields.into_iter().collect())
}

pub(super) fn dynamic_external_harness_artifact_object(
    fields: impl IntoIterator<Item = (String, ExternalHarnessArtifactJson)>,
) -> ExternalHarnessArtifactJson {
    ExternalHarnessArtifactJson::DynamicObject(fields.into_iter().collect())
}

pub(super) fn external_harness_artifact_field(
    field: &'static str,
    value: ExternalHarnessArtifactJson,
) -> (&'static str, ExternalHarnessArtifactJson) {
    (field, value)
}

pub(super) fn bool_field(
    field: &'static str,
    value: bool,
) -> (&'static str, ExternalHarnessArtifactJson) {
    (field, ExternalHarnessArtifactJson::Bool(value))
}

pub(super) fn string_field(
    field: &'static str,
    value: String,
) -> (&'static str, ExternalHarnessArtifactJson) {
    (field, ExternalHarnessArtifactJson::String(value))
}

pub(super) fn string_array_field(
    field: &'static str,
    values: impl IntoIterator<Item = String>,
) -> (&'static str, ExternalHarnessArtifactJson) {
    (
        field,
        ExternalHarnessArtifactJson::Array(
            values
                .into_iter()
                .map(ExternalHarnessArtifactJson::String)
                .collect(),
        ),
    )
}

pub(super) fn usize_field(
    field: &'static str,
    value: usize,
) -> (&'static str, ExternalHarnessArtifactJson) {
    u64_field(field, value as u64)
}

pub(super) fn u64_field(
    field: &'static str,
    value: u64,
) -> (&'static str, ExternalHarnessArtifactJson) {
    (field, ExternalHarnessArtifactJson::Unsigned(value))
}

pub(super) fn optional_u64_field(
    field: &'static str,
    value: Option<u64>,
) -> (&'static str, ExternalHarnessArtifactJson) {
    (
        field,
        value
            .map(ExternalHarnessArtifactJson::Unsigned)
            .unwrap_or(ExternalHarnessArtifactJson::Null),
    )
}

pub(super) fn optional_usize_field(
    field: &'static str,
    value: Option<usize>,
) -> (&'static str, ExternalHarnessArtifactJson) {
    optional_u64_field(field, value.map(|number| number as u64))
}

pub(super) fn optional_string_field(
    field: &'static str,
    value: Option<String>,
) -> (&'static str, ExternalHarnessArtifactJson) {
    (
        field,
        value
            .map(ExternalHarnessArtifactJson::String)
            .unwrap_or(ExternalHarnessArtifactJson::Null),
    )
}
