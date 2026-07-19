use worth_foundational::facade::{aspects, AspectIdentity, ScalarAspectType};
use worth_relational::facade::schema::{AspectBinding, DeclaredAspectContractBinding};

use super::keys::{aspect_key, field_key};

pub(crate) fn entity_string_field_aspect(
    aspect_label: &str,
    field_label: &str,
) -> Result<DeclaredAspectContractBinding, String> {
    Ok(DeclaredAspectContractBinding {
        binding: AspectBinding::EntityField {
            field: field_key(field_label)?,
        },
        contract: scalar_string_contract(aspect_label)?,
    })
}

#[cfg(test)]
pub(crate) fn relation_string_field_aspect(
    aspect_label: &str,
    field_label: &str,
) -> Result<DeclaredAspectContractBinding, String> {
    Ok(DeclaredAspectContractBinding {
        binding: AspectBinding::RelationField {
            field: field_key(field_label)?,
        },
        contract: scalar_string_contract(aspect_label)?,
    })
}

pub(crate) fn lifecycle_string_aspect(
    aspect_label: &str,
) -> Result<DeclaredAspectContractBinding, String> {
    Ok(DeclaredAspectContractBinding {
        binding: AspectBinding::LifecycleTransition,
        contract: scalar_string_contract(aspect_label)?,
    })
}

#[cfg(test)]
pub(crate) fn relation_source_endpoint_aspect(
    aspect_label: &str,
) -> Result<DeclaredAspectContractBinding, String> {
    Ok(DeclaredAspectContractBinding {
        binding: AspectBinding::RelationSourceEndpoint,
        contract: entity_reference_contract(aspect_label)?,
    })
}

#[cfg(test)]
pub(crate) fn relation_target_endpoint_aspect(
    aspect_label: &str,
) -> Result<DeclaredAspectContractBinding, String> {
    Ok(DeclaredAspectContractBinding {
        binding: AspectBinding::RelationTargetEndpoint,
        contract: entity_reference_contract(aspect_label)?,
    })
}

pub(super) fn scalar_string_contract(
    aspect_label: &str,
) -> Result<worth_foundational::AspectContract, String> {
    Ok(aspects()
        .contract()
        .for_key(aspect_key(aspect_label)?)
        .identified_by(AspectIdentity(stable_contract_identity(aspect_label)))
        .at_revision(aspects().vocabulary().revision(1))
        .scalar(ScalarAspectType::String))
}

#[cfg(test)]
fn entity_reference_contract(
    aspect_label: &str,
) -> Result<worth_foundational::AspectContract, String> {
    Ok(aspects()
        .contract()
        .for_key(aspect_key(aspect_label)?)
        .identified_by(AspectIdentity(stable_contract_identity(aspect_label)))
        .at_revision(aspects().vocabulary().revision(1))
        .reference_entity())
}

pub(super) fn stable_contract_identity(label: &str) -> u64 {
    let mut hash = 14695981039346656037_u64;
    for byte in label.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(1099511628211_u64);
    }
    hash
}
