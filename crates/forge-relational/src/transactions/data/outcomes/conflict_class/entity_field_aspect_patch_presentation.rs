use super::EntityFieldAspectPatchDenial;

pub(super) fn entity_field_aspect_patch_denial_detail(
    denial: &EntityFieldAspectPatchDenial,
) -> String {
    match denial {
        EntityFieldAspectPatchDenial::MissingAspectPlan { kind_id } => {
            format!(
                "entity field aspect update requires a lowered aspect plan for kind {}",
                kind_id.0
            )
        }
        EntityFieldAspectPatchDenial::UndeclaredEntityAspectTarget { field_locator } => {
            format!(
                "entity field aspect update targets undeclared aspect {:?} at field path '{}'",
                field_locator.aspect().aspect_key(),
                aspect_field_locator_path_label(field_locator)
            )
        }
        EntityFieldAspectPatchDenial::EntityAspectFieldPathMismatch { field_locator } => {
            format!(
                "entity field aspect update targets aspect {:?} with a field path '{}' not admitted by that contract binding",
                field_locator.aspect().aspect_key(),
                aspect_field_locator_path_label(field_locator)
            )
        }
        EntityFieldAspectPatchDenial::UnsupportedNestedEntityFieldPath { path } => {
            let label = path
                .iter()
                .map(forge_foundational::facade::FieldKey::as_str)
                .collect::<Vec<_>>()
                .join(".");
            format!(
                "entity field aspect update targets unsupported nested contract field path '{}'",
                label
            )
        }
        EntityFieldAspectPatchDenial::ContractValidationDenied {
            field_locator,
            denial,
        } => {
            format!(
                "entity field aspect update for '{}' failed contract validation for aspect {:?}: {:?}",
                aspect_field_locator_path_label(field_locator),
                field_locator.aspect().aspect_key(),
                denial
            )
        }
        EntityFieldAspectPatchDenial::PatchConstructionDenied {
            field_locator,
            denial,
        } => entity_field_patch_construction_denial_detail(field_locator.as_ref(), denial),
        EntityFieldAspectPatchDenial::FieldPatchApplicationDenied {
            field_locator,
            denial,
        } => entity_field_patch_application_denial_detail(field_locator, denial),
        EntityFieldAspectPatchDenial::WholeAspectPatchApplicationDenied { aspect_key, denial } => {
            format!(
                "entity aspect update for {:?} could not apply foundational whole-aspect patch: {:?}",
                aspect_key, denial
            )
        }
        EntityFieldAspectPatchDenial::MissingAuthoritativeAspectState { aspect_key } => {
            let aspect = aspect_key
                .as_ref()
                .map(|aspect_key| format!("{aspect_key:?}"))
                .unwrap_or_else(|| "unknown aspect".to_string());
            format!(
                "entity field aspect update for {aspect} requires stored authoritative aspect state"
            )
        }
        EntityFieldAspectPatchDenial::EmptyAuthoritativePatchPlan => {
            "entity field aspect update produced an empty authoritative patch plan".to_string()
        }
    }
}

fn entity_field_patch_construction_denial_detail(
    field_locator: Option<&forge_foundational::facade::AspectFieldLocator>,
    denial: &forge_foundational::facade::AuthoritativePatchConstructionDenial,
) -> String {
    match field_locator {
        Some(field_locator) => format!(
            "entity field aspect update for {:?} at '{}' could not construct foundational patch: {:?}",
            field_locator.aspect().aspect_key(),
            aspect_field_locator_path_label(field_locator),
            denial
        ),
        None => format!(
            "entity field aspect update could not construct foundational patch: {:?}",
            denial
        ),
    }
}

fn entity_field_patch_application_denial_detail(
    field_locator: &forge_foundational::facade::AspectFieldLocator,
    denial: &forge_foundational::facade::AuthoritativePatchApplicationDenial,
) -> String {
    format!(
        "entity field aspect update for {:?} at '{}' could not apply foundational field patch: {:?}",
        field_locator.aspect().aspect_key(),
        aspect_field_locator_path_label(field_locator),
        denial
    )
}

fn aspect_field_locator_path_label(
    field_locator: &forge_foundational::facade::AspectFieldLocator,
) -> String {
    field_locator
        .field_path()
        .fields()
        .iter()
        .map(forge_foundational::facade::FieldKey::as_str)
        .collect::<Vec<_>>()
        .join(".")
}
