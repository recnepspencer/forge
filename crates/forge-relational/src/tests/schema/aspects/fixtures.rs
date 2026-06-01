use forge_foundational::{AspectKey, FieldKey};

use crate::facade::schema::{AspectBinding, DeclaredAspectContractBinding};

pub(super) fn lifecycle_aspect_named(aspect_key: AspectKey) -> DeclaredAspectContractBinding {
    DeclaredAspectContractBinding {
        binding: AspectBinding::LifecycleTransition,
        contract: test_scalar_contract(aspect_key),
    }
}

pub(super) fn relation_field_with_entity_reference_contract(
    aspect_key: AspectKey,
    field: FieldKey,
) -> DeclaredAspectContractBinding {
    DeclaredAspectContractBinding {
        binding: AspectBinding::RelationField { field },
        contract: forge_foundational::aspects()
            .contract()
            .for_key(aspect_key)
            .identified_by(forge_foundational::AspectIdentity(42))
            .at_revision(forge_foundational::aspects().vocabulary().revision(1))
            .reference_entity(),
    }
}

fn test_scalar_contract(aspect_key: AspectKey) -> forge_foundational::AspectContract {
    forge_foundational::aspects()
        .contract()
        .for_key(aspect_key)
        .identified_by(forge_foundational::AspectIdentity(41))
        .at_revision(forge_foundational::aspects().vocabulary().revision(1))
        .scalar(forge_foundational::ScalarAspectType::String)
}
