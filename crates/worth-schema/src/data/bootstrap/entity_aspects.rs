use forge_relational::facade::schema::{
    AspectBinding, DeclaredAspectContractBinding, KindAspectContractDeclarations,
};

use crate::data::aspects::{
    aspect_key, entity_domain_aspect, entity_domain_field, field_key, scalar_string_contract,
    Aspect,
};
use crate::data::entities::EntityKind;

pub fn entity_aspects(kind: EntityKind) -> KindAspectContractDeclarations {
    KindAspectContractDeclarations::new(vec![
        entity_domain_field_aspect(entity_domain_aspect(kind), entity_domain_field(kind)),
        lifecycle_aspect(),
    ])
}

fn entity_domain_field_aspect(aspect: Aspect, field: &str) -> DeclaredAspectContractBinding {
    DeclaredAspectContractBinding {
        binding: AspectBinding::EntityField {
            field: field_key(field),
        },
        contract: scalar_string_contract(aspect.aspect_key().as_str()),
    }
}

fn lifecycle_aspect() -> DeclaredAspectContractBinding {
    DeclaredAspectContractBinding {
        binding: AspectBinding::LifecycleTransition,
        contract: scalar_string_contract(aspect_key("lifecycle").as_str()),
    }
}
