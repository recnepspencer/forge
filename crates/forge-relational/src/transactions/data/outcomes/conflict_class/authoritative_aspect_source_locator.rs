use forge_foundational::facade::{
    AspectFieldLocator, BoundarySourceLocator, CanonicalFieldPath, FieldKey,
};

pub(super) fn source_locator_aspect_label(locator: &BoundarySourceLocator) -> String {
    match locator {
        BoundarySourceLocator::Aspect(aspect) => aspect.aspect_key().as_str().to_string(),
        BoundarySourceLocator::AspectField(field) => {
            field.aspect().aspect_key().as_str().to_string()
        }
        BoundarySourceLocator::BoundaryArtifact(artifact) => format!("{artifact:?}"),
    }
}

pub(super) fn source_locator_field_label(locator: &BoundarySourceLocator) -> Option<String> {
    match locator {
        BoundarySourceLocator::AspectField(field) => {
            Some(field_path_presentation_label(field.field_path()))
        }
        BoundarySourceLocator::Aspect(_) | BoundarySourceLocator::BoundaryArtifact(_) => None,
    }
}

pub(super) fn aspect_field_locator_field_label(locator: &AspectFieldLocator) -> String {
    field_path_presentation_label(locator.field_path())
}

fn field_path_presentation_label(path: &CanonicalFieldPath) -> String {
    path.fields()
        .iter()
        .map(FieldKey::as_str)
        .collect::<Vec<_>>()
        .join(".")
}
