use serde_json::Number as ExternalHarnessPayloadNumber;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ExternalHarnessArtifactProjection {
    Null,
    Bool(bool),
    Unsigned(u64),
    String(String),
    Array(Vec<ExternalHarnessArtifactProjection>),
    Object(Vec<(&'static str, ExternalHarnessArtifactProjection)>),
    DynamicObject(Vec<(String, ExternalHarnessArtifactProjection)>),
}

impl ExternalHarnessArtifactProjection {
    pub(super) fn into_external_harness_payload(self) -> serde_json::value::Value {
        match self {
            Self::Null => serde_json::value::Value::Null,
            Self::Bool(value) => serde_json::value::Value::Bool(value),
            Self::Unsigned(value) => {
                serde_json::value::Value::Number(ExternalHarnessPayloadNumber::from(value))
            }
            Self::String(value) => serde_json::value::Value::String(value),
            Self::Array(values) => serde_json::value::Value::Array(
                values
                    .into_iter()
                    .map(Self::into_external_harness_payload)
                    .collect(),
            ),
            Self::Object(fields) => serde_json::value::Value::Object(
                fields
                    .into_iter()
                    .map(|(field, value)| {
                        (field.to_string(), value.into_external_harness_payload())
                    })
                    .collect(),
            ),
            Self::DynamicObject(fields) => serde_json::value::Value::Object(
                fields
                    .into_iter()
                    .map(|(field, value)| (field, value.into_external_harness_payload()))
                    .collect(),
            ),
        }
    }

    pub(super) fn object_bool_field(&self, requested_field: &str) -> Option<bool> {
        let Self::Object(fields) = self else {
            return None;
        };
        fields.iter().find_map(|(field, value)| {
            if *field == requested_field {
                match value {
                    Self::Bool(value) => Some(*value),
                    _ => None,
                }
            } else {
                None
            }
        })
    }
}

pub(super) fn external_harness_artifact_object(
    fields: impl IntoIterator<Item = (&'static str, ExternalHarnessArtifactProjection)>,
) -> ExternalHarnessArtifactProjection {
    ExternalHarnessArtifactProjection::Object(fields.into_iter().collect())
}

pub(super) fn dynamic_external_harness_artifact_object(
    fields: impl IntoIterator<Item = (String, ExternalHarnessArtifactProjection)>,
) -> ExternalHarnessArtifactProjection {
    ExternalHarnessArtifactProjection::DynamicObject(fields.into_iter().collect())
}

pub(super) fn external_harness_artifact_field(
    field: &'static str,
    value: ExternalHarnessArtifactProjection,
) -> (&'static str, ExternalHarnessArtifactProjection) {
    (field, value)
}

pub(super) fn bool_field(
    field: &'static str,
    value: bool,
) -> (&'static str, ExternalHarnessArtifactProjection) {
    (field, ExternalHarnessArtifactProjection::Bool(value))
}

pub(super) fn string_field(
    field: &'static str,
    value: String,
) -> (&'static str, ExternalHarnessArtifactProjection) {
    (field, ExternalHarnessArtifactProjection::String(value))
}

pub(super) fn string_array_field(
    field: &'static str,
    values: impl IntoIterator<Item = String>,
) -> (&'static str, ExternalHarnessArtifactProjection) {
    (
        field,
        ExternalHarnessArtifactProjection::Array(
            values
                .into_iter()
                .map(ExternalHarnessArtifactProjection::String)
                .collect(),
        ),
    )
}

pub(super) fn usize_field(
    field: &'static str,
    value: usize,
) -> (&'static str, ExternalHarnessArtifactProjection) {
    u64_field(field, value as u64)
}

pub(super) fn u64_field(
    field: &'static str,
    value: u64,
) -> (&'static str, ExternalHarnessArtifactProjection) {
    (field, ExternalHarnessArtifactProjection::Unsigned(value))
}

pub(super) fn optional_u64_field(
    field: &'static str,
    value: Option<u64>,
) -> (&'static str, ExternalHarnessArtifactProjection) {
    (
        field,
        value
            .map(ExternalHarnessArtifactProjection::Unsigned)
            .unwrap_or(ExternalHarnessArtifactProjection::Null),
    )
}

pub(super) fn optional_usize_field(
    field: &'static str,
    value: Option<usize>,
) -> (&'static str, ExternalHarnessArtifactProjection) {
    optional_u64_field(field, value.map(|number| number as u64))
}

pub(super) fn optional_string_field(
    field: &'static str,
    value: Option<String>,
) -> (&'static str, ExternalHarnessArtifactProjection) {
    (
        field,
        value
            .map(ExternalHarnessArtifactProjection::String)
            .unwrap_or(ExternalHarnessArtifactProjection::Null),
    )
}
