use forge_harness::facade::HarnessSummaryProjection;

pub(super) type WorkflowArtifactProjection = HarnessSummaryProjection;

pub(super) fn workflow_artifact_object(
    fields: impl IntoIterator<Item = (&'static str, WorkflowArtifactProjection)>,
) -> WorkflowArtifactProjection {
    WorkflowArtifactProjection::object(fields)
}

pub(super) fn dynamic_workflow_artifact_object(
    fields: impl IntoIterator<Item = (String, WorkflowArtifactProjection)>,
) -> WorkflowArtifactProjection {
    WorkflowArtifactProjection::object(fields)
}

pub(super) fn workflow_artifact_field(
    field: &'static str,
    value: WorkflowArtifactProjection,
) -> (&'static str, WorkflowArtifactProjection) {
    (field, value)
}

pub(super) fn bool_field(
    field: &'static str,
    value: bool,
) -> (&'static str, WorkflowArtifactProjection) {
    (field, WorkflowArtifactProjection::Bool(value))
}

pub(super) fn string_field(
    field: &'static str,
    value: String,
) -> (&'static str, WorkflowArtifactProjection) {
    (field, WorkflowArtifactProjection::String(value))
}

pub(super) fn string_array_field(
    field: &'static str,
    values: impl IntoIterator<Item = String>,
) -> (&'static str, WorkflowArtifactProjection) {
    (
        field,
        WorkflowArtifactProjection::Array(
            values
                .into_iter()
                .map(WorkflowArtifactProjection::String)
                .collect(),
        ),
    )
}

pub(super) fn usize_field(
    field: &'static str,
    value: usize,
) -> (&'static str, WorkflowArtifactProjection) {
    u64_field(field, value as u64)
}

pub(super) fn u64_field(
    field: &'static str,
    value: u64,
) -> (&'static str, WorkflowArtifactProjection) {
    (field, WorkflowArtifactProjection::Unsigned(value))
}

pub(super) fn optional_u64_field(
    field: &'static str,
    value: Option<u64>,
) -> (&'static str, WorkflowArtifactProjection) {
    (
        field,
        value
            .map(WorkflowArtifactProjection::Unsigned)
            .unwrap_or(WorkflowArtifactProjection::Null),
    )
}

pub(super) fn optional_usize_field(
    field: &'static str,
    value: Option<usize>,
) -> (&'static str, WorkflowArtifactProjection) {
    optional_u64_field(field, value.map(|number| number as u64))
}

pub(super) fn optional_string_field(
    field: &'static str,
    value: Option<String>,
) -> (&'static str, WorkflowArtifactProjection) {
    (
        field,
        value
            .map(WorkflowArtifactProjection::String)
            .unwrap_or(WorkflowArtifactProjection::Null),
    )
}

pub(super) fn workflow_artifact_bool_field(
    projection: &WorkflowArtifactProjection,
    requested_field: &str,
) -> Option<bool> {
    let WorkflowArtifactProjection::Object(fields) = projection else {
        return None;
    };
    fields.get(requested_field).and_then(|value| match value {
        WorkflowArtifactProjection::Bool(value) => Some(*value),
        _ => None,
    })
}
