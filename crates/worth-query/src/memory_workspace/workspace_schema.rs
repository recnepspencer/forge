use crate::aspect_field_authoring::entity_string_field_aspect;
use worth_relational::facade::schema::{AspectBinding, DeclaredAspectContractBinding};

use super::{WorthQueryAspect, WorthQueryWorkspaceError};

pub(super) fn inferred_string_declarations(
    aspects: &[WorthQueryAspect],
) -> Result<Vec<DeclaredAspectContractBinding>, WorthQueryWorkspaceError> {
    aspects
        .iter()
        .map(|aspect| {
            let touch = aspect.aspect_touch();
            let target = touch.parsed_target();
            let field_label = target
                .field_path()
                .and_then(single_field_label)
                .cloned()
                .unwrap_or(final_field_key(aspect.native_field_path())?.clone());
            entity_string_field_aspect(target.aspect_key().as_str(), field_label.as_str())
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(WorthQueryWorkspaceError::new)
}

pub(super) fn native_contract_declarations(
    aspects: &[WorthQueryAspect],
    contracts: impl IntoIterator<Item = worth_foundational::facade::AspectContract>,
) -> Result<Vec<DeclaredAspectContractBinding>, WorthQueryWorkspaceError> {
    contracts
        .into_iter()
        .filter_map(|contract| {
            let mapping = aspects
                .iter()
                .find(|aspect| aspect.aspect_touch().native_aspect_key() == contract.key());
            let Some(mapping) = mapping else {
                return None;
            };
            let declaration = mapping
                .native_field_path()
                .fields()
                .first()
                .cloned()
                .ok_or_else(|| WorthQueryWorkspaceError::new("physical mapping is empty"))
                .map(|field| DeclaredAspectContractBinding {
                    binding: AspectBinding::EntityField { field },
                    contract,
                });
            Some(declaration)
        })
        .collect()
}

fn single_field_label(
    path: &worth_foundational::facade::CanonicalFieldPath,
) -> Option<&worth_foundational::facade::FieldKey> {
    match path.fields() {
        [field] => Some(field),
        _ => None,
    }
}

fn final_field_key(
    path: &worth_foundational::facade::CanonicalFieldPath,
) -> Result<&worth_foundational::facade::FieldKey, String> {
    path.fields()
        .last()
        .ok_or_else(|| "native field path must contain a field segment".to_string())
}
