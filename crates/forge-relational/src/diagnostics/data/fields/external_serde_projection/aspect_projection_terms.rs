use forge_foundational::facade::{
    AspectFieldLocator, AspectMask, AspectMaskLocator, AspectValueLocator, CanonicalBasisEntry,
    CanonicalBasisReadyArtifact, DiagnosticMask,
};

use super::projected_value::ExternalSerdeDiagnosticProjectionValue;

pub(super) fn aspect_field_locator_external_serde_projection(
    locator: &AspectFieldLocator,
) -> ExternalSerdeDiagnosticProjectionValue {
    object([
        ("locator_kind", string("aspect_field")),
        (
            "authority",
            string(format!("{:?}", locator.aspect().authority())),
        ),
        ("aspect_key", string(locator.aspect().aspect_key().as_str())),
        ("field_path", field_path_projection(locator.field_path())),
    ])
}

pub(super) fn aspect_value_locator_external_serde_projection(
    locator: &AspectValueLocator,
) -> ExternalSerdeDiagnosticProjectionValue {
    match locator {
        AspectValueLocator::WholeAspect(aspect) => object([
            ("locator_kind", string("whole_aspect")),
            ("authority", string(format!("{:?}", aspect.authority()))),
            ("aspect_key", string(aspect.aspect_key().as_str())),
        ]),
        AspectValueLocator::StructField(field) => object([
            ("locator_kind", string("struct_field")),
            (
                "authority",
                string(format!("{:?}", field.aspect().authority())),
            ),
            ("aspect_key", string(field.aspect().aspect_key().as_str())),
            ("field_path", field_path_projection(field.field_path())),
        ]),
    }
}

pub(super) fn diagnostic_mask_external_serde_projection(
    mask: &AspectMask<DiagnosticMask>,
) -> ExternalSerdeDiagnosticProjectionValue {
    if mask.is_whole_aspect() {
        return object([("mask_kind", string("whole_aspect"))]);
    }

    object([
        ("mask_kind", string("fields")),
        ("field_paths", field_paths_projection(mask.paths())),
    ])
}

pub(super) fn diagnostic_mask_locator_external_serde_projection(
    locator: &AspectMaskLocator<DiagnosticMask>,
) -> ExternalSerdeDiagnosticProjectionValue {
    object([
        ("locator_kind", string("diagnostic_mask")),
        ("authority", string(format!("{:?}", locator.authority()))),
        ("aspect_key", string(locator.aspect_key().as_str())),
        ("field_paths", field_paths_projection(locator.paths())),
    ])
}

pub(super) fn canonical_basis_external_serde_projection(
    basis: &CanonicalBasisReadyArtifact,
) -> ExternalSerdeDiagnosticProjectionValue {
    object([
        ("basis_kind", string("canonical_basis_ready")),
        ("domain", string(format!("{:?}", basis.payload().domain()))),
        ("version", string(basis.payload().version().as_str())),
        (
            "entry_count",
            unsigned(basis.payload().entries().len() as u64),
        ),
        (
            "entries",
            ExternalSerdeDiagnosticProjectionValue::Array(
                basis
                    .payload()
                    .entries()
                    .iter()
                    .map(canonical_basis_entry_external_serde_projection)
                    .collect(),
            ),
        ),
    ])
}

fn canonical_basis_entry_external_serde_projection(
    entry: &CanonicalBasisEntry,
) -> ExternalSerdeDiagnosticProjectionValue {
    object([
        ("domain", string(format!("{:?}", entry.domain()))),
        ("locus", string(format!("{:?}", entry.locus()))),
        ("kind", string(format!("{:?}", entry.kind()))),
        ("value", string(format!("{:?}", entry.value()))),
    ])
}

fn field_paths_projection(
    paths: &[forge_foundational::facade::CanonicalFieldPath],
) -> ExternalSerdeDiagnosticProjectionValue {
    ExternalSerdeDiagnosticProjectionValue::Array(paths.iter().map(field_path_projection).collect())
}

fn field_path_projection(
    path: &forge_foundational::facade::CanonicalFieldPath,
) -> ExternalSerdeDiagnosticProjectionValue {
    ExternalSerdeDiagnosticProjectionValue::Array(
        path.fields()
            .iter()
            .map(|field| string(field.as_str()))
            .collect(),
    )
}

fn object(
    fields: impl IntoIterator<Item = (&'static str, ExternalSerdeDiagnosticProjectionValue)>,
) -> ExternalSerdeDiagnosticProjectionValue {
    ExternalSerdeDiagnosticProjectionValue::Object(
        fields
            .into_iter()
            .map(|(key, value)| (key.to_string(), value))
            .collect(),
    )
}

fn string(value: impl Into<String>) -> ExternalSerdeDiagnosticProjectionValue {
    ExternalSerdeDiagnosticProjectionValue::String(value.into())
}

fn unsigned(value: u64) -> ExternalSerdeDiagnosticProjectionValue {
    ExternalSerdeDiagnosticProjectionValue::Unsigned(value)
}
