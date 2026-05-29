use forge_foundational::facade::{
    AspectFieldLocator, AspectKey, BoundarySourceLocator, CanonicalFieldPath, FieldKey,
    LocatorAuthority,
};

pub(super) fn authoritative_aspect_field_source_locator(
    aspect_key: AspectKey,
    field: FieldKey,
) -> BoundarySourceLocator {
    BoundarySourceLocator::aspect_field(AspectFieldLocator::new(
        LocatorAuthority::SupportOnly,
        aspect_key,
        CanonicalFieldPath::single(field),
    ))
}

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

fn field_path_presentation_label(path: &CanonicalFieldPath) -> String {
    path.fields()
        .iter()
        .map(FieldKey::as_str)
        .collect::<Vec<_>>()
        .join(".")
}
